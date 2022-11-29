use crate::*;

mod func;
mod iter;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
/// Garbage-collected hash map implementing `Copy`.
pub struct Map<K: Obj, V: Obj>(pub GcCow<IMHashMap<K, V>>);

impl<K: Obj, V: Obj> GcCompat for Map<K, V> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<K: Obj, V: Obj> GcCompat for IMHashMap<K, V> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for (k, v) in self.iter() {
            k.points_to(m);
            v.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

// This is not #[derive]d, as this would wrongly require K: Default.
impl<K: Obj, V: Obj> Default for Map<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}
use crate::artifacts::vcs::VcsDemoProjection;
pub fn apply(projection: &mut VcsDemoProjection, counter: i64) { projection.counter = counter; }

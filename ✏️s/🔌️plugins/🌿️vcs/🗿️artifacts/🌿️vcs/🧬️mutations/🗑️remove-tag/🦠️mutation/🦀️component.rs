use crate::artifacts::vcs::VcsDemoProjection;
pub fn apply(projection: &mut VcsDemoProjection, tag: &str) { projection.tags.retain(|e| e != tag); }

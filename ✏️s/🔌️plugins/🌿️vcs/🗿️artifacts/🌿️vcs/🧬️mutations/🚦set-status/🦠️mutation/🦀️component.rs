use crate::artifacts::vcs::VcsDemoProjection;
pub fn apply(projection: &mut VcsDemoProjection, status: &str) { projection.status = status.to_string(); }

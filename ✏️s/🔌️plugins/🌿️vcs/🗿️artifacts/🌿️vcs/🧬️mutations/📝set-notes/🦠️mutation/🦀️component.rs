use crate::artifacts::vcs::VcsDemoProjection;
pub fn apply(projection: &mut VcsDemoProjection, notes: &str) { projection.notes = notes.to_string(); }

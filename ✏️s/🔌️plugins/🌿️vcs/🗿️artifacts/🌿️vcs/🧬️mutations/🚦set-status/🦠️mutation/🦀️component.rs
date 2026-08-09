use crate::artifacts::vcs::VcsSnapshot;
pub fn apply(projection: &mut VcsSnapshot, status: &str) { projection.status = status.to_string(); }

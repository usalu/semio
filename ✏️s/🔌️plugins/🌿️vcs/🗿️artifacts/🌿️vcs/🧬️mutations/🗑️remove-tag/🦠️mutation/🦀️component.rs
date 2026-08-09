use crate::artifacts::vcs::VcsSnapshot;
pub fn apply(projection: &mut VcsSnapshot, tag: &str) { projection.tags.retain(|e| e != tag); }

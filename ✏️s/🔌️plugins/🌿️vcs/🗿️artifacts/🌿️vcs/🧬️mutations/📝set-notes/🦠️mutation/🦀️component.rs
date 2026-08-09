use crate::artifacts::vcs::VcsSnapshot;
pub fn apply(projection: &mut VcsSnapshot, notes: &str) { projection.notes = notes.to_string(); }

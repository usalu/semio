use crate::artifacts::vcs::VcsSnapshot;
pub fn apply(projection: &mut VcsSnapshot, counter: i64) { projection.counter = counter; }

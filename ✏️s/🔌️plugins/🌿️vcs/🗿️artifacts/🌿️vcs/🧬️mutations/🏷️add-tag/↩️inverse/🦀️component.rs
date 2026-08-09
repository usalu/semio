use crate::artifacts::vcs::{VcsSnapshot, mutations::VcsDemoMutation};
pub fn inverse(_base: &VcsSnapshot, tag: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::RemoveTag { tag: tag.to_string() }]
}

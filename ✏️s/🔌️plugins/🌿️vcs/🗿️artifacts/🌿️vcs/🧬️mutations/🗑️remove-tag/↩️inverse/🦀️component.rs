use crate::artifacts::vcs::{VcsSnapshot, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsSnapshot, tag: &str) -> Vec<VcsDemoMutation> {
    if base.tags.iter().any(|t| t == tag) {
        vec![VcsDemoMutation::AddTag { tag: tag.to_string() }]
    } else {
        vec![VcsDemoMutation::AddTag { tag: tag.to_string() }]
    }
}

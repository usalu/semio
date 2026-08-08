use crate::artifacts::vcs::{VcsDemoProjection, mutations::VcsDemoMutation};
pub fn inverse(_base: &VcsDemoProjection, tag: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::RemoveTag { tag: tag.to_string() }]
}

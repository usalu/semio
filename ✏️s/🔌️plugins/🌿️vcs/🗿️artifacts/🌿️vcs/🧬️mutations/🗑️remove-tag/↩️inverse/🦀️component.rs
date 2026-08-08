use crate::artifacts::vcs::{VcsDemoProjection, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsDemoProjection, tag: &str) -> Vec<VcsDemoMutation> {
    if base.tags.iter().any(|t| t == tag) {
        vec![VcsDemoMutation::AddTag { tag: tag.to_string() }]
    } else {
        vec![VcsDemoMutation::AddTag { tag: tag.to_string() }]
    }
}

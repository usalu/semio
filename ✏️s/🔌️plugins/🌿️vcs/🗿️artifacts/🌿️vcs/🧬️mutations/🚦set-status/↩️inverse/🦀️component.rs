use crate::artifacts::vcs::{VcsDemoProjection, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsDemoProjection, _v: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetStatus { status: base.status.clone() }]
}

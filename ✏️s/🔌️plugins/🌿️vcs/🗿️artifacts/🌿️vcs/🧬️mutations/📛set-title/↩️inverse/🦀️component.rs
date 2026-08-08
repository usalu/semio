use crate::artifacts::vcs::{VcsDemoProjection, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsDemoProjection, _title: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetTitle { title: base.title.clone() }]
}

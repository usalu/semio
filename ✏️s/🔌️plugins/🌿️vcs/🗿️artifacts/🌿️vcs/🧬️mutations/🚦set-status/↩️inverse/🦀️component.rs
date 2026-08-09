use crate::artifacts::vcs::{VcsSnapshot, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsSnapshot, _v: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetStatus { status: base.status.clone() }]
}

use crate::artifacts::vcs::{VcsSnapshot, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsSnapshot, _title: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetTitle { title: base.title.clone() }]
}

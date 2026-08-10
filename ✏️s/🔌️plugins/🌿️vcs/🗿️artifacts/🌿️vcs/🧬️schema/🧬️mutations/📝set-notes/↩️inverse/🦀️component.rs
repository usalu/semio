use crate::artifacts::vcs::{VcsSnapshot, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsSnapshot, _v: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetNotes { notes: base.notes.clone() }]
}

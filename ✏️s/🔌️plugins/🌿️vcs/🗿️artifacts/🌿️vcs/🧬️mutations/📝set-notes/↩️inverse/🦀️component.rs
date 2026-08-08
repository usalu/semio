use crate::artifacts::vcs::{VcsDemoProjection, mutations::VcsDemoMutation};
pub fn inverse(base: &VcsDemoProjection, _v: &str) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetNotes { notes: base.notes.clone() }]
}

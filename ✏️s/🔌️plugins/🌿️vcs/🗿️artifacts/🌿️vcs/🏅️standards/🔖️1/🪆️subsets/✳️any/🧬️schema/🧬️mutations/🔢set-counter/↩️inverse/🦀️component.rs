use crate::artifacts::vcs::VcsSnapshot;
use crate::artifacts::vcs::schema::mutations::VcsDemoMutation;
pub fn inverse(base: &VcsSnapshot, _counter: i64) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetCounter { counter: base.counter }]
}

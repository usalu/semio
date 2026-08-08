use crate::artifacts::vcs::VcsDemoProjection;
use crate::artifacts::vcs::mutations::VcsDemoMutation;
pub fn inverse(base: &VcsDemoProjection, _counter: i64) -> Vec<VcsDemoMutation> {
    vec![VcsDemoMutation::SetCounter { counter: base.counter }]
}

//! ⚖️ Reference-twin mutation laws: the cold forms of `vcs::apply_mutation` and the framework inverse-sum law, for mutations whose reference rows (`Fem2dSnapshotValue`) are plain data and need no fail-closed retirement.
extern crate semio_framework_os_kernel as protocol;

use protocol::{ColdDiffAlgebra, ColdMutation, ColdMutationDiff, MutationApplyError, MutationMessage};
use semio_framework_value::ToValue;

/// ▶️ Computes `operation.diff(snapshot)` and applies it, returning the new reference snapshot with every message the outcome carried.
pub fn apply_cold_mutation<P, Op>(snapshot: &P, operation: &Op) -> Result<(P, Vec<MutationMessage>), MutationApplyError>
where
    P: Clone,
    Op: ColdMutation<P>,
{
    let (diff, messages) = operation.diff(snapshot).into_parts();
    Ok((protocol::apply_diff_cold(&diff, snapshot)?, messages))
}

/// ✅️ The concrete inverse of `mutation` replays to `base` and its diffs sum, by `absorb`, to the negative of the forward diff.
pub async fn assert_cold_mutation_inverse_sum_law<P, Op>(mutation: &Op, base: &P)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: ColdMutation<P>,
    Op::ReferenceDiff: ColdDiffAlgebra<P> + PartialEq,
{
    let (forward, messages) = mutation.diff(base).into_parts();
    let rejected = messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    assert!(!rejected, "a mutation expected to invert cleanly must not have been rejected: {messages:?}");
    let after = protocol::apply_diff_cold(&forward, base).expect("valid forward diff must apply");
    let changed = after != *base;
    let negative = forward.inverse(base);
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    let non_empty = !backward.is_empty();
    backward.reverse();
    let mut state = after.clone();
    let mut sum = <Op::ReferenceDiff as Default>::default();
    for undo in &backward {
        let (step, _) = undo.diff(&state).into_parts();
        let next = protocol::apply_diff_cold(&step, &state).expect("valid inverse diff must apply");
        ColdMutationDiff::absorb(&mut sum, step);
        state = next;
    }
    let summed = protocol::apply_diff_cold(&sum, &after);
    let mut canonical_sum = <Op::ReferenceDiff as Default>::default();
    canonical_sum.absorb(sum);
    let mut canonical_negative = <Op::ReferenceDiff as Default>::default();
    canonical_negative.absorb(negative);
    let report = format!("sum={:?} negative={:?} restored={state:?} base={base:?}", ToValue::to_value(&canonical_sum), ToValue::to_value(&canonical_negative));
    assert!(!changed || non_empty, "mutation.inverse(base) must not be empty for a mutation that changes state; base={base:?}");
    assert!(state == *base, "applying mutation.inverse(base) (reversed) after mutation must restore base; {report}");
    assert!(summed.as_ref() == Ok(base), "the summed inverse diffs must restore base from the applied state; {report}");
    assert!(!changed || canonical_sum == canonical_negative, "the summed inverse diffs must equal the negative of the forward diff; {report}");
}

/// ✅️ Applying `mutation` then the reversed `mutation.inverse(base)` restores `base`.
pub async fn assert_cold_mutation_inverse_law<P, Op>(base: &P, mutation: &Op)
where
    P: Clone + PartialEq + std::fmt::Debug,
    Op: ColdMutation<P>,
{
    let (forward, messages) = mutation.diff(base).into_parts();
    assert!(!messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)), "a mutation expected to invert cleanly must not have been rejected: {messages:?}");
    let mut state = protocol::apply_diff_cold(&forward, base).expect("valid forward diff must apply");
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    backward.reverse();
    for undo in &backward {
        let (delta, _) = undo.diff(&state).into_parts();
        state = protocol::apply_diff_cold(&delta, &state).expect("valid inverse diff must apply");
    }
    assert!(state == *base, "applying mutation.inverse(base) (reversed) after mutation must restore base; restored={state:?} base={base:?}");
}

/// ✅️ A mutation targeting an absent element reports `mutation.target-missing` and leaves the diff empty.
pub async fn assert_cold_missing_target_is_error<P, Op>(base: &P, mutation: &Op)
where
    Op: ColdMutation<P>,
    Op::ReferenceDiff: PartialEq + std::fmt::Debug + Default,
{
    let outcome = mutation.diff(base);
    let missing = outcome.messages().iter().any(|message| message.level == semio_framework_diagnostic::Severity::Error && message.code.0 == "mutation.target-missing");
    assert!(missing, "a mutation targeting an absent element must report mutation.target-missing; messages={:?}", outcome.messages());
    assert_eq!(outcome.diff(), &Op::ReferenceDiff::default(), "a mutation that reports an error must carry an empty diff");
}

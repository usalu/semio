
use super::*;
use crate::{SequenceStep, StepParams, default_snapshot};
use protocol::Mutation;

#[semio_framework_async_macros::async_test]
async fn create_step_diff_applies_onto_the_base_snapshot() {
    let base = default_snapshot();
    let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
    let operation = crate::mutations::create_step(step);
    let diff: SequenceDiff = operation.diff(&base).into_parts().0;
    assert!(diff.content.is_some(), "CreateStep must produce a content diff: {diff:?}");
    assert_eq!(diff.apply(&base).expect("valid mutation diff").to_fixture().steps.len(), base.to_fixture().steps.len() + 1);
}

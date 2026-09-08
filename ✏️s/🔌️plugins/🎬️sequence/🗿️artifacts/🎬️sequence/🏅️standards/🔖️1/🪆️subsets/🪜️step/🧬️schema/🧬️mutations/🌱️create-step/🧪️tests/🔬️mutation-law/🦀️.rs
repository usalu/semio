
use super::*;
use crate::{StepParams, default_snapshot};
use protocol::{
    Mutation,
    os_spr::testkit::{assert_fatal_never_applies, assert_mutation_inverse_law},
};

#[semio_framework_async_macros::async_test]
async fn create_step_inverse_law() {
    let base = default_snapshot();
    let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
    assert_mutation_inverse_law(&base, &create_step(step)).await;
}

#[semio_framework_async_macros::async_test]
async fn create_family_fatal_never_applies() {
    let base = default_snapshot();
    let outcome = create_step(SequenceStep { id: "step-1".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false }).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    assert_fatal_never_applies(&outcome).await;
}

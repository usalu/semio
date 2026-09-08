
use super::*;
use crate::default_snapshot;
use protocol::{
    Mutation,
    os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_inverse_law},
};

#[semio_framework_async_macros::async_test]
async fn duplicate_step_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &duplicate_step("step-1".into(), "step-1-copy".into(), 10.0, 10.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn duplicate_family_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &duplicate_step("missing".into(), "step-1-copy".into(), 0.0, 0.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn duplicate_family_fatal_never_applies() {
    let base = default_snapshot();
    let outcome = duplicate_step("step-1".into(), "step-2".into(), 0.0, 0.0).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    assert_fatal_never_applies(&outcome).await;
}

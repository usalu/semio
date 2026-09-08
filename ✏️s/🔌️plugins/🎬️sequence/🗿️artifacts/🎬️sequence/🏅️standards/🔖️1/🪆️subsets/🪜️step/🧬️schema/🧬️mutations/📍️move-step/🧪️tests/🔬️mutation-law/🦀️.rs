
use super::*;
use crate::default_snapshot;
use protocol::{
    Mutation,
    os_spr::testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_diff_absorb_law, assert_mutation_inverse_law},
};

#[semio_framework_async_macros::async_test]
async fn move_step_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &move_step("step-1".into(), 42.0, -8.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_step_diff_absorb_law() {
    use protocol::Mutation;
    let base = default_snapshot();
    let d1 = move_step("step-1".into(), 10.0, 10.0).diff(&base).into_parts().0;
    let mid = protocol::MutationDiff::apply(&d1, &base).expect("valid mutation diff");
    let d2 = move_step("step-1".into(), 20.0, 30.0).diff(&mid).into_parts().0;
    assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn move_family_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &move_step("missing".into(), 1.0, 1.0)).await;
}

#[semio_framework_async_macros::async_test]
async fn move_family_fatal_never_applies() {
    let base = default_snapshot();
    let outcome = move_step("step-1".into(), f64::NAN, 0.0).diff(&base);
    assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
    assert_fatal_never_applies(&outcome).await;
}

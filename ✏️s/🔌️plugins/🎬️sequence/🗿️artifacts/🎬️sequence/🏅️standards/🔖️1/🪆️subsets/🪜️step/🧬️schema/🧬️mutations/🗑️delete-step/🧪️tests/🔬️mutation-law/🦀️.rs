
use super::*;
use crate::default_snapshot;
use protocol::os_spr::testkit::{assert_missing_target_is_error, assert_mutation_inverse_law};

#[semio_framework_async_macros::async_test]
async fn delete_step_inverse_law() {
    let base = default_snapshot();
    assert_mutation_inverse_law(&base, &delete_step("step-1".into())).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_family_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &delete_step("missing".into())).await;
}

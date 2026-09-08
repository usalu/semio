
use super::*;
use crate::default_snapshot;
use protocol::os_spr::testkit::assert_missing_target_is_error;

#[semio_framework_async_macros::async_test]
async fn edit_family_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &edit_step_params("missing".into(), StepParams::new())).await;
}

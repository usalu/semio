
use super::*;
use crate::default_snapshot;
use protocol::os_spr::testkit::assert_missing_target_is_error;

#[semio_framework_async_macros::async_test]
async fn change_family_missing_target_is_error() {
    let base = default_snapshot();
    assert_missing_target_is_error(&base, &change_step_collapsed("missing".into(), true)).await;
}

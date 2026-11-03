use crate::modes::view::windows::checks::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_CHECKS);
    assert_eq!(definition().id, WINDOW_CHECKS);
}

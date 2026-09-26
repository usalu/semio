use super::*;
use crate::editor::en1994::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_INPUTS);
    assert_eq!(definition().id, WINDOW_INPUTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_structured_inputs() {
    let mut app = context::app_with_registry().await;
    let body = context::render(&mut app, BODY_INPUTS).await;
    assert!(!body.is_empty(), "structured inputs must render a non-empty body");
    context::close(&mut app);
}

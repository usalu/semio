use crate::modes::edit::windows::checks::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_CHECKS);
    assert_eq!(definition().id, WINDOW_CHECKS);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_computed_checks_without_an_evaluate_command() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_CHECKS).await;
    assert!(!rendered.contains("No checks computed."), "the default document must show its checks right after open: {rendered}");
    context::close(&mut app);
}

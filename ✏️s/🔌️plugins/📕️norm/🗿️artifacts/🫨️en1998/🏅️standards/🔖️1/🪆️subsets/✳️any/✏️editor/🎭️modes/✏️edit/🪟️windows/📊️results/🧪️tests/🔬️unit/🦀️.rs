use super::*;
use crate::editor::en1998::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_RESULTS);
    assert_eq!(definition().id, WINDOW_RESULTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_computed_checks() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_RESULTS).await;
    assert!(!rendered.contains("No checks computed."), "the default document must compute at least one check: {rendered}");
}

use crate::modes::edit::windows::properties::*;
use crate::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_PROPERTIES);
    assert_eq!(definition().id, WINDOW_PROPERTIES);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_typed_inspector() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, BODY_PROPERTIES).await.contains(':'), "the properties body renders the typed inspector");
    context::close(&mut app);
}

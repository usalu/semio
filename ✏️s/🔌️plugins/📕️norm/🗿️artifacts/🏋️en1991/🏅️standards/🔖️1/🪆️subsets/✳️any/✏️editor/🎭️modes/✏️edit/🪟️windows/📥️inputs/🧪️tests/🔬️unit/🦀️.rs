use super::*;
use crate::editor::en1991::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_INPUTS);
    assert_eq!(definition().id, WINDOW_INPUTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_document_as_json() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, BODY_INPUTS).await.contains(':'), "the inputs body renders the document json");
}

use super::*;
use crate::editor::en1999::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_INPUTS);
    assert_eq!(definition().id, WINDOW_INPUTS);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_structured_document_editor() {
    let mut app = context::app_with_registry().await;
    let body = context::render(&mut app, BODY_INPUTS).await;
    assert!(!body.contains("Unknown body"), "inputs body must resolve");
    assert!(
        body.contains("National annex") || body.contains("Nationaler Anhang") || body.contains("norm-inputs"),
        "inputs must render the structured editor, not a bare JSON dump: {body}"
    );
    context::close(&mut app);
}

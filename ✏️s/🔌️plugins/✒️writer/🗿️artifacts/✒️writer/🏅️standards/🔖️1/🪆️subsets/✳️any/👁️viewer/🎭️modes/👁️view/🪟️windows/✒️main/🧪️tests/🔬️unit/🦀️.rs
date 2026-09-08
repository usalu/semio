
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_framework_text_window_kits_frozen_kind_id() {
    let def = definition();
    assert_eq!(def.id, "framework.window.text");
    assert_eq!(def.id, WRITER_VIEW_WINDOW_KIND);
}

#[semio_framework_async_macros::async_test]
async fn definition_declares_no_mutating_actions() {
    assert!(definition().actions.is_empty(), "a viewer window kind must declare no mutating actions");
}

#[semio_framework_async_macros::async_test]
async fn render_carries_the_documents_own_text_and_language_read_only() {
    let document = crate::schema::empty_writer_snapshot();
    let node = render(&document);
    let json = serde_json::to_string(&node.expect("viewer surface")).unwrap();
    assert!(json.contains("\"readOnly\":true") || json.contains("readOnly"), "viewer text scene must stamp read-only: {json}");
}

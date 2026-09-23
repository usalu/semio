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
    let document = crate::writer_snapshot_with_text(crate::WRITER_DOCUMENT_SCHEMA, "viewer", "jack", "writer://viewer", "let x = 1;");
    let node = render(&document).expect("viewer surface");
    let scene = semio_framework_plugin::artifact_app_laws::built_surface_scene::<semio_framework_plugin::TextEditorScene>(&node).expect("text-editor scene");
    assert_eq!(scene.buffer, "let x = 1;");
    assert_eq!(scene.language.as_deref(), Some("jack"));
    assert_eq!(scene.settings_json.as_deref(), Some("{\"readOnly\":true}"));
}

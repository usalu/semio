use super::*;

#[test]
fn source_window_is_a_registered_text_editor_surface() {
    let definition = definition();
    assert_eq!(definition.id, PLAYBOOK_PLAY_WINDOW_SOURCE);
    assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_SOURCE);
    assert_eq!(definition.surface_kind, SurfaceKind::TextEditor);
    let projected = scene(&crate::PlaybookSnapshot::default());
    assert_eq!(projected.language.as_deref(), Some("json"));
    let _: crate::PlaybookSnapshot = protocol::json::from_json_str(&projected.buffer).expect("snapshot source");
}

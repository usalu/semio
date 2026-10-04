use super::*;

#[test]
fn source_window_is_a_registered_text_editor_surface() {
    let definition = definition();
    assert_eq!(definition.id, PLAYBOOK_PLAY_WINDOW_SOURCE);
    assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_SOURCE);
    assert_eq!(definition.surface_kind, SurfaceKind::TextEditor);
    let projected = scene(&crate::playbook::empty_playbook_snapshot());
    assert_eq!(projected.language.as_deref(), Some("json"));
    let source: crate::PlaybookSpec = semio_framework_pack_json::from_json_str(&projected.buffer, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("composed playbook source");
    assert_eq!(source, crate::playbook::empty_playbook_snapshot());
}

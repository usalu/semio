use super::*;

#[test]
fn definition_declares_the_diff_view_surface_and_body_key() {
    let definition = definition();
    assert_eq!(definition.id, PLAYBOOK_PLAY_WINDOW_CHANGES);
    assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_CHANGES);
    assert_eq!(definition.surface_kind, SurfaceKind::DiffView);
}

#[test]
fn authored_playbook_projects_to_a_real_diff_view_scene() {
    let mut spec = crate::PlaybookSnapshot::default();
    spec.title = Some("Emergency response".into());
    let projected = scene(&spec);
    assert_eq!(projected.language.as_deref(), Some("json"));
    assert_eq!(projected.mode.as_deref(), Some("unified"));
    assert_ne!(projected.before, projected.after);
    let roundtrip: crate::PlaybookSnapshot = protocol::json::from_json_str(&projected.after).expect("authored after payload");
    assert_eq!(roundtrip, spec);
}

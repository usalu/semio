use super::*;

#[test]
fn activity_window_is_a_registered_event_feed_surface() {
    let definition = definition();
    assert_eq!(definition.id, PLAYBOOK_PLAY_WINDOW_ACTIVITY);
    assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_ACTIVITY);
    assert_eq!(definition.surface_kind, SurfaceKind::EventFeed);
    let projected = scene(&crate::playbook::empty_playbook_snapshot());
    let entries: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&projected.entries_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("entries");
    assert!(matches!(entries, semio_framework_value::DslValue::Array(_)));
}

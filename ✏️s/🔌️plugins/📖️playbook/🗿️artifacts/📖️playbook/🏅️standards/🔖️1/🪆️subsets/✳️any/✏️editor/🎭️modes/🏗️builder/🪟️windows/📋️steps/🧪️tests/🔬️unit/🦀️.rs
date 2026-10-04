use super::*;

#[test]
fn steps_window_is_a_registered_table_surface() {
    let definition = definition();
    assert_eq!(definition.id, PLAYBOOK_PLAY_WINDOW_STEPS);
    assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_STEPS);
    assert_eq!(definition.surface_kind, SurfaceKind::Table);
    let projected = scene(&crate::playbook::empty_playbook_snapshot());
    let columns: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&projected.columns_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("columns");
    let rows: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&projected.rows_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rows");
    assert!(matches!(columns, semio_framework_value::DslValue::Array(columns) if columns.len() == 3));
    assert!(matches!(rows, semio_framework_value::DslValue::Array(_)));
}

use super::*;

#[test]
fn steps_window_is_a_registered_table_surface() {
    let definition = definition();
    assert_eq!(definition.id, PLAYBOOK_PLAY_WINDOW_STEPS);
    assert_eq!(definition.body_key, PLAYBOOK_PLAY_BODY_STEPS);
    assert_eq!(definition.surface_kind, SurfaceKind::Table);
    let projected = scene(&crate::PlaybookSnapshot::default());
    let columns: dsl::DslValue = protocol::json::from_json_str(&projected.columns_json).expect("columns");
    let rows: dsl::DslValue = protocol::json::from_json_str(&projected.rows_json).expect("rows");
    assert!(matches!(columns, dsl::DslValue::Array(columns) if columns.len() == 3));
    assert!(matches!(rows, dsl::DslValue::Array(_)));
}

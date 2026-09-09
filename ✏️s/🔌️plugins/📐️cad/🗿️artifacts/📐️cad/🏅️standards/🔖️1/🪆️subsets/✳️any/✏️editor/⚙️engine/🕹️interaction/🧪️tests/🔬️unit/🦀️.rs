use super::*;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::Brep;

fn point_arg(x: f64, y: f64, z: f64) -> DslValue {
    DslValue::object([("point".to_string(), vec3_json([x, y, z]))])
}

fn value_arg(value: f64) -> DslValue {
    DslValue::object([("value".to_string(), DslValue::float(value))])
}

#[semio_framework_async_macros::async_test]
async fn catalog_includes_json_driven_and_legacy_building_entries() {
    assert!(interaction_by_id("primitive.box").is_some());
    assert!(interaction_by_id("solid.sphere").is_some());
    assert!(interaction_by_id("energy.energy.constructExternalWall").is_some());
    assert!(interaction_by_id("structure.structure.constructReinforcedConcreteColumn").is_some());
    assert!(interaction_by_id("building.building.constructWall").is_some());
    assert_eq!(list_interactions_for_model_definition("spatial.shape").len(), 37);
}

#[semio_framework_async_macros::async_test]
async fn box_interaction_commits_after_height() {
    let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "mode.diagonal", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([2.0, 3.0, 0.0]))));
    assert!(apply_event(&mut session, "set.height", Some(&DslValue::float(2.5))));
    assert!(apply_event(&mut session, "confirm", None));
    assert!(can_commit(&session));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    assert!(object.is_some());
    assert_eq!(object.unwrap().typology, "spatial.shape.primitive.box");
}

#[semio_framework_async_macros::async_test]
async fn box_interaction_default_mode_is_point_and_requires_length_prompt() {
    // 📦️box.json's default `boxMode` (set by the `start` transition) is "point", not "diagonal" —
    // a plain pointer.down after start does NOT reach diagonal_rubber.
    let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert_eq!(session.state, "first_corner_other_or_length");
}

#[semio_framework_async_macros::async_test]
async fn sphere_interaction_commits_via_command_finish() {
    let mut session = start_session("solid.sphere", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(2.0, 0.0, 0.0))));
    assert!(can_commit(&session));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    let object = object.expect("sphere commits");
    assert_eq!(object.typology, "spatial.shape.solid.sphere");
    assert_eq!(object.origin, [0.0, 0.0, 0.0]);
    assert_eq!(object.extent, Some([4.0, 4.0, 4.0]));
}

#[semio_framework_async_macros::async_test]
async fn external_wall_interaction_commits_via_generic_from_2_points_and_height() {
    let mut session = start_session("energy.energy.constructExternalWall", CadPaneId::Energy).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(4.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "set.height", Some(&value_arg(3.0))));
    assert!(can_commit(&session));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    let object = object.expect("wall commits");
    assert_eq!(object.typology, "energy.energy.externalwall");
}

#[semio_framework_async_macros::async_test]
async fn reinforced_concrete_column_interaction_commits_as_cylinder() {
    let mut session = start_session("structure.structure.constructReinforcedConcreteColumn", CadPaneId::StructureClassic).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(1.0, 1.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(1.5, 1.0, 0.0))));
    assert!(apply_event(&mut session, "set.height", Some(&value_arg(3.0))));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    let object = object.expect("column commits");
    assert_eq!(object.typology, "structure.structure.reinforcedconcretecolumn");
    assert_eq!(object.origin, [1.0, 1.0, 0.0]);
}

#[semio_framework_async_macros::async_test]
async fn slab_interaction_commits() {
    let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(4.0, 5.0, 0.0))));
    assert!(apply_event(&mut session, "set.height", Some(&value_arg(0.3))));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    assert!(object.is_some());
}

#[semio_framework_async_macros::async_test]
async fn slab_preview_shows_footprint_point() {
    let mut session = start_session("structure.structure.constructOneWayReinforcedConcreteSlab", CadPaneId::StructureClassic).expect("session");
    assert!(apply_event(&mut session, "mode.2points", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&point_arg(0.0, 0.0, 0.0))));
    let items = preview_display_items(&session);
    assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")));
}

#[semio_framework_async_macros::async_test]
async fn legacy_column_preview_shows_base_point() {
    let mut session = start_session("building.building.constructColumn", CadPaneId::Building).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([1.0, 2.0, 0.0]))));
    let items = preview_display_items(&session);
    assert!(items.iter().any(|item| item.get("kind").and_then(|value| value.as_str()) == Some("point")));
}

#[semio_framework_async_macros::async_test]
async fn legacy_wall_interaction_still_commits() {
    let mut session = start_session("building.building.constructWall", CadPaneId::Building).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([4.0, 0.0, 0.0]))));
    assert!(apply_event(&mut session, "set.height", Some(&DslValue::float(3.0))));
    assert!(can_commit(&session));
    let mut kernel = Brep::new();
    let object = commit_object(&mut kernel, &session, 0, |prefix| format!("{prefix}-1"));
    assert!(object.is_some());
    assert_eq!(object.unwrap().typology, "building.building.wall");
}

#[semio_framework_async_macros::async_test]
async fn parse_repl_line_accepts_legacy_raw_forms() {
    assert_eq!(parse_repl_line("set.height 2.5", None), Some(("set.height".into(), Some(DslValue::float(2.5)))));
    assert_eq!(parse_repl_line("dist 12", None), Some(("set.distance".into(), Some(DslValue::float(12.0)))));
}

#[semio_framework_async_macros::async_test]
async fn parse_repl_line_accepts_shell_normalized_forms() {
    // The React shell PascalCases every draft (framework/renderer/react `normalizeEngagementCommandText`),
    // so `set.height 3.5` arrives as `SetHeight3.5` with no separators.
    assert_eq!(parse_repl_line("SetHeight3.5", None), Some(("set.height".into(), Some(DslValue::float(3.5)))));
    assert_eq!(parse_repl_line("setheight0.25", None), Some(("set.height".into(), Some(DslValue::float(0.25)))));
    assert_eq!(parse_repl_line("Dist12.75", None), Some(("set.distance".into(), Some(DslValue::float(12.75)))));
}

#[semio_framework_async_macros::async_test]
async fn parse_repl_line_commits_bare_number_only_in_numeric_entry_state() {
    // Bare numeric entry (premigration `tryCommitNumericEntry`) only applies while a
    // numeric-entry state (e.g. box's first_corner_height) is active.
    assert_eq!(parse_repl_line("3.5", Some("first_corner_height")), Some(("set.height".into(), Some(DslValue::float(3.5)))));
    assert_eq!(parse_repl_line("2", Some("column_height")), Some(("set.height".into(), Some(DslValue::float(2.0)))));
    // Outside a numeric-entry state, a bare number is treated as an (unresolvable) interaction key.
    assert_eq!(parse_repl_line("3.5", None), Some(("3.5".into(), None)));
    assert_eq!(parse_repl_line("3.5", Some("idle")), Some(("3.5".into(), None)));
}

#[semio_framework_async_macros::async_test]
async fn box_interaction_commits_via_shell_normalized_repl_line() {
    let mut session = start_session("primitive.box", CadPaneId::Shape).expect("session");
    assert!(apply_event(&mut session, "start", None));
    assert!(apply_event(&mut session, "mode.diagonal", None));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([0.0, 0.0, 0.0]))));
    assert!(apply_event(&mut session, "pointer.down", Some(&vec3_json([2.0, 3.0, 0.0]))));
    let (event_kind, payload) = parse_repl_line("SetHeight2.5", Some(&session.state)).expect("parsed line");
    assert!(apply_event(&mut session, &event_kind, payload.as_ref()));
    assert!(apply_event(&mut session, "confirm", None));
    assert!(can_commit(&session));
}

use super::*;
use protocol::Inference;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const TABLE: &str = include_str!("../../../../../../../🧫️fixtures/💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json");

fn model() -> ModelSnapshot {
    from_json_str(TABLE, JsonMemberPolicy::Reject).expect("the table decodes")
}

fn json(rows: &[DslValue]) -> String {
    semio_framework_pack_json::to_json_string(&rows.to_vec())
}

fn rows_of(id: &str, labels: &BimLabels) -> Vec<DslValue> {
    let snapshot = model();
    let inference = ModelInference::infer(&snapshot).expect("infers");
    rows(&snapshot, &inference, id, labels)
}

#[test]
fn the_selection_picks_a_family_a_solid_s_family_or_the_first_family() {
    let snapshot = model();
    let ids = |list: &[&str]| list.iter().map(|id| id.to_string()).collect::<Vec<_>>();
    assert_eq!(selected(&snapshot, &[], &[]), Some("fam-broken".to_string()), "the first family of the model by id");
    assert_eq!(selected(&snapshot, &[], &ids(&["fam-hea"])), Some("fam-hea".to_string()));
    assert_eq!(selected(&snapshot, &ids(&["s-top"]), &[]), Some("fam-table".to_string()), "a solid selects its family");
    assert_eq!(selected(&snapshot, &ids(&["w-ghost"]), &ids(&["fam-rhs"])), Some("fam-rhs".to_string()));
    assert_eq!(selected(&ModelSnapshot::default(), &[], &[]), None);
}

#[test]
fn the_table_lists_the_family_its_parameters_with_values_and_its_solids_with_one_input_per_slot() {
    let en = &BimLabels::NATIVE_EN;
    let rows = rows_of("fam-table", en);
    let text = json(&rows);
    let snapshot = model();
    let parameters = snapshot.family_parameters.values().filter(|row| row.family == "fam-table").count();
    let slots: usize = snapshot.family_solids.values().filter(|row| row.family == "fam-table").map(|row| crate::standards::v1::subsets::any::schema::authored::formula::solid_slots(row).len()).sum();
    let solids = snapshot.family_solids.values().filter(|row| row.family == "fam-table").count();
    assert_eq!(rows.len(), 1 + (parameters + 1) + (solids + slots + 1), "family, parameters and the add row, solids with their slots and the add row");
    for expected in ["1600 mm", "Dining table", "\"m-oak\"", "height - top_thickness", "width > 1.4 m", "15°", "Furniture", "s-top#height", "Cuboid", "Revolution"] {
        assert!(text.contains(expected), "{expected} missing");
    }
    assert!(text.contains("\"yes\""), "the truth value reads yes");
    assert!(!text.contains("1.28 m"), "areas are numbers, not lengths");
}

#[test]
fn the_german_table_speaks_german() {
    let text = json(&rows_of("fam-table", &BimLabels::NATIVE_DE));
    assert!(text.contains("Möbel") && text.contains("\"ja\"") && text.contains("Quader"), "{text}");
    assert!(!text.contains("Furniture"));
}

#[test]
fn every_formula_input_names_the_command_the_part_the_operation_and_the_key() {
    let text = json(&rows_of("fam-table", &BimLabels::NATIVE_EN));
    for expected in ["editFamily", "\"part\":\"parameter\"", "\"op\":\"formula\"", "\"key\":\"width\"", "\"part\":\"solid\"", "\"op\":\"slot\"", "\"key\":\"s-top#height\"", "\"op\":\"name\"", "\"part\":\"family\"", "\"op\":\"rename\""] {
        assert!(text.contains(expected), "{expected} missing");
    }
}

#[test]
fn the_buttons_add_every_kind_and_shape_cycle_a_kind_remove_and_turn_a_revolution() {
    let text = json(&rows_of("fam-table", &BimLabels::NATIVE_EN));
    for kind in edit::KINDS {
        assert!(text.contains(&format!("\"op\":\"add\",\"key\":\"{kind:?}\"")) || text.contains(&format!("\"key\":\"{kind:?}\"")), "{kind:?}");
    }
    for token in edit::SHAPES {
        assert!(text.contains(&format!("\"key\":\"{token}\"")), "{token}");
    }
    assert!(text.contains("\"op\":\"remove\"") && text.contains("\"op\":\"kind\"") && text.contains("\"op\":\"axis\""));
    let broken = json(&rows_of("fam-hea", &BimLabels::NATIVE_EN));
    assert!(!broken.contains("\"op\":\"axis\""), "only a revolution turns");
}

#[test]
fn issues_show_in_the_language_of_the_labels_next_to_the_part_they_belong_to() {
    let en = json(&rows_of("fam-broken", &BimLabels::NATIVE_EN));
    assert!(en.contains("circular reference between loop_a, loop_b"), "{en}");
    assert!(en.contains("division by zero") && en.contains("unknown name ghost") && en.contains("depends on syntax which has no value"));
    let de = json(&rows_of("fam-broken", &BimLabels::NATIVE_DE));
    assert!(de.contains("Zirkelbezug zwischen loop_a, loop_b") && de.contains("Division durch null") && de.contains("unbekannter Name ghost"), "{de}");
}

#[test]
fn the_window_renders_a_family_the_hint_and_the_empty_model() {
    let snapshot = model();
    let inference = ModelInference::infer(&snapshot).expect("infers");
    assert!(render(&snapshot, &inference, &[], &["fam-table".to_string()], &BimLabels::NATIVE_EN).is_ok());
    let empty = ModelSnapshot::default();
    let nothing = ModelInference::infer(&empty).expect("infers");
    assert!(render(&empty, &nothing, &[], &[], &BimLabels::NATIVE_DE).is_ok());
    assert_eq!(columns(&BimLabels::NATIVE_EN).len(), 6);
}

#[test]
fn the_preview_draws_the_visible_solids_of_the_selected_family() {
    let snapshot = model();
    let inference = ModelInference::infer(&snapshot).expect("infers");
    let value = &inference.families["fam-table"];
    let drawn = drawn(value);
    let names: Vec<&str> = drawn.iter().map(|(id, _)| id.as_str()).collect();
    assert!(names.contains(&"s-top") && names.contains(&"s-rail") && !names.contains(&"s-shelf"), "the hidden shelf is not drawn: {names:?}");
    let top = &drawn.iter().find(|(id, _)| id == "s-top").expect("top").1;
    assert_eq!((top.indices.len(), top.groups.len(), top.face_groups.len()), (36, 1, 12));
    assert!((top.volume - 1.6 * 0.8 * 0.03).abs() < 1e-12);
    assert_eq!(top.groups[0].material, "m-oak");
    assert!(view_render(&snapshot, &inference, &[], &["fam-table".to_string()]).is_ok());
    assert!(view_render(&ModelSnapshot::default(), &ModelInference::default(), &[], &[]).is_ok());
}

#[test]
fn the_definitions_are_a_table_and_a_world_with_localized_names() {
    let (editor, view) = (definition(), view_definition());
    assert_eq!((editor.id.as_str(), editor.body_key.as_str()), (WINDOW_KIND_ID, BODY_KEY));
    assert_eq!((view.id.as_str(), view.body_key.as_str()), (VIEW_KIND_ID, VIEW_BODY_KEY));
    assert!(matches!(editor.surface_kind, SurfaceKind::Table) && matches!(view.surface_kind, SurfaceKind::World3d));
    assert_ne!(BimLabels::NATIVE_EN.window_family.as_str(), BimLabels::NATIVE_DE.window_family.as_str());
}

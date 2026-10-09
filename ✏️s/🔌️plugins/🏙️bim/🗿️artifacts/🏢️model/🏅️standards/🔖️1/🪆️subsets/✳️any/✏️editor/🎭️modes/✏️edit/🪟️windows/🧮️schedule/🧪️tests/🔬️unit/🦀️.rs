use super::*;
use crate::schedule_kit::{preset, PRESETS};
use crate::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
use protocol::Inference;

const HOUSE: &str = include_str!("../../../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

fn house() -> (ModelSnapshot, ModelInference) {
    let mut snapshot = decode_model_snapshot_json(HOUSE).expect("the committed house decodes");
    snapshot.spaces.get_mut("sp-1").expect("the living room").floor_finish = Some("m-wood".into());
    for key in PRESETS {
        snapshot.schedules.insert(format!("sch-{key}"), preset(key, key).expect("the preset exists"));
    }
    let inference = ModelInference::infer(&snapshot).expect("infers");
    (snapshot, inference)
}

fn json(value: &DslValue) -> String {
    semio_framework_pack_json::to_json_string(value)
}

fn config(schedule: &str, editing: bool) -> BimScheduleWindowConfig {
    BimScheduleWindowConfig { schedule: schedule.into(), editing }
}

#[semio_framework_async_macros::async_test]
async fn the_list_has_a_row_per_schedule_with_show_export_and_delete_and_a_row_per_preset() {
    let (snapshot, inference) = house();
    let rows = list_rows(&snapshot, &inference, &BimLabels::NATIVE_EN);
    assert_eq!(rows.len(), snapshot.schedules.len() + PRESETS.len());
    let door = rows.iter().map(json).find(|row| row.contains("\"sch-door\"")).expect("the door schedule row");
    for action in ["\"setView\"", "\"exportScheduleCsv\"", "\"deleteSelection\""] {
        assert!(door.contains(action), "a schedule row offers {action}: {door}");
    }
    assert!(door.contains("\"Doors\"") || door.contains("\"door\""), "the row names the schedule");
    for key in PRESETS {
        assert!(rows.iter().map(json).any(|row| row.contains(&format!("preset:{key}")) && row.contains("\"createEntity\"")), "a preset row creates the {key} schedule");
    }
}

#[semio_framework_async_macros::async_test]
async fn every_preset_is_named_in_both_languages_and_the_names_differ() {
    let (english, german): (Vec<String>, Vec<String>) = PRESETS.iter().map(|key| (preset_label(&BimLabels::NATIVE_EN, key), preset_label(&BimLabels::NATIVE_DE, key))).unzip();
    for names in [&english, &german] {
        let mut sorted = names.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), PRESETS.len(), "every preset has its own name: {names:?}");
        assert!(names.iter().all(|name| !name.is_empty()));
    }
    assert_eq!(preset_label(&BimLabels::NATIVE_EN, "finish"), "Room finish schedule");
    assert_eq!(preset_label(&BimLabels::NATIVE_DE, "finish"), "Raumbelagsliste");
    assert_ne!(english, german);
}

#[semio_framework_async_macros::async_test]
async fn the_table_keys_item_rows_by_element_group_rows_by_position_and_labels_the_total() {
    let (snapshot, inference) = house();
    let (columns, rows) = table_parts(&snapshot.schedules["sch-wall"], &inference, "sch-wall", &BimLabels::NATIVE_EN);
    assert_eq!(columns.len(), snapshot.schedules["sch-wall"].columns.len());
    let ids: Vec<String> = rows.iter().map(|row| json(row)).collect();
    assert!(ids.iter().any(|row| row.contains("\"id\":\"w-south\"")), "an item row is selected by its element id");
    assert!(ids.iter().any(|row| row.contains("\"id\":\"group:")), "a group row is keyed by its position");
    let total = ids.last().expect("rows");
    assert!(total.contains("\"id\":\"total\"") && total.contains("Total"), "the grand total row is labelled in the locale: {total}");
    let (_, german) = table_parts(&snapshot.schedules["sch-wall"], &inference, "sch-wall", &BimLabels::NATIVE_DE);
    assert!(json(german.last().expect("rows")).contains("Summe"));
}

#[semio_framework_async_macros::async_test]
async fn headings_and_enumerated_cells_follow_the_locale() {
    let (snapshot, inference) = house();
    let (english, _) = table_parts(&snapshot.schedules["sch-door"], &inference, "sch-door", &BimLabels::NATIVE_EN);
    let (german, rows) = table_parts(&snapshot.schedules["sch-door"], &inference, "sch-door", &BimLabels::NATIVE_DE);
    assert_ne!(english.iter().map(json).collect::<Vec<_>>(), german.iter().map(json).collect::<Vec<_>>());
    assert!(german.iter().map(json).any(|column| column.contains("Anschlag")), "the swing column speaks German");
    assert!(rows.iter().map(json).any(|row| row.contains("Rechts") || row.contains("Links")), "a swing token shows as a word");
}

#[semio_framework_async_macros::async_test]
async fn the_finish_schedule_shows_the_surface_and_the_material_name_of_every_finished_room() {
    let (snapshot, inference) = house();
    let (columns, rows) = table_parts(&snapshot.schedules["sch-finish"], &inference, "sch-finish", &BimLabels::NATIVE_DE);
    assert!(columns.iter().map(json).any(|column| column.contains("Bauteilseite")) && columns.iter().map(json).any(|column| column.contains("Belagsfläche")));
    let text: String = rows.iter().map(json).collect();
    assert!(text.contains("Boden") && text.contains("Decke") && text.contains("Wände"), "the surfaces are words of the locale: {text}");
    assert!(text.contains("Oak"), "the finish shows the name of its material");
}

#[semio_framework_async_macros::async_test]
async fn the_window_renders_the_list_the_table_and_the_editor_in_both_languages() {
    let (snapshot, inference) = house();
    for labels in [&BimLabels::NATIVE_EN, &BimLabels::NATIVE_DE] {
        assert!(render(&snapshot, &inference, &config("", false), labels).is_ok(), "the list");
        assert!(render(&snapshot, &inference, &config("sch-room", false), labels).is_ok(), "the table");
        assert!(render(&snapshot, &inference, &config("sch-finish", true), labels).is_ok(), "the definition editor");
        assert!(render(&snapshot, &inference, &config("sch-vanished", false), labels).is_ok(), "an unknown schedule falls back to the list");
    }
    assert!(matches!(definition().surface_kind, SurfaceKind::Table));
    assert!(render(&ModelSnapshot::default(), &ModelInference::default(), &BimScheduleWindowConfig::default(), &BimLabels::NATIVE_EN).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn the_editor_offers_the_finish_fields_only_to_finish_schedules() {
    let (snapshot, _) = house();
    let finish = editor_rows(&snapshot, "sch-finish", &BimLabels::NATIVE_EN).iter().map(json).collect::<String>();
    let walls = editor_rows(&snapshot, "sch-wall", &BimLabels::NATIVE_EN).iter().map(json).collect::<String>();
    assert!(finish.contains("Finish area") && finish.contains("Surface"));
    assert!(!walls.contains("Finish area"), "a wall schedule does not offer finish fields");
    assert!(editor_rows(&snapshot, "no-such-schedule", &BimLabels::NATIVE_EN).is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_row_is_picked_under_the_kind_of_the_elements_it_stands_for_and_never_under_a_fixed_kind() {
    let (snapshot, inference) = house();
    let kind = |key: &str| row_kind(&snapshot, &inference, &format!("sch-{key}"));
    assert_eq!((kind("wall").as_deref(), kind("room").as_deref(), kind("door").as_deref(), kind("window").as_deref()), (Some("wall"), Some("space"), Some("opening"), Some("opening")));
    assert_eq!(kind("material"), None, "a material take-off stands for library entries, which the elements domain does not hold");
    assert_eq!(row_kind(&snapshot, &inference, "sch-nothing"), None);
    for key in ["wall", "room", "material"] {
        assert!(render(&snapshot, &inference, &config(&format!("sch-{key}"), false), &BimLabels::NATIVE_EN).is_ok(), "{key} renders with or without a pick kind");
    }
}

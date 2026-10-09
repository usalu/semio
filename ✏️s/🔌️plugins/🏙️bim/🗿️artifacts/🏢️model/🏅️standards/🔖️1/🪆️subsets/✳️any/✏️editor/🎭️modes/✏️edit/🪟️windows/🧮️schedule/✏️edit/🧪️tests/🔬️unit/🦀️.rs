use super::*;

fn walls() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    snapshot.schedules.insert("sch".into(), created("wall", "Walls"));
    snapshot
}

fn edit(part: &str, op: &str, key: &str, value: &str) -> Edit {
    Edit { part: part.into(), op: op.into(), key: key.into(), value: value.into() }
}

fn tokens(columns: &[ScheduleColumn]) -> Vec<String> {
    columns.iter().map(|column| column.key.token()).collect()
}

#[semio_framework_async_macros::async_test]
async fn an_edit_carries_exactly_the_changed_list_of_the_set_schedule_payload() {
    let snapshot = walls();
    let payload = apply(&snapshot, "sch", &edit("column", "add", "mass", "")).expect("adds a column");
    let columns = payload.columns.expect("the column list");
    assert_eq!(columns.len(), snapshot.schedules["sch"].columns.len() + 1);
    assert_eq!(columns.last().map(|column| column.key.token()), Some("mass".to_string()));
    assert!(payload.name.is_none() && payload.sort.is_none() && payload.filter.is_none() && payload.group.is_none() && payload.itemize.is_none() && payload.storeys.is_none() && payload.phases.is_none() && payload.category.is_none());
}

#[semio_framework_async_macros::async_test]
async fn columns_move_toggle_their_sum_and_take_a_heading() {
    let snapshot = walls();
    let before = tokens(&snapshot.schedules["sch"].columns);
    let moved = apply(&snapshot, "sch", &edit("column", "down", &before[0], "")).expect("moves down").columns.expect("columns");
    assert_eq!(tokens(&moved)[..2], [before[1].clone(), before[0].clone()]);
    let unmoved = apply(&snapshot, "sch", &edit("column", "up", &before[0], "")).expect("the first column stays where it is").columns.expect("columns");
    assert_eq!(tokens(&unmoved), before);
    let total = &snapshot.schedules["sch"].columns[3];
    let toggled = apply(&snapshot, "sch", &edit("column", "total", &total.key.token(), "")).expect("toggles").columns.expect("columns");
    assert_eq!(toggled[3].total, !total.total);
    let headed = apply(&snapshot, "sch", &edit("column", "heading", &before[0], "Label")).expect("headings").columns.expect("columns");
    assert_eq!(headed[0].heading.as_deref(), Some("Label"));
    let cleared = apply(&snapshot, "sch", &edit("column", "heading", &before[0], "  ")).expect("clears").columns.expect("columns");
    assert_eq!(cleared[0].heading, None);
}

#[semio_framework_async_macros::async_test]
async fn filters_are_added_cycled_valued_and_removed_by_index() {
    let mut snapshot = walls();
    let added = apply(&snapshot, "sch", &edit("filter", "add", "length", "")).expect("adds").filter.expect("filters");
    assert_eq!((added.len(), added[0].op, added[0].value.as_str()), (1, ScheduleOp::NotEmpty, ""));
    snapshot.schedules.get_mut("sch").expect("schedule").filter = added;
    let compared = apply(&snapshot, "sch", &edit("filter", "op", "0", ">")).expect("sets the comparison").filter.expect("filters");
    assert_eq!((compared[0].op, compared[0].value.as_str()), (ScheduleOp::Greater, "0"), "a binary comparison gets a value to compare with");
    let valued = apply(&snapshot, "sch", &edit("filter", "value", "0", "4")).expect("sets the value").filter.expect("filters");
    assert_eq!(valued[0].value, "4");
    assert!(apply(&snapshot, "sch", &edit("filter", "remove", "0", "")).expect("removes").filter.expect("filters").is_empty());
    assert_eq!(apply(&snapshot, "sch", &edit("filter", "remove", "5", "")).err(), Some("bim.schedule.edit-invalid"));
}

#[semio_framework_async_macros::async_test]
async fn scope_toggles_storeys_and_phases_and_itemize_flips() {
    let mut snapshot = walls();
    let scoped = apply(&snapshot, "sch", &edit("storey", "toggle", "st-1", "")).expect("scopes").storeys.expect("storeys");
    assert_eq!(scoped, vec!["st-1".to_string()]);
    snapshot.schedules.get_mut("sch").expect("schedule").storeys = scoped;
    assert!(apply(&snapshot, "sch", &edit("storey", "toggle", "st-1", "")).expect("unscopes").storeys.expect("storeys").is_empty());
    assert_eq!(apply(&snapshot, "sch", &edit("phase", "toggle", "NEW", "")).expect("phases").phases, Some(vec![Phase::New]));
    assert_eq!(apply(&snapshot, "sch", &edit("phase", "toggle", "someday", "")).err(), Some("bim.schedule.phase-unknown"));
    assert_eq!(apply(&snapshot, "sch", &edit("itemize", "toggle", "", "")).expect("flips").itemize, Some(!snapshot.schedules["sch"].itemize));
}

#[semio_framework_async_macros::async_test]
async fn refused_edits_name_their_reason_and_the_same_edit_gives_the_same_payload() {
    let snapshot = walls();
    assert_eq!(apply(&snapshot, "missing", &edit("name", "set", "", "x")).err(), Some("bim.schedule.missing"));
    assert_eq!(apply(&snapshot, "sch", &edit("shape", "set", "", "")).err(), Some("bim.schedule.part-unknown"));
    assert_eq!(apply(&snapshot, "sch", &edit("column", "add", "no_such_field", "")).err(), Some("bim.schedule.key-unknown"));
    assert_eq!(apply(&snapshot, "sch", &edit("category", "set", "", "no-category")).err(), Some("bim.schedule.category-unknown"));
    let existing = snapshot.schedules["sch"].columns[0].key.token();
    assert_eq!(apply(&snapshot, "sch", &edit("column", "add", &existing, "")).err(), Some("bim.schedule.edit-invalid"), "a column is added once");
    let again = edit("group", "add", "storey", "");
    assert_eq!(apply(&snapshot, "sch", &again).expect("adds"), apply(&snapshot, "sch", &again).expect("adds"));
}

#[semio_framework_async_macros::async_test]
async fn a_created_schedule_is_the_named_preset_or_the_wall_schedule() {
    assert_eq!(created("door", "Doors").category, ScheduleCategory::Door);
    assert_eq!(created("unknown", "Anything").category, ScheduleCategory::Wall);
    assert_eq!(created("room", "Rooms").name, "Rooms");
    assert!(presets().iter().all(|key| preset(key, "x").is_some()));
}

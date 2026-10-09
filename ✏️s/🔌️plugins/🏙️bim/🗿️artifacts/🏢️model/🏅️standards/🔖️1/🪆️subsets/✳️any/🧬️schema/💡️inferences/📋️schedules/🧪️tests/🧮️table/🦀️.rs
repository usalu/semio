use super::kit::{house, items, plain, scheduled, table_of, texts};
use super::table::{natural, passes};
use super::*;
use crate::standards::v1::subsets::any::io::text::inferences::schedules::table_json;
use crate::schedule_kit::preset;
use crate::{ModelInference, ScheduleCategory, ScheduleColumn, ScheduleField, ScheduleFilter, ScheduleGroup, ScheduleOp, ScheduleSort};
use protocol::Inference;

fn number(cell: &ScheduleCell) -> f64 {
    match cell {
        ScheduleCell::Number { value } => *value,
        other => panic!("a number cell, got {other:?}"),
    }
}

fn filter(field: ScheduleField, op: ScheduleOp, value: &str) -> ScheduleFilter {
    ScheduleFilter { key: ScheduleKey::field(field), op, value: value.into() }
}

fn sort(field: ScheduleField, descending: bool) -> ScheduleSort {
    ScheduleSort { key: ScheduleKey::field(field), descending }
}

fn group(field: ScheduleField) -> ScheduleGroup {
    ScheduleGroup { key: ScheduleKey::field(field) }
}

#[semio_framework_async_macros::async_test]
async fn natural_order_compares_digit_runs_as_integers_and_ignores_leading_zeros() {
    let mut names = vec!["D10", "d2", "D1", "D02", "D2", "Door", "10", "9"];
    names.sort_by(|a, b| natural(a, b));
    assert_eq!(names, ["9", "10", "D1", "D02", "D2", "d2", "D10", "Door"], "digits before letters, 02 ties with 2 by the raw text");
    assert_eq!(natural("a1b2", "a1b10"), std::cmp::Ordering::Less);
    assert_eq!(natural("a", "a1"), std::cmp::Ordering::Less, "a prefix sorts first");
    assert_eq!(natural("", ""), std::cmp::Ordering::Equal);
}

#[semio_framework_async_macros::async_test]
async fn every_filter_operator_decides_numbers_by_value_and_texts_by_case_folded_text() {
    let cell = ScheduleCell::number(10.0);
    for (op, value, expected) in [
        (ScheduleOp::Equals, "10.0", true),
        (ScheduleOp::Equals, "10.000000000001", true),
        (ScheduleOp::NotEquals, "10", false),
        (ScheduleOp::Greater, "9.99", true),
        (ScheduleOp::Greater, "10", false),
        (ScheduleOp::GreaterOrEqual, "10", true),
        (ScheduleOp::Less, "10.5", true),
        (ScheduleOp::Less, "2", false),
        (ScheduleOp::LessOrEqual, "10", true),
        (ScheduleOp::Contains, "1", true),
        (ScheduleOp::Contains, "9", false),
    ] {
        assert_eq!(passes(&cell, op, value), expected, "10 {} {value}", op.symbol());
    }
    let text = ScheduleCell::text("Door 10");
    for (op, value, expected) in [
        (ScheduleOp::Equals, "door 10", true),
        (ScheduleOp::NotEquals, "Door 10", false),
        (ScheduleOp::Contains, "R 1", true),
        (ScheduleOp::Greater, "Door 9", true),
        (ScheduleOp::GreaterOrEqual, "door 10", true),
        (ScheduleOp::Less, "Door 9", false),
        (ScheduleOp::LessOrEqual, "Door 10", true),
    ] {
        assert_eq!(passes(&text, op, value), expected, "'Door 10' {} {value}", op.symbol());
    }
    assert!(passes(&ScheduleCell::Empty, ScheduleOp::Empty, "") && !passes(&ScheduleCell::Empty, ScheduleOp::NotEmpty, ""));
    assert!(!passes(&ScheduleCell::Empty, ScheduleOp::Equals, "x") && passes(&ScheduleCell::Empty, ScheduleOp::NotEquals, "x"));
    assert!(!passes(&ScheduleCell::number(5.0), ScheduleOp::Equals, "five"), "a number against a word compares as text");
}

#[semio_framework_async_macros::async_test]
async fn rows_sort_in_natural_order_ascending_and_descending() {
    let mut snapshot = house();
    snapshot.openings.get_mut("o-door-1").expect("the door").name = "D10".into();
    snapshot.openings.get_mut("o-door-2").expect("the door").name = "D2".into();
    let mut schedule = plain(ScheduleCategory::Door, &[(ScheduleField::Name, false)]);
    schedule.sort = vec![sort(ScheduleField::Name, false)];
    assert_eq!(texts(&table_of(&snapshot, schedule.clone()), 0), ["D2", "D10"], "D2 before D10, not after");
    schedule.sort = vec![sort(ScheduleField::Name, true)];
    assert_eq!(texts(&table_of(&snapshot, schedule), 0), ["D10", "D2"]);
}

#[semio_framework_async_macros::async_test]
async fn rows_with_equal_sort_keys_keep_the_element_id_order_and_later_keys_break_ties() {
    let snapshot = house();
    let mut schedule = plain(ScheduleCategory::Window, &[(ScheduleField::Id, false), (ScheduleField::Type, false)]);
    schedule.sort = vec![sort(ScheduleField::Type, false)];
    assert_eq!(texts(&table_of(&snapshot, schedule.clone()), 0), ["o-win-1", "o-win-2", "o-win-arc"], "one type: the id order stays");
    schedule.sort = vec![sort(ScheduleField::Type, false), sort(ScheduleField::Id, true)];
    assert_eq!(texts(&table_of(&snapshot, schedule), 0), ["o-win-arc", "o-win-2", "o-win-1"]);
}

#[semio_framework_async_macros::async_test]
async fn numbers_sort_by_value_and_empty_cells_sort_first() {
    let snapshot = house();
    let mut schedule = plain(ScheduleCategory::Wall, &[(ScheduleField::Id, false), (ScheduleField::Length, false)]);
    schedule.sort = vec![sort(ScheduleField::Length, true)];
    let lengths: Vec<f64> = items(&table_of(&snapshot, schedule)).iter().map(|row| number(&row.cells[1])).collect();
    assert!(lengths.windows(2).all(|pair| pair[0] >= pair[1]) && lengths.len() == 7, "{lengths:?}");
    let mut schedule = plain(ScheduleCategory::Space, &[(ScheduleField::Id, false), (ScheduleField::Name, false)]);
    schedule.sort = vec![sort(ScheduleField::Id, true)];
    assert_eq!(texts(&table_of(&snapshot, schedule), 0), ["sp-2", "sp-1"]);
}

#[semio_framework_async_macros::async_test]
async fn a_row_stays_only_when_every_filter_holds() {
    let snapshot = house();
    let mut schedule = plain(ScheduleCategory::Door, &[(ScheduleField::Id, false), (ScheduleField::Width, false)]);
    schedule.filter = vec![filter(ScheduleField::Width, ScheduleOp::GreaterOrEqual, "1.0")];
    assert_eq!(texts(&table_of(&snapshot, schedule.clone()), 0), ["o-door-2"], "only the double door is that wide");
    schedule.filter = vec![filter(ScheduleField::Width, ScheduleOp::Less, "1.0"), filter(ScheduleField::Storey, ScheduleOp::Equals, "ground")];
    assert_eq!(texts(&table_of(&snapshot, schedule.clone()), 0), ["o-door-1"]);
    schedule.filter = vec![filter(ScheduleField::Name, ScheduleOp::Contains, "door"), filter(ScheduleField::Storey, ScheduleOp::Equals, "Basement")];
    assert!(table_of(&snapshot, schedule.clone()).rows.is_empty(), "no door is in the basement");
    schedule.filter = vec![filter(ScheduleField::Swing, ScheduleOp::Empty, "")];
    assert!(table_of(&snapshot, schedule).rows.is_empty(), "every door has a swing");
    let mut windows = plain(ScheduleCategory::Window, &[(ScheduleField::Id, false)]);
    windows.filter = vec![filter(ScheduleField::Swing, ScheduleOp::Empty, "")];
    assert_eq!(table_of(&snapshot, windows).items, 3, "no window has a swing");
}

#[semio_framework_async_macros::async_test]
async fn a_property_filter_and_a_property_sort_read_the_authored_property_set() {
    let snapshot = house();
    let rating = ScheduleKey::property("Pset_WallCommon", "FireRating");
    let mut schedule = plain(ScheduleCategory::Wall, &[(ScheduleField::Id, false)]);
    schedule.columns.push(ScheduleColumn { key: rating.clone(), heading: None, total: false });
    schedule.filter = vec![ScheduleFilter { key: rating.clone(), op: ScheduleOp::NotEmpty, value: String::new() }];
    let table = table_of(&snapshot, schedule.clone());
    assert_eq!((texts(&table, 0), texts(&table, 1)), (vec!["w-south".to_string()], vec!["EI60".to_string()]));
    schedule.filter.clear();
    schedule.sort = vec![crate::ScheduleSort { key: rating, descending: true }];
    assert_eq!(texts(&table_of(&snapshot, schedule), 0)[0], "w-south", "a text sorts after the empty cells of the other walls when descending");
}

#[semio_framework_async_macros::async_test]
async fn a_group_closes_with_a_subtotal_row_and_the_grand_total_sums_the_items() {
    let snapshot = house();
    let table = table_of(&snapshot, preset("wall", "Walls").expect("the wall preset"));
    assert_eq!(table.items as usize, snapshot.walls.len());
    let types: std::collections::BTreeSet<&str> = snapshot.walls.values().map(|wall| wall.wall_type.as_str()).collect();
    let groups: Vec<usize> = table.rows.iter().enumerate().filter(|(_, row)| row.kind == RowKind::Group).map(|(index, _)| index).collect();
    assert_eq!(groups.len(), types.len());
    let mut start = 0;
    for end in groups {
        let members = &table.rows[start..end];
        assert!(members.iter().all(|row| row.kind == RowKind::Item && row.level == 1), "items of a group carry the number of grouping levels");
        let sum: f64 = members.iter().map(|row| number(&row.cells[3])).sum();
        assert!((number(&table.rows[end].cells[3]) - sum).abs() < 1e-9, "the subtotal is the sum of the length of its items");
        assert_eq!(table.rows[end].level, 0);
        let mut elements: Vec<String> = members.iter().flat_map(|row| row.elements.clone()).collect();
        let mut closed = table.rows[end].elements.clone();
        elements.sort();
        closed.sort();
        assert_eq!(elements, closed, "a group row selects the elements it closes");
        start = end + 1;
    }
    let total = table.rows.last().expect("rows");
    assert_eq!((total.kind, total.level, total.elements.is_empty()), (RowKind::Total, 0, true));
    let sum: f64 = items(&table).iter().map(|row| number(&row.cells[3])).sum();
    assert!((number(&total.cells[3]) - sum).abs() < 1e-9);
    assert_eq!(total.cells[0], ScheduleCell::Empty, "the label of the total row is the viewer's to add");
}

#[semio_framework_async_macros::async_test]
async fn nested_grouping_levels_nest_their_subtotals_inside_out() {
    let snapshot = house();
    let mut schedule = plain(ScheduleCategory::Wall, &[(ScheduleField::Id, false), (ScheduleField::Length, true)]);
    schedule.group = vec![group(ScheduleField::Storey), group(ScheduleField::Type)];
    let table = table_of(&snapshot, schedule);
    let levels: Vec<(RowKind, u32)> = table.rows.iter().map(|row| (row.kind, row.level)).collect();
    let storeys: std::collections::BTreeSet<&str> = snapshot.walls.values().map(|wall| wall.storey.as_str()).collect();
    let pairs: std::collections::BTreeSet<(&str, &str)> = snapshot.walls.values().map(|wall| (wall.storey.as_str(), wall.wall_type.as_str())).collect();
    assert_eq!(levels.iter().filter(|(kind, level)| *kind == RowKind::Group && *level == 1).count(), pairs.len());
    assert_eq!(levels.iter().filter(|(kind, level)| *kind == RowKind::Group && *level == 0).count(), storeys.len());
    assert!(levels.iter().filter(|(kind, _)| *kind == RowKind::Item).all(|(_, level)| *level == 2));
    for (index, (kind, level)) in levels.iter().enumerate() {
        if (*kind, *level) == (RowKind::Group, 0) {
            assert_eq!(levels[index - 1], (RowKind::Group, 1), "a storey closes right after its last type subtotal");
        }
    }
    let storey_total: f64 = table.rows.iter().filter(|row| row.kind == RowKind::Group && row.level == 0).map(|row| number(&row.cells[1])).sum();
    assert!((storey_total - number(&table.rows.last().expect("rows").cells[1])).abs() < 1e-9, "the storey subtotals add up to the grand total");
}

#[semio_framework_async_macros::async_test]
async fn a_schedule_that_is_not_itemized_collapses_every_group_into_one_row() {
    let snapshot = house();
    let mut schedule = preset("wall", "Walls").expect("the wall preset");
    schedule.itemize = false;
    let table = table_of(&snapshot, schedule);
    let kinds: Vec<RowKind> = table.rows.iter().map(|row| row.kind).collect();
    assert_eq!(kinds, [RowKind::Group, RowKind::Group, RowKind::Total], "a row per wall type, then the grand total");
    let covered: usize = table.rows.iter().filter(|row| row.kind == RowKind::Group).map(|row| row.elements.len()).sum();
    assert_eq!(covered, snapshot.walls.len());
    assert!(table.rows.iter().filter(|row| row.kind == RowKind::Group).all(|row| row.cells[1] != ScheduleCell::Empty), "the type is common to the group, so it is shown");
    assert_eq!(table.items as usize, snapshot.walls.len(), "collapsing does not change how many source rows passed");
}

#[semio_framework_async_macros::async_test]
async fn without_grouping_equal_rows_merge_and_their_counts_add() {
    let snapshot = house();
    let mut schedule = plain(ScheduleCategory::Window, &[(ScheduleField::Type, false), (ScheduleField::Count, true)]);
    schedule.itemize = false;
    let table = table_of(&snapshot, schedule);
    assert_eq!(table.rows.len(), 2, "one window type: one merged row and the total");
    assert_eq!(table.rows[0].cells, [ScheduleCell::text("Window 120"), ScheduleCell::number(3.0)]);
    assert_eq!(table.rows[0].elements.len(), 3);
    assert_eq!(table.rows[1].cells[1], ScheduleCell::number(3.0));
}

#[semio_framework_async_macros::async_test]
async fn nothing_to_list_gives_no_rows_and_no_total_row_even_with_summed_columns() {
    let snapshot = house();
    let mut schedule = plain(ScheduleCategory::Wall, &[(ScheduleField::Name, false), (ScheduleField::Length, true)]);
    schedule.filter = vec![filter(ScheduleField::Name, ScheduleOp::Equals, "no wall is called this")];
    let table = table_of(&snapshot, schedule.clone());
    assert_eq!((table.rows.len(), table.items), (0, 0));
    assert_eq!(table.keys.len(), 2, "the table still knows its columns");
    schedule.filter.clear();
    schedule.itemize = false;
    schedule.group = vec![group(ScheduleField::Name)];
    schedule.storeys = vec!["st-roof".into()];
    assert!(table_of(&snapshot, schedule).rows.is_empty());
    let railings = plain(ScheduleCategory::Void, &[(ScheduleField::Name, false)]);
    let empty = table_of(&crate::ModelSnapshot::default(), railings);
    assert!(empty.rows.is_empty() && empty.items == 0);
}

#[semio_framework_async_macros::async_test]
async fn a_table_without_a_summed_column_has_no_total_row_and_a_summed_text_column_stays_empty() {
    let snapshot = house();
    let table = table_of(&snapshot, plain(ScheduleCategory::Window, &[(ScheduleField::Name, false), (ScheduleField::Width, false)]));
    assert!(table.rows.iter().all(|row| row.kind == RowKind::Item));
    let table = table_of(&snapshot, plain(ScheduleCategory::Window, &[(ScheduleField::Name, true), (ScheduleField::Width, true)]));
    let total = table.rows.last().expect("rows");
    assert_eq!((total.kind, total.cells[0].clone()), (RowKind::Total, ScheduleCell::Empty), "texts cannot be summed");
    assert!((number(&total.cells[1]) - items(&table).iter().map(|row| number(&row.cells[1])).sum::<f64>()).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn the_room_and_door_and_window_presets_infer_one_item_per_element() {
    let snapshot = scheduled();
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(inferred.schedules.len(), snapshot.schedules.len());
    assert_eq!(inferred.schedules["sch-door"].items, 2);
    assert_eq!(inferred.schedules["sch-window"].items, 3);
    assert_eq!(inferred.schedules["sch-room"].items as usize, snapshot.spaces.len());
    let resolved = snapshot.spaces.keys().filter(|id| inferred.quantities.elements.get(*id).is_some_and(|quantity| !quantity.finishes.is_empty())).count();
    assert_eq!(inferred.schedules["sch-finish"].items as usize, 3 * resolved);
    let doors = &inferred.schedules["sch-door"];
    assert_eq!(texts(doors, 0), ["Door 1", "Double door"], "sorted by storey, ground first");
    assert_eq!(texts(doors, 6), ["right", "right"]);
}

#[semio_framework_async_macros::async_test]
async fn the_oracle_json_marks_the_empty_cell_null_and_texts_and_numbers_by_their_type() {
    let snapshot = scheduled();
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    let json: serde_json::Value = serde_json::from_str(&table_json(&inferred.schedules)).expect("a JSON table");
    let doors = &json["sch-door"];
    assert_eq!(doors["items"], 2);
    let first = &doors["rows"][0];
    assert_eq!((first["kind"].as_str(), first["level"].as_u64()), (Some("Item"), Some(0)), "an ungrouped table has item rows at level 0");
    assert_eq!(first["elements"], serde_json::json!(["o-door-1"]));
    assert!(first["cells"][0].is_string() && first["cells"][3].is_number());
    let window = &json["sch-window"]["rows"][0]["cells"];
    assert_eq!((window[0].as_str(), window[5].as_f64()), (Some("Curved window"), Some(2.0)), "the first window by storey and name, with its two panes");
    let rooms = &json["sch-room"]["rows"][0]["cells"];
    assert!(rooms.as_array().expect("cells").iter().any(|cell| cell.is_null()) || json["sch-room"]["items"] == 0, "a cell with no value is null");
}

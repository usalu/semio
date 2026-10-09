use super::table::{natural, order, passes};
use super::*;
use crate::standards::v1::subsets::any::io::text::inferences::schedules::table_json;
use crate::schedule_kit::preset;
use crate::{ModelInference, Phase, ScheduleOp};
use protocol::Inference;
use std::cmp::Ordering;

fn demo() -> ModelSnapshot {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    snapshot.schedules.insert("sch-walls".into(), preset("wall", "Walls").expect("the wall preset"));
    snapshot
}

fn length(row: &ScheduleRow) -> f64 {
    match &row.cells[3] {
        ScheduleCell::Number { value } => *value,
        other => panic!("a length cell, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn cells_are_empty_for_the_empty_text_and_display_in_shortest_round_trip_form() {
    assert_eq!(ScheduleCell::text(""), ScheduleCell::Empty);
    assert_eq!(ScheduleCell::text("Brick").display(), "Brick");
    assert_eq!(ScheduleCell::number(2.5).display(), "2.5");
    assert_eq!(ScheduleCell::number(8.0).display(), "8");
    assert_eq!(ScheduleCell::Empty.display(), "");
}

#[semio_framework_async_macros::async_test]
async fn texts_order_naturally_and_cells_order_empty_then_numbers_then_texts() {
    assert_eq!(natural("D2", "D10"), Ordering::Less);
    assert_eq!(natural("d10", "D2"), Ordering::Greater);
    assert_eq!(natural("a", "A"), Ordering::Greater, "equal when folded, the raw text breaks the tie");
    let (empty, number, text) = (ScheduleCell::Empty, ScheduleCell::number(7.0), ScheduleCell::text("a"));
    assert_eq!(order(&empty, &number), Ordering::Less);
    assert_eq!(order(&number, &text), Ordering::Less);
    assert_eq!(order(&text, &empty), Ordering::Greater);
    assert_eq!(order(&ScheduleCell::number(1.0), &ScheduleCell::number(2.0)), Ordering::Less);
}

#[semio_framework_async_macros::async_test]
async fn filters_compare_numbers_as_numbers_and_everything_else_as_case_folded_text() {
    let number = ScheduleCell::number(3.0);
    assert!(passes(&number, ScheduleOp::Equals, "3.0") && passes(&number, ScheduleOp::GreaterOrEqual, "3") && !passes(&number, ScheduleOp::Greater, "3"));
    assert!(passes(&number, ScheduleOp::Less, "10"), "numerically, not as text");
    let text = ScheduleCell::text("Brick 300");
    assert!(passes(&text, ScheduleOp::Contains, "brick") && passes(&text, ScheduleOp::Equals, "BRICK 300") && passes(&text, ScheduleOp::NotEquals, "timber"));
    assert!(passes(&ScheduleCell::Empty, ScheduleOp::Empty, "") && passes(&text, ScheduleOp::NotEmpty, "") && !passes(&text, ScheduleOp::Empty, ""));
}

#[semio_framework_async_macros::async_test]
async fn the_scope_selects_the_candidates_by_category_storey_and_phase() {
    let snapshot = demo();
    let mut schedule = snapshot.schedules["sch-walls"].clone();
    let all: Vec<String> = snapshot.walls.keys().cloned().collect();
    assert_eq!(rows::candidates(&snapshot, &schedule), all);
    schedule.storeys = vec!["no-such-storey".into()];
    assert!(rows::candidates(&snapshot, &schedule).is_empty());
    schedule.storeys.clear();
    schedule.phases = vec![Phase::Demolished];
    assert!(rows::candidates(&snapshot, &schedule).is_empty(), "no wall of the demo is demolished");
    schedule.phases = vec![Phase::New];
    assert_eq!(rows::candidates(&snapshot, &schedule), all, "the demo walls are new");
}

#[semio_framework_async_macros::async_test]
async fn the_wall_schedule_lists_every_wall_once_closes_each_type_and_totals_the_lengths() {
    let snapshot = demo();
    let inferred = ModelInference::infer(&snapshot).expect("infers");
    let table = &inferred.schedules["sch-walls"];
    assert_eq!(table.items as usize, snapshot.walls.len());
    let items: Vec<&ScheduleRow> = table.rows.iter().filter(|row| row.kind == RowKind::Item).collect();
    let mut listed: Vec<String> = items.iter().flat_map(|row| row.elements.clone()).collect();
    listed.sort();
    assert_eq!(listed, snapshot.walls.keys().cloned().collect::<Vec<_>>());
    let types: std::collections::BTreeSet<&str> = snapshot.walls.values().map(|wall| wall.wall_type.as_str()).collect();
    assert_eq!(table.rows.iter().filter(|row| row.kind == RowKind::Group).count(), types.len(), "one subtotal row per wall type");
    let total = table.rows.last().expect("rows");
    assert_eq!(total.kind, RowKind::Total);
    let sum: f64 = items.iter().map(|row| length(row)).sum();
    assert!((length(total) - sum).abs() < 1e-9, "the grand total is the sum of the item lengths");
}

#[semio_framework_async_macros::async_test]
async fn the_table_and_its_oracle_json_are_deterministic_and_follow_the_authored_filter() {
    let mut snapshot = demo();
    let first = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(first, ModelInference::infer(&snapshot).expect("infers"));
    assert_eq!(table_json(&first.schedules), table_json(&ModelInference::infer(&snapshot).expect("infers").schedules));
    let schedule = snapshot.schedules.get_mut("sch-walls").expect("the schedule");
    schedule.filter.push(crate::ScheduleFilter { key: ScheduleKey::field(crate::ScheduleField::Name), op: ScheduleOp::Equals, value: "no wall is called this".into() });
    let filtered = ModelInference::infer(&snapshot).expect("infers");
    assert_eq!(filtered.schedules["sch-walls"].items, 0);
    assert!(filtered.schedules["sch-walls"].rows.is_empty());
    assert!(table_json(&filtered.schedules).contains("\"items\":0"));
}

#[semio_framework_async_macros::async_test]
async fn the_dependency_is_null_for_an_unknown_schedule_and_follows_the_definition() {
    let mut snapshot = demo();
    assert_eq!(dependency(&snapshot, "no-such-schedule"), DslValue::Null);
    let before = dependency(&snapshot, "sch-walls");
    assert_eq!(before, dependency(&snapshot, "sch-walls"));
    snapshot.schedules.get_mut("sch-walls").expect("the schedule").itemize = false;
    assert_ne!(before, dependency(&snapshot, "sch-walls"));
    assert!(READS.contains(&"schedules") && READS.contains(&"walls"));
}

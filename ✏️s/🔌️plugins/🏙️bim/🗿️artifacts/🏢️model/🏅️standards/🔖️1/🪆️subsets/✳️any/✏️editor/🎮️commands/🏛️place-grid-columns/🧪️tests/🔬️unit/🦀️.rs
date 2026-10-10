use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::{ColumnType, Material, MaterialCategory, Profile, Rgb};

fn line(label: &str, building: &str, start: (f64, f64), end: (f64, f64)) -> GridLine {
    GridLine { building: building.into(), label: label.into(), start: Point2 { x: start.0, y: start.1 }, end: Point2 { x: end.0, y: end.1 } }
}

fn gridded() -> (ModelSnapshot, String) {
    let mut snapshot = demo();
    snapshot.columns.clear();
    snapshot.grids.clear();
    let (storey, building) = snapshot.storeys.iter().next().map(|(id, row)| (id.clone(), row.building.clone())).expect("a storey");
    if snapshot.column_types.is_empty() {
        snapshot.materials.entry("m-steel".into()).or_insert(Material { name: "Steel".into(), category: MaterialCategory::Metal, color: Rgb { r: 0.5, g: 0.5, b: 0.5 }, density: 7850.0, conductivity: 50.0, specific_heat: 470.0 });
        snapshot.column_types.insert("ct".into(), ColumnType { name: "Column".into(), profile: Profile::Rectangle { width: 0.3, depth: 0.3 }, material: "m-steel".into() });
    }
    for (id, grid) in [("g-a", line("A", &building, (0.0, -1.0), (0.0, 5.0))), ("g-b", line("B", &building, (4.0, -1.0), (4.0, 5.0))), ("g-1", line("1", &building, (-1.0, 0.0), (5.0, 0.0))), ("g-2", line("2", &building, (-1.0, 3.0), (5.0, 3.0)))] {
        snapshot.grids.insert(id.into(), grid);
    }
    (snapshot, storey)
}

fn place(snapshot: &ModelSnapshot, storey: &str, ids: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(&[]);
    run(snapshot, |doc, cfg| handle(&PlaceGridColumns { storey: storey.into(), ids: ids.iter().map(|id| id.to_string()).collect() }, doc, cfg, &mut ctx))
}

#[test]
fn two_lines_cross_where_both_reach_and_never_when_parallel() {
    let a = line("A", "b", (0.0, 0.0), (0.0, 4.0));
    assert_eq!(crossing(&a, &line("1", "b", (-1.0, 2.0), (1.0, 2.0))), Some(Point2 { x: 0.0, y: 2.0 }));
    assert_eq!(crossing(&a, &line("far", "b", (1.0, 2.0), (3.0, 2.0))), None, "the second line stops short");
    assert_eq!(crossing(&a, &line("B", "b", (2.0, 0.0), (2.0, 4.0))), None, "parallel lines never cross");
    assert_eq!(crossings(&[&a, &line("1", "b", (-1.0, 2.0), (1.0, 2.0)), &line("2", "b", (-1.0, 2.0), (1.0, 2.0))]).len(), 1, "a crossing of three lines counts once");
}

#[semio_framework_async_macros::async_test]
async fn every_crossing_of_the_grid_gets_one_ordinary_create_column_in_one_emit() {
    let (snapshot, storey) = gridded();
    let emit = place(&snapshot, &storey, &[]).expect("places");
    assert_eq!(emit.artifact_mutations.len(), 4);
    assert!(emit.artifact_mutations.iter().all(|mutation| matches!(mutation, ModelMutation::CreateColumn(create) if create.column.storey == storey)));
    let after = applied(&snapshot, &emit);
    let mut at: Vec<(i64, i64)> = after.columns.values().map(|column| (column.position.x.round() as i64, column.position.y.round() as i64)).collect();
    at.sort();
    assert_eq!(at, vec![(0, 0), (0, 3), (4, 0), (4, 3)]);
    assert_eq!(after.columns.len(), 4, "the ids are distinct");
}

#[semio_framework_async_macros::async_test]
async fn a_crossing_with_a_column_is_not_taken_twice_and_a_full_grid_is_refused() {
    let (snapshot, storey) = gridded();
    let once = applied(&snapshot, &place(&snapshot, &storey, &[]).expect("places"));
    assert_eq!(place(&once, &storey, &[]).err().map(|fault| fault.code.0), Some("bim.grid-columns.crossing-missing".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn only_the_named_grid_lines_are_crossed() {
    let (snapshot, storey) = gridded();
    let emit = place(&snapshot, &storey, &["g-a", "g-1", "g-2"]).expect("places");
    assert_eq!(emit.artifact_mutations.len(), 2, "A crosses 1 and 2; 1 and 2 are parallel");
}

#[semio_framework_async_macros::async_test]
async fn a_model_without_a_column_type_is_refused_with_its_own_code() {
    let (mut snapshot, storey) = gridded();
    snapshot.column_types.clear();
    assert_eq!(place(&snapshot, &storey, &[]).err().map(|fault| fault.code.0), Some("bim.grid-columns.type-missing".to_string()));
}

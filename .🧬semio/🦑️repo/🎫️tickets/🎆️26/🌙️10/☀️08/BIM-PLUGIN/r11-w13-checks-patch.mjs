import { readFileSync, writeFileSync } from "node:fs";
const [checks, house, office] = process.argv.slice(2);
const edit = (file, swaps, marker) => {
  let t = readFileSync(file, "utf8");
  if (t.includes(marker)) return;
  const crlf = t.includes("\r\n");
  t = t.replaceAll("\r\n", "\n");
  for (const [from, to] of swaps) { if (!t.includes(from)) throw new Error(file.slice(-50) + " missing " + from.slice(0, 80)); t = t.replace(from, to); }
  writeFileSync(file, crlf ? t.replaceAll("\n", "\r\n") : t);
};
edit(checks, [[
`    assert_eq!(inferred.spaces.len(), model.spaces.len());
`,
`    assert_eq!(inferred.schedules.len(), model.schedules.len(), "every authored schedule has a table");
    for (id, schedule) in &model.schedules {
        schedules_add_up(id, schedule, &inferred.schedules[id]);
    }
    assert_eq!(inferred.spaces.len(), model.spaces.len());
`], [
`macro_rules! families {`,
`/// 📋️ A schedule table is consistent with its definition: it has a total row exactly when a column is summed and a row was listed, and every summed column of the total row is the sum of the rows the table lists (the items
/// when itemized, else the collapsed rows of the outermost level).
fn schedules_add_up(id: &str, schedule: &crate::Schedule, table: &crate::standards::v1::subsets::any::schema::inferences::schedules::ScheduleTable) {
    use crate::standards::v1::subsets::any::schema::inferences::schedules::{RowKind, ScheduleCell};
    assert_eq!(table.keys, schedule.columns.iter().map(|column| column.key.clone()).collect::<Vec<_>>(), "schedule {id} keeps its columns");
    let total = table.rows.iter().find(|row| row.kind == RowKind::Total);
    assert_eq!(total.is_some(), table.items > 0 && schedule.columns.iter().any(|column| column.total), "schedule {id} has a total row exactly when something is summed");
    let Some(total) = total else { return };
    let listed = |row: &&crate::standards::v1::subsets::any::schema::inferences::schedules::ScheduleRow| if schedule.itemize { row.kind == RowKind::Item } else { row.kind == RowKind::Group && row.level == 0 };
    for (index, column) in schedule.columns.iter().enumerate().filter(|(_, column)| column.total) {
        let sum: f64 = table.rows.iter().filter(listed).filter_map(|row| if let ScheduleCell::Number { value } = row.cells[index] { Some(value) } else { None }).sum();
        match total.cells[index] {
            ScheduleCell::Number { value } => assert!((value - sum).abs() < 1e-6 * sum.abs().max(1.0), "schedule {id}: the total of column {index} is {value}, its rows sum to {sum}"),
            ScheduleCell::Empty => assert!(sum == 0.0, "schedule {id}: column {index} sums to {sum} but its total is empty"),
            ref other => panic!("schedule {id}: the total of column {index} is {other:?}"),
        }
    }
}

macro_rules! families {`]], "fn schedules_add_up");

edit(house, [[
`#[semio_framework_async_macros::async_test]
async fn the_house_round_trips_and_validates() {`,
`#[semio_framework_async_macros::async_test]
async fn the_house_schedules_list_every_door_window_room_and_finish_of_the_building() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let items = |id: &str| inferred.schedules[id].items as usize;
    assert_eq!((items("sch-doors"), items("sch-windows"), items("sch-rooms")), (9, 25, 12), "a door, a window and a room schedule list every one of them");
    assert_eq!(items("sch-finishes"), 3 * model.spaces.len(), "a floor, a wall and a ceiling row for every room");
    let ground_new_doors = model.openings.iter().filter(|(id, opening)| matches!(opening.kind, OpeningKind::Door { .. }) && crate::standards::v1::subsets::any::schema::inferences::schedules::rows::storey_of(&model, id).is_some_and(|storey| storey == "st-ground")).count();
    assert!(ground_new_doors > 0 && items("sch-doors-ground") <= ground_new_doors, "the scoped door schedule lists the new doors of the ground floor only: {} of {ground_new_doors}", items("sch-doors-ground"));
    assert!(model.schedules["sch-doors-ground"].storeys == ["st-ground"], "its scope is authored");
}

#[semio_framework_async_macros::async_test]
async fn the_house_round_trips_and_validates() {`]], "the_house_schedules_list_every_door");

//! 🧰️ The fixtures the schedule tests share: the committed IFC house (seven walls on two storeys in three phases, three windows, two doors, a void, two rooms) and the helpers that build a schedule over it.

use crate::schedule_kit::preset;
use crate::standards::v1::subsets::any::io::text::snapshot::decode_model_snapshot_json;
use crate::standards::v1::subsets::any::schema::inferences::schedules::{ScheduleRow, ScheduleTable};
use crate::{ModelInference, ModelSnapshot, Schedule, ScheduleCategory, ScheduleColumn, ScheduleField, ScheduleKey};
use protocol::Inference;

const HOUSE: &str = include_str!("../../../../../🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot/🔣️.json");

/// 🏠️ The committed house without schedules.
pub fn house() -> ModelSnapshot {
    decode_model_snapshot_json(HOUSE).expect("the committed house decodes")
}

/// 🏠️ The house with every preset of the library as a schedule `sch-<key>`.
pub fn scheduled() -> ModelSnapshot {
    let mut snapshot = house();
    for key in crate::schedule_kit::PRESETS {
        snapshot.schedules.insert(format!("sch-{key}"), preset(key, key).expect("the preset exists"));
    }
    snapshot
}

/// 🏗️ A column of a built-in field.
pub fn column(field: ScheduleField, total: bool) -> ScheduleColumn {
    ScheduleColumn { key: ScheduleKey::field(field), heading: None, total }
}

/// 🏗️ A plain schedule of `category` listing `columns`: every element, no sort, no filter, no grouping, itemized.
pub fn plain(category: ScheduleCategory, columns: &[(ScheduleField, bool)]) -> Schedule {
    Schedule { name: "Test".into(), category, columns: columns.iter().map(|(field, total)| column(*field, *total)).collect(), sort: Vec::new(), filter: Vec::new(), group: Vec::new(), itemize: true, storeys: Vec::new(), phases: Vec::new() }
}

/// 📋️ The table of `schedule` over `snapshot`, inferred through the model graph.
pub fn table_of(snapshot: &ModelSnapshot, schedule: Schedule) -> ScheduleTable {
    let mut model = snapshot.clone();
    model.schedules.insert("sch-test".into(), schedule);
    ModelInference::infer(&model).expect("infers").schedules.remove("sch-test").expect("the table of the schedule")
}

/// 🧾️ The item rows of a table.
pub fn items(table: &ScheduleTable) -> Vec<&ScheduleRow> {
    table.rows.iter().filter(|row| row.kind == crate::standards::v1::subsets::any::schema::inferences::schedules::RowKind::Item).collect()
}

/// 🔤️ The text shown in column `index` of every item row.
pub fn texts(table: &ScheduleTable, index: usize) -> Vec<String> {
    items(table).iter().map(|row| row.cells[index].display()).collect()
}

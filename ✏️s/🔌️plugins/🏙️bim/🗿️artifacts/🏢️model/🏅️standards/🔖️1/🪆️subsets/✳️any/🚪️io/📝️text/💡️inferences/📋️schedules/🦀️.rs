//! 📊️ Native JSON inference table projection.
use crate::standards::v1::subsets::any::schema::inferences::schedules::{ScheduleCell, ScheduleRow, ScheduleTable};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

/// 🧾️ The table the third-party oracle reproduces: per schedule id its source row count and its rows as `{kind, level, elements, cells}`, a cell being `null`, a text or a number.
pub fn table_json(tables: &BTreeMap<String, ScheduleTable>) -> String {
    let cell = |cell: &ScheduleCell| match cell {
        ScheduleCell::Empty => DslValue::Null,
        ScheduleCell::Text { value } => DslValue::String(value.clone()),
        ScheduleCell::Number { value } => DslValue::Number(semio_framework_value::Number::Float(*value)),
    };
    let row = |row: &ScheduleRow| {
        DslValue::object([
            ("kind".to_string(), DslValue::String(format!("{:?}", row.kind))),
            ("level".to_string(), DslValue::Number(semio_framework_value::Number::UInt(u64::from(row.level)))),
            ("elements".to_string(), DslValue::Array(row.elements.iter().cloned().map(DslValue::String).collect())),
            ("cells".to_string(), DslValue::Array(row.cells.iter().map(cell).collect())),
        ])
    };
    let table = |table: &ScheduleTable| DslValue::object([("items".to_string(), DslValue::Number(semio_framework_value::Number::UInt(u64::from(table.items)))), ("rows".to_string(), DslValue::Array(table.rows.iter().map(row).collect()))]);
    semio_framework_pack_json::to_json_string(&DslValue::object(tables.iter().map(|(id, found)| (id.clone(), table(found)))))
}

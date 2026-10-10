//! 📋️ `schedules`: the user-defined schedules of the model. A [`crate::Schedule`] is authored (category, columns, sort, filter, grouping, storey and phase scope); its table is inferred: one row per
//! element (or, for the material category, per layer or material run of each element; for the finish category, per finished surface of each room, read from the `finishes` of its quantities) read from the `Quantity` nodes of the model graph and from the authored fields, filtered, sorted, grouped and
//! summed. No row is ever stored, so a quantity edit, a rename or a property edit reaches the table through the inference gate alone.
//!
//! The submodules: [`rows`] reads the authored facts of the elements and turns one source row into cells, [`table`] filters, sorts, groups and totals. The vocabulary of keys, the validity of a definition and the
//! presets of the library are authored values and live with the snapshot (`crate::schedule_kit`).

use super::super::quantities::ElementQuantity;
use crate::{ModelSnapshot, Schedule, ScheduleKey};
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

#[path = "🗂️rows/🦀️.rs"]
pub mod rows;
#[path = "🧮️table/🦀️.rs"]
pub mod table;

pub use rows::{AuthoredView, Facts};

/// 🗺️ The snapshot collections the schedules read, directly or through the quantities they are computed from.
pub const READS: &[&str] = &["schedules", "properties", "walls", "wall_types", "curtain_walls", "curtain_wall_types", "curtain_panel_overrides", "slabs", "slab_types", "ceilings", "roofs", "roof_types", "columns", "column_types", "beams", "beam_types", "openings", "window_types", "door_types", "stairs", "railings", "spaces", "materials", "storeys", "buildings", "sites"];

//#region 🔖️Values
/// 🧾️ One cell of a schedule table: nothing, a text (a stable token for enumerations, localized only when shown) or a number in the unit of its field (metres, square metres, cubic metres, kilograms, a count).
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub enum ScheduleCell {
    #[default]
    Empty,
    Text {
        value: String,
    },
    Number {
        value: f64,
    },
}

impl ScheduleCell {
    /// 🏗️ A text cell, empty for the empty text.
    pub fn text(value: impl Into<String>) -> Self {
        let value = value.into();
        if value.is_empty() {
            Self::Empty
        } else {
            Self::Text { value }
        }
    }

    /// 🔢️ A number cell.
    pub fn number(value: f64) -> Self {
        Self::Number { value }
    }

    /// 🔤️ The text a viewer reads: the text itself, a number in shortest round-trip form, nothing for an empty cell.
    pub fn display(&self) -> String {
        match self {
            Self::Empty => String::new(),
            Self::Text { value } => value.clone(),
            Self::Number { value } => format!("{value}"),
        }
    }
}

/// 🧩️ What a row of a table is: one element (or layer), the subtotal or collapsed row of a group, or the grand total.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum RowKind {
    #[default]
    Item,
    Group,
    Total,
}

/// 🧾️ One row of a table: its kind, its nesting depth (the grouping level a group row closes; the number of grouping levels for an item), the elements a pick selects and one cell per column.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ScheduleRow {
    pub kind: RowKind,
    pub level: u32,
    pub elements: Vec<String>,
    pub cells: Vec<ScheduleCell>,
}

/// 📋️ The table of one schedule: its column keys, its rows and the number of source rows that passed the scope and the filters.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ScheduleTable {
    pub keys: Vec<ScheduleKey>,
    pub rows: Vec<ScheduleRow>,
    pub items: u32,
}
//#endregion 🔖️Values

//#region 🔖️Inference
/// 📋️ The table of `schedule` from the quantities of its candidate elements (by element id) and the authored facts it reads. Pure: the snapshot is read only through `view`.
pub fn schedule_of(schedule: &Schedule, view: &AuthoredView, quantities: &BTreeMap<&str, &ElementQuantity>) -> ScheduleTable {
    table::table_of(schedule, view, quantities)
}

/// 🔑️ What the table of schedule `id` reads of the snapshot besides its parents: the definition, the candidate element ids and their authored facts.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(schedule) = snapshot.schedules.get(id) else { return DslValue::Null };
    let ids = rows::candidates(snapshot, schedule);
    let view = AuthoredView::of(snapshot, schedule, &ids);
    DslValue::object([("schedule".to_string(), semio_framework_value::ToValue::to_value(schedule)), ("ids".to_string(), semio_framework_value::ToValue::to_value(&ids)), ("view".to_string(), semio_framework_value::ToValue::to_value(&view))])
}
//#endregion 🔖️Inference

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🧰️kit/🦀️.rs"]
mod kit;
#[cfg(test)]
#[path = "🧪️tests/🗂️rows/🦀️.rs"]
mod tests_rows;
#[cfg(test)]
#[path = "🧪️tests/🧮️table/🦀️.rs"]
mod tests_table;
#[cfg(test)]
#[path = "🧪️tests/🕸️graph/🦀️.rs"]
mod tests_graph;

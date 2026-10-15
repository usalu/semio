//! 📊️ `create-schedule` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds/🦀️.rs"]
mod tests_adds;
#[cfg(test)]
#[path = "🧪️tests/🚪️adds-a-scoped-door-schedule/🦀️.rs"]
mod tests_adds_a_scoped_door_schedule;
#[cfg(test)]
#[path = "🧪️tests/🧱️adds-a-material-take-off/🦀️.rs"]
mod tests_adds_a_material_take_off;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/📛️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🕳️no-columns/🦀️.rs"]
mod tests_no_columns;
#[cfg(test)]
#[path = "🧪️tests/🧲️field-not-offered/🦀️.rs"]
mod tests_field_not_offered;
#[cfg(test)]
#[path = "🧪️tests/♻️repeated-column/🦀️.rs"]
mod tests_repeated_column;
#[cfg(test)]
#[path = "🧪️tests/🔎️filter-without-a-value/🦀️.rs"]
mod tests_filter_without_a_value;
#[cfg(test)]
#[path = "🧪️tests/🏢️storey-missing/🦀️.rs"]
mod tests_storey_missing;
//#endregion 🔖️Leaves

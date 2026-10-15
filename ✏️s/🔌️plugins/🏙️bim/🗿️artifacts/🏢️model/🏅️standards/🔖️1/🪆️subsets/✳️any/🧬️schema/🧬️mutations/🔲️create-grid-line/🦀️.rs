//! 🔲️ `create-grid-line` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-axis-a/🦀️.rs"]
mod tests_adds_axis_a;
#[cfg(test)]
#[path = "🧪️tests/🧲️same-label-other-building/🦀️.rs"]
mod tests_same_label_other_building;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/⛔️building-missing/🦀️.rs"]
mod tests_building_missing;
#[cfg(test)]
#[path = "🧪️tests/💥️zero-length/🦀️.rs"]
mod tests_zero_length;
#[cfg(test)]
#[path = "🧪️tests/🔒️label-taken/🦀️.rs"]
mod tests_label_taken;
#[cfg(test)]
#[path = "🧪️tests/🔁️blank-label/🦀️.rs"]
mod tests_blank_label;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves

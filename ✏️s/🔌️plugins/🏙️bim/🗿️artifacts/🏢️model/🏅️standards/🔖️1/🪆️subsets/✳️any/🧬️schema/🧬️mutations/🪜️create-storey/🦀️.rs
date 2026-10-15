//! 🪜️ `create-storey` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️stacks-above/🦀️.rs"]
mod tests_stacks_above;
#[cfg(test)]
#[path = "🧪️tests/🚫️level-taken/🦀️.rs"]
mod tests_level_taken;
#[cfg(test)]
#[path = "🧪️tests/⛔️building-missing/🦀️.rs"]
mod tests_building_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️with-a-plan-cut-height/🦀️.rs"]
mod tests_with_a_plan_cut_height;
#[cfg(test)]
#[path = "🧪️tests/🧯️non-positive-plan-cut-height/🦀️.rs"]
mod tests_non_positive_plan_cut_height;
//#endregion 🔖️Leaves

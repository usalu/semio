//! 🗼️ `create-column-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-round-column/🦀️.rs"]
mod tests_adds_a_round_column;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-an-l-shaped-outline/🦀️.rs"]
mod tests_adds_an_l_shaped_outline;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️non-positive-profile/🦀️.rs"]
mod tests_non_positive_profile;
#[cfg(test)]
#[path = "🧪️tests/❌️clockwise-outline/🦀️.rs"]
mod tests_clockwise_outline;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves

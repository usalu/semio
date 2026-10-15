//! 🌉️ `create-beam-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-an-i-shape/🦀️.rs"]
mod tests_adds_an_i_shape;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-a-bulged-outline/🦀️.rs"]
mod tests_adds_a_bulged_outline;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️oversized-flanges/🦀️.rs"]
mod tests_oversized_flanges;
#[cfg(test)]
#[path = "🧪️tests/❌️self-crossing-outline/🦀️.rs"]
mod tests_self_crossing_outline;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves

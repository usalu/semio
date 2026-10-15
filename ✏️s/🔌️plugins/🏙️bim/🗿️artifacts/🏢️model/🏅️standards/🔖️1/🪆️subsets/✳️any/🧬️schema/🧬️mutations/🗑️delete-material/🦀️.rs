//! 🗑️ `delete-material` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️removes/🦀️.rs"]
mod tests_removes;
#[cfg(test)]
#[path = "🧪️tests/🚫️used-by-a-layer/🦀️.rs"]
mod tests_used_by_a_layer;
#[cfg(test)]
#[path = "🧪️tests/🏛️used-by-a-column-type/🦀️.rs"]
mod tests_used_by_a_column_type;
#[cfg(test)]
#[path = "🧪️tests/🛤️used-by-a-railing/🦀️.rs"]
mod tests_used_by_a_railing;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🛋️used-by-a-space-finish/🦀️.rs"]
mod tests_used_by_a_space_finish;
#[cfg(test)]
#[path = "🧪️tests/🧭️used-by-a-ramp/🦀️.rs"]
mod tests_used_by_a_ramp;
//#endregion 🔖️Leaves

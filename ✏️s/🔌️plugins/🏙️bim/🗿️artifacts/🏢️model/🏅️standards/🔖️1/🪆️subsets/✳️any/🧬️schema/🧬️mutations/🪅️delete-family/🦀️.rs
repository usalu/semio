//! 🪅️ `delete-family` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️removes-with-its-parts/🦀️.rs"]
mod tests_removes_with_its_parts;
#[cfg(test)]
#[path = "🧪️tests/➕️removes-an-empty-family/🦀️.rs"]
mod tests_removes_an_empty_family;
#[cfg(test)]
#[path = "🧪️tests/🚫️used-by-a-column-type/🦀️.rs"]
mod tests_used_by_a_column_type;
#[cfg(test)]
#[path = "🧪️tests/⛔️used-by-a-beam-type/🦀️.rs"]
mod tests_used_by_a_beam_type;
#[cfg(test)]
#[path = "🧪️tests/❌️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

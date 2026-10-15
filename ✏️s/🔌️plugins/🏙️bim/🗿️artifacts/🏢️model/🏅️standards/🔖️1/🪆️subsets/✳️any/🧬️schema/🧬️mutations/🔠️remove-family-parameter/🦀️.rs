//! 🔠️ `remove-family-parameter` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️removes-an-unused-parameter/🦀️.rs"]
mod tests_removes_an_unused_parameter;
#[cfg(test)]
#[path = "🧪️tests/🚫️used-by-a-formula/🦀️.rs"]
mod tests_used_by_a_formula;
#[cfg(test)]
#[path = "🧪️tests/⛔️used-by-a-solid/🦀️.rs"]
mod tests_used_by_a_solid;
#[cfg(test)]
#[path = "🧪️tests/❌️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️family-missing/🦀️.rs"]
mod tests_family_missing;
//#endregion 🔖️Leaves

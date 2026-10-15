//! 🥶️ `remove-space-conditions` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️removes-the-record/🦀️.rs"]
mod tests_removes_the_record;
#[cfg(test)]
#[path = "🧪️tests/🧲️removes-a-sparse-record/🦀️.rs"]
mod tests_removes_a_sparse_record;
#[cfg(test)]
#[path = "🧪️tests/🧵️space-without-conditions/🦀️.rs"]
mod tests_space_without_conditions;
#[cfg(test)]
#[path = "🧪️tests/🧴️space-missing/🦀️.rs"]
mod tests_space_missing;
//#endregion 🔖️Leaves

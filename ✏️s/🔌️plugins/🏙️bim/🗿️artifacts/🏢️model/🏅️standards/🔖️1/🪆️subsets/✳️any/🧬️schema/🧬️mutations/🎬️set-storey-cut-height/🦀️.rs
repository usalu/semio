//! 🎬️ `set-storey-cut-height` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️sets-the-cut/🦀️.rs"]
mod tests_sets_the_cut;
#[cfg(test)]
#[path = "🧪️tests/➕️clears-the-cut/🦀️.rs"]
mod tests_clears_the_cut;
#[cfg(test)]
#[path = "🧪️tests/🚫️non-positive/🦀️.rs"]
mod tests_non_positive;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🔴️already-set/🦀️.rs"]
mod tests_already_set;
#[cfg(test)]
#[path = "🧪️tests/🟢️already-default/🦀️.rs"]
mod tests_already_default;
//#endregion 🔖️Leaves

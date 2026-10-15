//! 📏️ `set-storey-height` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️raises-the-ground-storey/🦀️.rs"]
mod tests_raises_the_ground_storey;
#[cfg(test)]
#[path = "🧪️tests/🚫️non-positive/🦀️.rs"]
mod tests_non_positive;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

//! 📒️ `delete-sheet` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️cascades-viewports-and-revisions/🦀️.rs"]
mod tests_cascades_viewports_and_revisions;
#[cfg(test)]
#[path = "🧪️tests/🧲️removes-its-data/🦀️.rs"]
mod tests_removes_its_data;
#[cfg(test)]
#[path = "🧪️tests/🧵️removes-an-empty-sheet/🦀️.rs"]
mod tests_removes_an_empty_sheet;
#[cfg(test)]
#[path = "🧪️tests/🧴️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

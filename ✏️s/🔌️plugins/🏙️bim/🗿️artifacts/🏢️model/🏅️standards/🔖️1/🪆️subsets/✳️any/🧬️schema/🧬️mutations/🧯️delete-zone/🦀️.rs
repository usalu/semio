//! 🧯️ `delete-zone` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️clears-the-memberships/🦀️.rs"]
mod tests_clears_the_memberships;
#[cfg(test)]
#[path = "🧪️tests/🗂️removes-its-data/🦀️.rs"]
mod tests_removes_its_data;
#[cfg(test)]
#[path = "🧪️tests/🛖️empty-zone/🦀️.rs"]
mod tests_empty_zone;
#[cfg(test)]
#[path = "🧪️tests/🗃️counted-by-an-area-scheme/🦀️.rs"]
mod tests_counted_by_an_area_scheme;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

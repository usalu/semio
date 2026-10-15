//! 💭️ `delete-issue-comment` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🧲️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

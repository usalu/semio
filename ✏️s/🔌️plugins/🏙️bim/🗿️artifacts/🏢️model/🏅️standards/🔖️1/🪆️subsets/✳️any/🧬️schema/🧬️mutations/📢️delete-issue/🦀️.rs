//! 📢️ `delete-issue` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️cascades-its-comments/🦀️.rs"]
mod tests_cascades_its_comments;
#[cfg(test)]
#[path = "🧪️tests/🧲️removes-an-issue-without-comments/🦀️.rs"]
mod tests_removes_an_issue_without_comments;
#[cfg(test)]
#[path = "🧪️tests/🧵️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

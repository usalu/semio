//! 🧺️ `delete-annotation-style` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🚫️used-by-dimensions/🦀️.rs"]
mod tests_used_by_dimensions;
#[cfg(test)]
#[path = "🧪️tests/⛔️used-by-tags/🦀️.rs"]
mod tests_used_by_tags;
#[cfg(test)]
#[path = "🧪️tests/❌️used-by-text-notes/🦀️.rs"]
mod tests_used_by_text_notes;
#[cfg(test)]
#[path = "🧪️tests/🛑️used-by-leaders/🦀️.rs"]
mod tests_used_by_leaders;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

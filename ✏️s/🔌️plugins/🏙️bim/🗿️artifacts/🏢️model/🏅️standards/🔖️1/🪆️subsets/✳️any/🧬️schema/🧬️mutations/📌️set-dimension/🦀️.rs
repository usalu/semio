//! 📌️ `set-dimension` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves-the-dimension-line/🦀️.rs"]
mod tests_moves_the_dimension_line;
#[cfg(test)]
#[path = "🧪️tests/➕️re-anchors/🦀️.rs"]
mod tests_re_anchors;
#[cfg(test)]
#[path = "🧪️tests/✨️locks-the-value/🦀️.rs"]
mod tests_locks_the_value;
#[cfg(test)]
#[path = "🧪️tests/👍️removes-the-lock/🦀️.rs"]
mod tests_removes_the_lock;
#[cfg(test)]
#[path = "🧪️tests/🧲️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️style-missing/🦀️.rs"]
mod tests_style_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️anchor-missing/🦀️.rs"]
mod tests_anchor_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️one-anchor/🦀️.rs"]
mod tests_one_anchor;
#[cfg(test)]
#[path = "🧪️tests/🙅️lock-not-positive/🦀️.rs"]
mod tests_lock_not_positive;
#[cfg(test)]
#[path = "🧪️tests/📛️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

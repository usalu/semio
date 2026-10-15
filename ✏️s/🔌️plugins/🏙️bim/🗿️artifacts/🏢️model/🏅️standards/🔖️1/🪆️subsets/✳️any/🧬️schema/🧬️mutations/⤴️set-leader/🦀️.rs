//! ⤴️ `set-leader` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️rewrites/🦀️.rs"]
mod tests_rewrites;
#[cfg(test)]
#[path = "🧪️tests/➕️re-anchors/🦀️.rs"]
mod tests_re_anchors;
#[cfg(test)]
#[path = "🧪️tests/✨️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️anchor-missing/🦀️.rs"]
mod tests_anchor_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️blank/🦀️.rs"]
mod tests_blank;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

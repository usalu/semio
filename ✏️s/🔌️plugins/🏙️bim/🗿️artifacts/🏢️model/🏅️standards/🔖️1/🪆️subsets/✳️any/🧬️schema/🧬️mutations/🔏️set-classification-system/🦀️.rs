//! 🔏️ `set-classification-system` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames-and-reeditions/🦀️.rs"]
mod tests_renames_and_reeditions;
#[cfg(test)]
#[path = "🧪️tests/🚫️replaces-the-entries/🦀️.rs"]
mod tests_replaces_the_entries;
#[cfg(test)]
#[path = "🧪️tests/⛔️sets-the-source/🦀️.rs"]
mod tests_sets_the_source;
#[cfg(test)]
#[path = "🧪️tests/🛑️clears-the-source/🦀️.rs"]
mod tests_clears_the_source;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/💤️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/📝️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🎯️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/📉️parent-missing/🦀️.rs"]
mod tests_parent_missing;
//#endregion 🔖️Leaves

//! 🛠️ `set-property-template` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames-and-retargets/🦀️.rs"]
mod tests_renames_and_retargets;
#[cfg(test)]
#[path = "🧪️tests/🚫️replaces-the-definitions/🦀️.rs"]
mod tests_replaces_the_definitions;
#[cfg(test)]
#[path = "🧪️tests/⛔️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🛑️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/🧭️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/💤️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/📝️name-already-defined/🦀️.rs"]
mod tests_name_already_defined;
#[cfg(test)]
#[path = "🧪️tests/🎯️kind-listed-twice/🦀️.rs"]
mod tests_kind_listed_twice;
#[cfg(test)]
#[path = "🧪️tests/📉️broken-definition/🦀️.rs"]
mod tests_broken_definition;
//#endregion 🔖️Leaves

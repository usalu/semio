//! 🗳️ `set-area-scheme` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️retargets-the-rule/🦀️.rs"]
mod tests_retargets_the_rule;
#[cfg(test)]
#[path = "🧪️tests/📐️changes-the-measure/🦀️.rs"]
mod tests_changes_the_measure;
#[cfg(test)]
#[path = "🧪️tests/♾️counts-everything/🦀️.rs"]
mod tests_counts_everything;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/💤️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🏘️zone-missing/🦀️.rs"]
mod tests_zone_missing;
#[cfg(test)]
#[path = "🧪️tests/📝️blank-usage/🦀️.rs"]
mod tests_blank_usage;
//#endregion 🔖️Leaves

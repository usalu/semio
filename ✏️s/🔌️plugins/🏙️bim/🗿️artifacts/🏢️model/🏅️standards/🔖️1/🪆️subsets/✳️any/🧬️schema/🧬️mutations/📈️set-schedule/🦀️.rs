//! 📈️ `set-schedule` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames/🦀️.rs"]
mod tests_renames;
#[cfg(test)]
#[path = "🧪️tests/📐️replaces-the-columns/🦀️.rs"]
mod tests_replaces_the_columns;
#[cfg(test)]
#[path = "🧪️tests/🔎️sorts-and-filters/🦀️.rs"]
mod tests_sorts_and_filters;
#[cfg(test)]
#[path = "🧪️tests/🗂️groups-and-collapses/🦀️.rs"]
mod tests_groups_and_collapses;
#[cfg(test)]
#[path = "🧪️tests/🏢️scopes-the-storeys-and-phases/🦀️.rs"]
mod tests_scopes_the_storeys_and_phases;
#[cfg(test)]
#[path = "🧪️tests/🚪️changes-the-category/🦀️.rs"]
mod tests_changes_the_category;
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
#[path = "🧪️tests/🧲️category-drops-a-field/🦀️.rs"]
mod tests_category_drops_a_field;
#[cfg(test)]
#[path = "🧪️tests/🕳️no-columns/🦀️.rs"]
mod tests_no_columns;
#[cfg(test)]
#[path = "🧪️tests/📛️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🏘️storey-missing/🦀️.rs"]
mod tests_storey_missing;
//#endregion 🔖️Leaves

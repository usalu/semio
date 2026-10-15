//! 💨️ `set-mep-element` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️resizes/🦀️.rs"]
mod tests_resizes;
#[cfg(test)]
#[path = "🧪️tests/➕️changes-the-section-kind/🦀️.rs"]
mod tests_changes_the_section_kind;
#[cfg(test)]
#[path = "🧪️tests/✨️reroutes/🦀️.rs"]
mod tests_reroutes;
#[cfg(test)]
#[path = "🧪️tests/👍️changes-the-system/🦀️.rs"]
mod tests_changes_the_system;
#[cfg(test)]
#[path = "🧪️tests/🧲️renames/🦀️.rs"]
mod tests_renames;
#[cfg(test)]
#[path = "🧪️tests/💪️moves-to-another-storey/🦀️.rs"]
mod tests_moves_to_another_storey;
#[cfg(test)]
#[path = "🧪️tests/🎉️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️names-no-field/🦀️.rs"]
mod tests_names_no_field;
#[cfg(test)]
#[path = "🧪️tests/❌️section-not-positive/🦀️.rs"]
mod tests_section_not_positive;
#[cfg(test)]
#[path = "🧪️tests/🛑️path-too-short/🦀️.rs"]
mod tests_path_too_short;
#[cfg(test)]
#[path = "🧪️tests/🚷️repeated-point/🦀️.rs"]
mod tests_repeated_point;
#[cfg(test)]
#[path = "🧪️tests/🙅️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/📛️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

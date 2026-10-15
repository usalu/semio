//! 🏴️ `set-tag` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️reads-the-type/🦀️.rs"]
mod tests_reads_the_type;
#[cfg(test)]
#[path = "🧪️tests/➕️retargets-and-moves/🦀️.rs"]
mod tests_retargets_and_moves;
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
#[path = "🧪️tests/❌️element-missing/🦀️.rs"]
mod tests_element_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️style-missing/🦀️.rs"]
mod tests_style_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

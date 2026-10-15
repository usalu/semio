//! 🏤️ `set-curtain-wall-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/➕️regrids/🦀️.rs"]
mod tests_regrids;
#[cfg(test)]
#[path = "🧪️tests/✨️swaps-the-mullions/🦀️.rs"]
mod tests_swaps_the_mullions;
#[cfg(test)]
#[path = "🧪️tests/👍️changes-the-default-panel/🦀️.rs"]
mod tests_changes_the_default_panel;
#[cfg(test)]
#[path = "🧪️tests/🧲️swaps-the-materials/🦀️.rs"]
mod tests_swaps_the_materials;
#[cfg(test)]
#[path = "🧪️tests/🌟️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🚫️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/⛔️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/❌️spacing-non-positive/🦀️.rs"]
mod tests_spacing_non_positive;
#[cfg(test)]
#[path = "🧪️tests/🛑️lines-not-ascending/🦀️.rs"]
mod tests_lines_not_ascending;
#[cfg(test)]
#[path = "🧪️tests/🚷️window-type-missing/🦀️.rs"]
mod tests_window_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🙅️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/📛️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

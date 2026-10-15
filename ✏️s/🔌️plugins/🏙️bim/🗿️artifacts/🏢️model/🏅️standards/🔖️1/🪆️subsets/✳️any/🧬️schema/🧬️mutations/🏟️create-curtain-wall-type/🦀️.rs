//! 🏟️ `create-curtain-wall-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-uniform-grid/🦀️.rs"]
mod tests_adds_a_uniform_grid;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-explicit-lines-and-a-solid-panel/🦀️.rs"]
mod tests_adds_explicit_lines_and_a_solid_panel;
#[cfg(test)]
#[path = "🧪️tests/✨️adds-a-door-default/🦀️.rs"]
mod tests_adds_a_door_default;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️spacing-non-positive/🦀️.rs"]
mod tests_spacing_non_positive;
#[cfg(test)]
#[path = "🧪️tests/🛑️lines-not-ascending/🦀️.rs"]
mod tests_lines_not_ascending;
#[cfg(test)]
#[path = "🧪️tests/🚷️mullion-flat/🦀️.rs"]
mod tests_mullion_flat;
#[cfg(test)]
#[path = "🧪️tests/🙅️door-type-missing/🦀️.rs"]
mod tests_door_type_missing;
#[cfg(test)]
#[path = "🧪️tests/📛️material-missing/🦀️.rs"]
mod tests_material_missing;
//#endregion 🔖️Leaves

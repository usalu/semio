//! 🏩️ `set-curtain-panel-override` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️swaps-the-panel/🦀️.rs"]
mod tests_swaps_the_panel;
#[cfg(test)]
#[path = "🧪️tests/➕️empties-the-cell/🦀️.rs"]
mod tests_empties_the_cell;
#[cfg(test)]
#[path = "🧪️tests/✨️another-door-type/🦀️.rs"]
mod tests_another_door_type;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️door-type-missing/🦀️.rs"]
mod tests_door_type_missing;
#[cfg(test)]
#[path = "🧪️tests/❌️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

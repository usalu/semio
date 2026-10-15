//! 🏨️ `create-curtain-panel-override` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-door/🦀️.rs"]
mod tests_adds_a_door;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-a-window/🦀️.rs"]
mod tests_adds_a_window;
#[cfg(test)]
#[path = "🧪️tests/✨️adds-a-solid-panel/🦀️.rs"]
mod tests_adds_a_solid_panel;
#[cfg(test)]
#[path = "🧪️tests/👍️adds-an-empty-cell/🦀️.rs"]
mod tests_adds_an_empty_cell;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️cell-taken/🦀️.rs"]
mod tests_cell_taken;
#[cfg(test)]
#[path = "🧪️tests/🛑️curtain-wall-missing/🦀️.rs"]
mod tests_curtain_wall_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️door-type-missing/🦀️.rs"]
mod tests_door_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🙅️window-type-missing/🦀️.rs"]
mod tests_window_type_missing;
#[cfg(test)]
#[path = "🧪️tests/📛️material-missing/🦀️.rs"]
mod tests_material_missing;
//#endregion 🔖️Leaves

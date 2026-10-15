//! 🖼️ `create-window-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds/🦀️.rs"]
mod tests_adds;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-a-floor-level-window/🦀️.rs"]
mod tests_adds_a_floor_level_window;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️non-positive-width/🦀️.rs"]
mod tests_non_positive_width;
#[cfg(test)]
#[path = "🧪️tests/❌️negative-sill/🦀️.rs"]
mod tests_negative_sill;
#[cfg(test)]
#[path = "🧪️tests/📛️no-panes/🦀️.rs"]
mod tests_no_panes;
#[cfg(test)]
#[path = "🧪️tests/🚷️non-positive-frame/🦀️.rs"]
mod tests_non_positive_frame;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves

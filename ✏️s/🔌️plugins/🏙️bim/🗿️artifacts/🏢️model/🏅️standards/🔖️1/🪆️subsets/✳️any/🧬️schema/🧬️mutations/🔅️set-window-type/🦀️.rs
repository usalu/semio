//! 🔅️ `set-window-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️widens/🦀️.rs"]
mod tests_widens;
#[cfg(test)]
#[path = "🧪️tests/🧲️lowers-the-sill-and-adds-panes/🦀️.rs"]
mod tests_lowers_the_sill_and_adds_panes;
#[cfg(test)]
#[path = "🧪️tests/✨️renames-and-reframes/🦀️.rs"]
mod tests_renames_and_reframes;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
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
#[path = "🧪️tests/🚷️non-positive-frame-depth/🦀️.rs"]
mod tests_non_positive_frame_depth;
#[cfg(test)]
#[path = "🧪️tests/🚳️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves

//! 🔑️ `set-door-type` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🧲️swaps-leaves-and-swing/🦀️.rs"]
mod tests_swaps_leaves_and_swing;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/⛔️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️non-positive-height/🦀️.rs"]
mod tests_non_positive_height;
#[cfg(test)]
#[path = "🧪️tests/❌️non-positive-frame-width/🦀️.rs"]
mod tests_non_positive_frame_width;
#[cfg(test)]
#[path = "🧪️tests/📛️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves

//! 📔️ `set-viewport` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves/🦀️.rs"]
mod tests_moves;
#[cfg(test)]
#[path = "🧪️tests/🧲️rescales/🦀️.rs"]
mod tests_rescales;
#[cfg(test)]
#[path = "🧪️tests/🧵️shows-another-view/🦀️.rs"]
mod tests_shows_another_view;
#[cfg(test)]
#[path = "🧪️tests/🧴️moves-to-another-sheet/🦀️.rs"]
mod tests_moves_to_another_sheet;
#[cfg(test)]
#[path = "🧪️tests/🧳️crops/🦀️.rs"]
mod tests_crops;
#[cfg(test)]
#[path = "🧪️tests/🧱️clears-the-crop/🦀️.rs"]
mod tests_clears_the_crop;
#[cfg(test)]
#[path = "🧪️tests/🧰️labels/🦀️.rs"]
mod tests_labels;
#[cfg(test)]
#[path = "🧪️tests/🧯️clears-the-label/🦀️.rs"]
mod tests_clears_the_label;
#[cfg(test)]
#[path = "🧪️tests/🧮️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧭️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧩️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧨️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧧️sheet-missing/🦀️.rs"]
mod tests_sheet_missing;
#[cfg(test)]
#[path = "🧪️tests/🧦️view-missing/🦀️.rs"]
mod tests_view_missing;
#[cfg(test)]
#[path = "🧪️tests/🧥️camera-view/🦀️.rs"]
mod tests_camera_view;
#[cfg(test)]
#[path = "🧪️tests/🧤️scale-zero/🦀️.rs"]
mod tests_scale_zero;
#[cfg(test)]
#[path = "🧪️tests/🧣️scale-too-large/🦀️.rs"]
mod tests_scale_too_large;
#[cfg(test)]
#[path = "🧪️tests/🧢️empty-crop/🦀️.rs"]
mod tests_empty_crop;
#[cfg(test)]
#[path = "🧪️tests/🧡️blank-label/🦀️.rs"]
mod tests_blank_label;
//#endregion 🔖️Leaves

//! 📓️ `create-viewport` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️places-a-plan/🦀️.rs"]
mod tests_places_a_plan;
#[cfg(test)]
#[path = "🧪️tests/🧲️places-a-cropped-section/🦀️.rs"]
mod tests_places_a_cropped_section;
#[cfg(test)]
#[path = "🧪️tests/🧵️places-the-same-view-twice/🦀️.rs"]
mod tests_places_the_same_view_twice;
#[cfg(test)]
#[path = "🧪️tests/🧴️places-at-the-largest-scale/🦀️.rs"]
mod tests_places_at_the_largest_scale;
#[cfg(test)]
#[path = "🧪️tests/🧳️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧱️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧰️sheet-missing/🦀️.rs"]
mod tests_sheet_missing;
#[cfg(test)]
#[path = "🧪️tests/🧯️view-missing/🦀️.rs"]
mod tests_view_missing;
#[cfg(test)]
#[path = "🧪️tests/🧮️camera-view/🦀️.rs"]
mod tests_camera_view;
#[cfg(test)]
#[path = "🧪️tests/🧭️scale-zero/🦀️.rs"]
mod tests_scale_zero;
#[cfg(test)]
#[path = "🧪️tests/🧩️scale-too-large/🦀️.rs"]
mod tests_scale_too_large;
#[cfg(test)]
#[path = "🧪️tests/🧨️empty-crop/🦀️.rs"]
mod tests_empty_crop;
#[cfg(test)]
#[path = "🧪️tests/🧧️blank-label/🦀️.rs"]
mod tests_blank_label;
//#endregion 🔖️Leaves

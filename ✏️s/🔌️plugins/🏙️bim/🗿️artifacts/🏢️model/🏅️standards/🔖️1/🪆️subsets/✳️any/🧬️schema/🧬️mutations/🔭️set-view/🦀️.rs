//! 🔭️ `set-view` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

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
#[path = "🧪️tests/🧲️moves-the-plane/🦀️.rs"]
mod tests_moves_the_plane;
#[cfg(test)]
#[path = "🧪️tests/🧵️retargets-the-plan/🦀️.rs"]
mod tests_retargets_the_plan;
#[cfg(test)]
#[path = "🧪️tests/🧴️cuts-lower/🦀️.rs"]
mod tests_cuts_lower;
#[cfg(test)]
#[path = "🧪️tests/🧳️clears-the-cut/🦀️.rs"]
mod tests_clears_the_cut;
#[cfg(test)]
#[path = "🧪️tests/🧱️crops/🦀️.rs"]
mod tests_crops;
#[cfg(test)]
#[path = "🧪️tests/🧰️clears-the-crop/🦀️.rs"]
mod tests_clears_the_crop;
#[cfg(test)]
#[path = "🧪️tests/🧯️hides-categories/🦀️.rs"]
mod tests_hides_categories;
#[cfg(test)]
#[path = "🧪️tests/🧮️filters-by-phase/🦀️.rs"]
mod tests_filters_by_phase;
#[cfg(test)]
#[path = "🧪️tests/🧭️clears-the-phase/🦀️.rs"]
mod tests_clears_the_phase;
#[cfg(test)]
#[path = "🧪️tests/🧩️rescales/🦀️.rs"]
mod tests_rescales;
#[cfg(test)]
#[path = "🧪️tests/🧨️orbits/🦀️.rs"]
mod tests_orbits;
#[cfg(test)]
#[path = "🧪️tests/🧧️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🧦️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧥️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🧤️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/🧣️name-taken/🦀️.rs"]
mod tests_name_taken;
#[cfg(test)]
#[path = "🧪️tests/🧢️plane-on-a-plan/🦀️.rs"]
mod tests_plane_on_a_plan;
#[cfg(test)]
#[path = "🧪️tests/🧡️camera-on-a-section/🦀️.rs"]
mod tests_camera_on_a_section;
#[cfg(test)]
#[path = "🧪️tests/🧠️storey-on-a-section/🦀️.rs"]
mod tests_storey_on_a_section;
#[cfg(test)]
#[path = "🧪️tests/🧟️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧞️storey-of-another-building/🦀️.rs"]
mod tests_storey_of_another_building;
#[cfg(test)]
#[path = "🧪️tests/🧝️depth-zero/🦀️.rs"]
mod tests_depth_zero;
#[cfg(test)]
#[path = "🧪️tests/🧜️scale-too-large/🦀️.rs"]
mod tests_scale_too_large;
#[cfg(test)]
#[path = "🧪️tests/🧛️duplicate-hidden/🦀️.rs"]
mod tests_duplicate_hidden;
#[cfg(test)]
#[path = "🧪️tests/🧚️empty-crop/🦀️.rs"]
mod tests_empty_crop;
//#endregion 🔖️Leaves

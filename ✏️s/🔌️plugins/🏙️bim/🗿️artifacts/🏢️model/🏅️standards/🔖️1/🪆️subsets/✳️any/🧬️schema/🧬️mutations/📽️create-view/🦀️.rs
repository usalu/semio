//! 📽️ `create-view` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-plan/🦀️.rs"]
mod tests_adds_a_plan;
#[cfg(test)]
#[path = "🧪️tests/🧲️adds-a-ceiling-plan/🦀️.rs"]
mod tests_adds_a_ceiling_plan;
#[cfg(test)]
#[path = "🧪️tests/🧵️adds-a-section/🦀️.rs"]
mod tests_adds_a_section;
#[cfg(test)]
#[path = "🧪️tests/🧴️adds-an-elevation/🦀️.rs"]
mod tests_adds_an_elevation;
#[cfg(test)]
#[path = "🧪️tests/🧳️adds-a-camera/🦀️.rs"]
mod tests_adds_a_camera;
#[cfg(test)]
#[path = "🧪️tests/🧱️adds-a-configured-plan/🦀️.rs"]
mod tests_adds_a_configured_plan;
#[cfg(test)]
#[path = "🧪️tests/🧰️duplicate-id/🦀️.rs"]
mod tests_duplicate_id;
#[cfg(test)]
#[path = "🧪️tests/🧯️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️building-missing/🦀️.rs"]
mod tests_building_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧩️storey-of-another-building/🦀️.rs"]
mod tests_storey_of_another_building;
#[cfg(test)]
#[path = "🧪️tests/🧨️plan-without-storey/🦀️.rs"]
mod tests_plan_without_storey;
#[cfg(test)]
#[path = "🧪️tests/🧧️section-without-plane/🦀️.rs"]
mod tests_section_without_plane;
#[cfg(test)]
#[path = "🧪️tests/🧦️plane-without-length/🦀️.rs"]
mod tests_plane_without_length;
#[cfg(test)]
#[path = "🧪️tests/🧥️camera-without-camera/🦀️.rs"]
mod tests_camera_without_camera;
#[cfg(test)]
#[path = "🧪️tests/🧤️blank-name/🦀️.rs"]
mod tests_blank_name;
#[cfg(test)]
#[path = "🧪️tests/🧣️name-taken/🦀️.rs"]
mod tests_name_taken;
#[cfg(test)]
#[path = "🧪️tests/🧢️depth-zero/🦀️.rs"]
mod tests_depth_zero;
#[cfg(test)]
#[path = "🧪️tests/🧡️unordered-hidden/🦀️.rs"]
mod tests_unordered_hidden;
#[cfg(test)]
#[path = "🧪️tests/🧠️scale-zero/🦀️.rs"]
mod tests_scale_zero;
#[cfg(test)]
#[path = "🧪️tests/🧟️empty-crop/🦀️.rs"]
mod tests_empty_crop;
#[cfg(test)]
#[path = "🧪️tests/🧞️cut-height-on-a-section/🦀️.rs"]
mod tests_cut_height_on_a_section;
//#endregion 🔖️Leaves

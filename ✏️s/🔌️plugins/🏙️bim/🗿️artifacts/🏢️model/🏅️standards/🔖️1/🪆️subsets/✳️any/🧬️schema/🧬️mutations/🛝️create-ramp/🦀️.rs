//! 🛝️ `create-ramp` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-straight-ramp/🦀️.rs"]
mod tests_adds_a_straight_ramp;
#[cfg(test)]
#[path = "🧪️tests/📐️adds-a-bent-ramp-with-railings/🦀️.rs"]
mod tests_adds_a_bent_ramp_with_railings;
#[cfg(test)]
#[path = "🧪️tests/🌀️adds-a-curved-ramp-to-the-first-storey/🦀️.rs"]
mod tests_adds_a_curved_ramp_to_the_first_storey;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/⛔️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧱️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🔴️top-storey-missing/🦀️.rs"]
mod tests_top_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/📏️path-too-short/🦀️.rs"]
mod tests_path_too_short;
#[cfg(test)]
#[path = "🧪️tests/🛑️non-positive-width/🦀️.rs"]
mod tests_non_positive_width;
#[cfg(test)]
#[path = "🧪️tests/💥️non-positive-thickness/🦀️.rs"]
mod tests_non_positive_thickness;
#[cfg(test)]
#[path = "🧪️tests/📉️non-positive-slope-limit/🦀️.rs"]
mod tests_non_positive_slope_limit;
#[cfg(test)]
#[path = "🧪️tests/🚧️negative-landing/🦀️.rs"]
mod tests_negative_landing;
//#endregion 🔖️Leaves

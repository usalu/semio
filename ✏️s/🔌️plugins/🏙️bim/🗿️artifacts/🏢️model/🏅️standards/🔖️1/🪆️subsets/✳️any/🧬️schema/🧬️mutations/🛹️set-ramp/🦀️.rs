//! 🛹️ `set-ramp` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️reshapes/🦀️.rs"]
mod tests_reshapes;
#[cfg(test)]
#[path = "🧪️tests/🏷️renames-only/🦀️.rs"]
mod tests_renames_only;
#[cfg(test)]
#[path = "🧪️tests/🛤️carries-railings-on-both-sides/🦀️.rs"]
mod tests_carries_railings_on_both_sides;
#[cfg(test)]
#[path = "🧪️tests/⛰️steepens-beyond-the-limit/🦀️.rs"]
mod tests_steepens_beyond_the_limit;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/💤️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/🚫️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/📏️path-too-short/🦀️.rs"]
mod tests_path_too_short;
#[cfg(test)]
#[path = "🧪️tests/🛑️non-positive-width/🦀️.rs"]
mod tests_non_positive_width;
#[cfg(test)]
#[path = "🧪️tests/📉️non-positive-slope-limit/🦀️.rs"]
mod tests_non_positive_slope_limit;
#[cfg(test)]
#[path = "🧪️tests/🧱️material-missing/🦀️.rs"]
mod tests_material_missing;
#[cfg(test)]
#[path = "🧪️tests/🔴️top-storey-missing/🦀️.rs"]
mod tests_top_storey_missing;
//#endregion 🔖️Leaves

//! 🖌️ `set-material` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️recolours-and-renames/🦀️.rs"]
mod tests_recolours_and_renames;
#[cfg(test)]
#[path = "🧪️tests/🌡️retunes-the-physics/🦀️.rs"]
mod tests_retunes_the_physics;
#[cfg(test)]
#[path = "🧪️tests/🧲️recategorises/🦀️.rs"]
mod tests_recategorises;
#[cfg(test)]
#[path = "🧪️tests/📭️empty-patch/🦀️.rs"]
mod tests_empty_patch;
#[cfg(test)]
#[path = "🧪️tests/♻️same-values/🦀️.rs"]
mod tests_same_values;
#[cfg(test)]
#[path = "🧪️tests/🚫️negative-conductivity/🦀️.rs"]
mod tests_negative_conductivity;
#[cfg(test)]
#[path = "🧪️tests/🌈️colour-out-of-range/🦀️.rs"]
mod tests_colour_out_of_range;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
//#endregion 🔖️Leaves

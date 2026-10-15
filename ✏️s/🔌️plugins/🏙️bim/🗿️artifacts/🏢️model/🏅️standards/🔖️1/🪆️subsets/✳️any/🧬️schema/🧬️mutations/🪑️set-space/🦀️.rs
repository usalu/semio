//! 🪑️ `set-space` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames-and-retypes/🦀️.rs"]
mod tests_renames_and_retypes;
#[cfg(test)]
#[path = "🧪️tests/📐️redraws-the-boundary/🦀️.rs"]
mod tests_redraws_the_boundary;
#[cfg(test)]
#[path = "🧪️tests/🔢️renumbers/🦀️.rs"]
mod tests_renumbers;
#[cfg(test)]
#[path = "🧪️tests/💤️nothing-to-change/🦀️.rs"]
mod tests_nothing_to_change;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
#[cfg(test)]
#[path = "🧪️tests/🚫️number-taken/🦀️.rs"]
mod tests_number_taken;
#[cfg(test)]
#[path = "🧪️tests/💥️outline-degenerate/🦀️.rs"]
mod tests_outline_degenerate;
#[cfg(test)]
#[path = "🧪️tests/🧭️restates-an-unchanged-field/🦀️.rs"]
mod tests_restates_an_unchanged_field;
#[cfg(test)]
#[path = "🧪️tests/🏘️assigns-a-zone/🦀️.rs"]
mod tests_assigns_a_zone;
#[cfg(test)]
#[path = "🧪️tests/🚶️moves-to-another-zone/🦀️.rs"]
mod tests_moves_to_another_zone;
#[cfg(test)]
#[path = "🧪️tests/🚮️clears-the-zone/🦀️.rs"]
mod tests_clears_the_zone;
#[cfg(test)]
#[path = "🧪️tests/🎨️finishes-the-room/🦀️.rs"]
mod tests_finishes_the_room;
#[cfg(test)]
#[path = "🧪️tests/🧼️clears-a-finish/🦀️.rs"]
mod tests_clears_a_finish;
#[cfg(test)]
#[path = "🧪️tests/📌️zone-already-set/🦀️.rs"]
mod tests_zone_already_set;
#[cfg(test)]
#[path = "🧪️tests/🗺️zone-missing/🦀️.rs"]
mod tests_zone_missing;
#[cfg(test)]
#[path = "🧪️tests/🧱️finish-material-missing/🦀️.rs"]
mod tests_finish_material_missing;
//#endregion 🔖️Leaves

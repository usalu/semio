//! 💠️ `array-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️arrays-a-column-in-a-row/🦀️.rs"]
mod tests_arrays_a_column_in_a_row;
#[cfg(test)]
#[path = "🧪️tests/🌀️arrays-a-column-around-a-centre/🦀️.rs"]
mod tests_arrays_a_column_around_a_centre;
#[cfg(test)]
#[path = "🧪️tests/🔗️arrays-a-wall-with-its-openings/🦀️.rs"]
mod tests_arrays_a_wall_with_its_openings;
#[cfg(test)]
#[path = "🧪️tests/🛑️no-copies/🦀️.rs"]
mod tests_no_copies;
#[cfg(test)]
#[path = "🧪️tests/⛔️spacing-without-length/🦀️.rs"]
mod tests_spacing_without_length;
#[cfg(test)]
#[path = "🧪️tests/🚧️too-many-copies/🦀️.rs"]
mod tests_too_many_copies;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/🧩️minted-id-taken/🦀️.rs"]
mod tests_minted_id_taken;
//#endregion 🔖️Leaves

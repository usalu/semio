//! 🧶️ `offset-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️offsets-to-the-left/🦀️.rs"]
mod tests_offsets_to_the_left;
#[cfg(test)]
#[path = "🧪️tests/🌟️offsets-to-the-right/🦀️.rs"]
mod tests_offsets_to_the_right;
#[cfg(test)]
#[path = "🧪️tests/🌀️offsets-an-arc/🦀️.rs"]
mod tests_offsets_an_arc;
#[cfg(test)]
#[path = "🧪️tests/🚧️arc-collapses/🦀️.rs"]
mod tests_arc_collapses;
#[cfg(test)]
#[path = "🧪️tests/🛑️zero-distance/🦀️.rs"]
mod tests_zero_distance;
#[cfg(test)]
#[path = "🧪️tests/🧩️new-id-taken/🦀️.rs"]
mod tests_new_id_taken;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

//! 🍰️ `split-slab` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️splits-a-rectangle/🦀️.rs"]
mod tests_splits_a_rectangle;
#[cfg(test)]
#[path = "🧪️tests/🧱️sends-a-hole-with-its-piece/🦀️.rs"]
mod tests_sends_a_hole_with_its_piece;
#[cfg(test)]
#[path = "🧪️tests/🌀️splits-an-arc-outline/🦀️.rs"]
mod tests_splits_an_arc_outline;
#[cfg(test)]
#[path = "🧪️tests/🌟️splits-through-two-corners/🦀️.rs"]
mod tests_splits_through_two_corners;
#[cfg(test)]
#[path = "🧪️tests/🚫️misses-the-slab/🦀️.rs"]
mod tests_misses_the_slab;
#[cfg(test)]
#[path = "🧪️tests/🚧️crosses-a-hole/🦀️.rs"]
mod tests_crosses_a_hole;
#[cfg(test)]
#[path = "🧪️tests/⛔️cuts-more-than-twice/🦀️.rs"]
mod tests_cuts_more_than_twice;
#[cfg(test)]
#[path = "🧪️tests/🛑️line-without-length/🦀️.rs"]
mod tests_line_without_length;
#[cfg(test)]
#[path = "🧪️tests/🧩️new-id-taken/🦀️.rs"]
mod tests_new_id_taken;
#[cfg(test)]
#[path = "🧪️tests/🕳️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

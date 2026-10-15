//! ↔️ `create-dimension` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️dimensions-a-wall/🦀️.rs"]
mod tests_dimensions_a_wall;
#[cfg(test)]
#[path = "🧪️tests/➕️chains-an-opening-centre/🦀️.rs"]
mod tests_chains_an_opening_centre;
#[cfg(test)]
#[path = "🧪️tests/✨️thickness-between-faces/🦀️.rs"]
mod tests_thickness_between_faces;
#[cfg(test)]
#[path = "🧪️tests/👍️locks-a-free-span/🦀️.rs"]
mod tests_locks_a_free_span;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/❌️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️style-missing/🦀️.rs"]
mod tests_style_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️anchor-missing/🦀️.rs"]
mod tests_anchor_missing;
#[cfg(test)]
#[path = "🧪️tests/🙅️one-anchor/🦀️.rs"]
mod tests_one_anchor;
#[cfg(test)]
#[path = "🧪️tests/📛️lock-not-positive/🦀️.rs"]
mod tests_lock_not_positive;
//#endregion 🔖️Leaves

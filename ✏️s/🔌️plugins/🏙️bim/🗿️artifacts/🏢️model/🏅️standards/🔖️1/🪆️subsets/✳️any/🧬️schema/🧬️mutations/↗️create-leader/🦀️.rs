//! ↗️ `create-leader` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️points-at-a-wall-face/🦀️.rs"]
mod tests_points_at_a_wall_face;
#[cfg(test)]
#[path = "🧪️tests/➕️points-at-a-free-point/🦀️.rs"]
mod tests_points_at_a_free_point;
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
#[path = "🧪️tests/🛑️anchor-missing/🦀️.rs"]
mod tests_anchor_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️style-missing/🦀️.rs"]
mod tests_style_missing;
#[cfg(test)]
#[path = "🧪️tests/🙅️blank/🦀️.rs"]
mod tests_blank;
//#endregion 🔖️Leaves

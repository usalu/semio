//! ⬜️ `create-slab` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-slab-with-a-hole/🦀️.rs"]
mod tests_adds_a_slab_with_a_hole;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-a-sloped-curved-slab/🦀️.rs"]
mod tests_adds_a_sloped_curved_slab;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/❌️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️too-few-vertices/🦀️.rs"]
mod tests_too_few_vertices;
#[cfg(test)]
#[path = "🧪️tests/🚷️zero-area/🦀️.rs"]
mod tests_zero_area;
#[cfg(test)]
#[path = "🧪️tests/🙅️self-intersecting/🦀️.rs"]
mod tests_self_intersecting;
#[cfg(test)]
#[path = "🧪️tests/📛️clockwise/🦀️.rs"]
mod tests_clockwise;
#[cfg(test)]
#[path = "🧪️tests/🚧️hole-outside/🦀️.rs"]
mod tests_hole_outside;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves

//! 🧱️ `create-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-on-the-first-storey/🦀️.rs"]
mod tests_adds_on_the_first_storey;
#[cfg(test)]
#[path = "🧪️tests/🚫️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/⛔️zero-length/🦀️.rs"]
mod tests_zero_length;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/🧮️adds-a-wall-under-the-roof/🦀️.rs"]
mod tests_adds_a_wall_under_the_roof;
#[cfg(test)]
#[path = "🧪️tests/🧯️adds-a-wall-standing-on-a-slab/🦀️.rs"]
mod tests_adds_a_wall_standing_on_a_slab;
#[cfg(test)]
#[path = "🧪️tests/🧰️roof-missing/🦀️.rs"]
mod tests_roof_missing;
#[cfg(test)]
#[path = "🧪️tests/🧲️base-slab-missing/🦀️.rs"]
mod tests_base_slab_missing;
#[cfg(test)]
#[path = "🧪️tests/🧳️base-slab-in-another-building/🦀️.rs"]
mod tests_base_slab_in_another_building;
//#endregion 🔖️Leaves

//! ➖️ `create-beam` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-below-the-storey-top/🦀️.rs"]
mod tests_adds_below_the_storey_top;
#[cfg(test)]
#[path = "🧪️tests/🚫️duplicate/🦀️.rs"]
mod tests_duplicate;
#[cfg(test)]
#[path = "🧪️tests/⛔️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🧲️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🛑️zero-length/🦀️.rs"]
mod tests_zero_length;
#[cfg(test)]
#[path = "🧪️tests/📌️authored-top-offset-is-stored/🦀️.rs"]
mod tests_authored_top_offset_is_stored;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
#[cfg(test)]
#[path = "🧪️tests/💪️adds-an-arc-beam/🦀️.rs"]
mod tests_adds_an_arc_beam;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-an-inclined-beam/🦀️.rs"]
mod tests_adds_an_inclined_beam;
#[cfg(test)]
#[path = "🧪️tests/🧱️flat-arc/🦀️.rs"]
mod tests_flat_arc;
//#endregion 🔖️Leaves

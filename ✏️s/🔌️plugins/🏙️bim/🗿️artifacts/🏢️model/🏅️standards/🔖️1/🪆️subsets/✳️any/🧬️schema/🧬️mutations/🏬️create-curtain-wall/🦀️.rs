//! 🏬️ `create-curtain-wall` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-facade/🦀️.rs"]
mod tests_adds_a_facade;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-a-curved-facade/🦀️.rs"]
mod tests_adds_a_curved_facade;
#[cfg(test)]
#[path = "🧪️tests/✨️adds-a-facade-with-its-own-grid/🦀️.rs"]
mod tests_adds_a_facade_with_its_own_grid;
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
#[path = "🧪️tests/🛑️type-missing/🦀️.rs"]
mod tests_type_missing;
#[cfg(test)]
#[path = "🧪️tests/🚷️zero-length/🦀️.rs"]
mod tests_zero_length;
#[cfg(test)]
#[path = "🧪️tests/🙅️lines-not-ascending/🦀️.rs"]
mod tests_lines_not_ascending;
#[cfg(test)]
#[path = "🧪️tests/📛️spacing-non-positive/🦀️.rs"]
mod tests_spacing_non_positive;
//#endregion 🔖️Leaves

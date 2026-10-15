//! 🏠️ `create-roof` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️adds-a-gable/🦀️.rs"]
mod tests_adds_a_gable;
#[cfg(test)]
#[path = "🧪️tests/➕️adds-a-flat-roof/🦀️.rs"]
mod tests_adds_a_flat_roof;
#[cfg(test)]
#[path = "🧪️tests/✨️adds-a-mansard/🦀️.rs"]
mod tests_adds_a_mansard;
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
#[path = "🧪️tests/🛑️self-intersecting/🦀️.rs"]
mod tests_self_intersecting;
#[cfg(test)]
#[path = "🧪️tests/🚷️pitch-zero/🦀️.rs"]
mod tests_pitch_zero;
#[cfg(test)]
#[path = "🧪️tests/🙅️pitch-too-steep/🦀️.rs"]
mod tests_pitch_too_steep;
#[cfg(test)]
#[path = "🧪️tests/📛️negative-overhang/🦀️.rs"]
mod tests_negative_overhang;
#[cfg(test)]
#[path = "🧪️tests/🧭️id-taken-by-another-kind/🦀️.rs"]
mod tests_id_taken_by_another_kind;
//#endregion 🔖️Leaves

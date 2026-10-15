//! 🪧️ `place-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️sets-absolute-placements/🦀️.rs"]
mod tests_sets_absolute_placements;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/🚧️kind-mismatch/🦀️.rs"]
mod tests_kind_mismatch;
#[cfg(test)]
#[path = "🧪️tests/🛑️same-placement/🦀️.rs"]
mod tests_same_placement;
#[cfg(test)]
#[path = "🧪️tests/⛔️empty-map/🦀️.rs"]
mod tests_empty_map;
//#endregion 🔖️Leaves

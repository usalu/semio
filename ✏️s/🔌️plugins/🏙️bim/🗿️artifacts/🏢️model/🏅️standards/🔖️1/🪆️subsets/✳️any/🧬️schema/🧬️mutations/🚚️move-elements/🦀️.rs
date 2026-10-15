//! 🚚️ `move-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves-a-wall-and-its-opening/🦀️.rs"]
mod tests_moves_a_wall_and_its_opening;
#[cfg(test)]
#[path = "🧪️tests/🚛️moves-every-placed-kind/🦀️.rs"]
mod tests_moves_every_placed_kind;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/⛔️empty-selection/🦀️.rs"]
mod tests_empty_selection;
#[cfg(test)]
#[path = "🧪️tests/🛑️zero-vector/🦀️.rs"]
mod tests_zero_vector;
#[cfg(test)]
#[path = "🧪️tests/🚧️storey-has-no-placement/🦀️.rs"]
mod tests_storey_has_no_placement;
#[cfg(test)]
#[path = "🧪️tests/🧭️moves-a-ramp/🦀️.rs"]
mod tests_moves_a_ramp;
//#endregion 🔖️Leaves

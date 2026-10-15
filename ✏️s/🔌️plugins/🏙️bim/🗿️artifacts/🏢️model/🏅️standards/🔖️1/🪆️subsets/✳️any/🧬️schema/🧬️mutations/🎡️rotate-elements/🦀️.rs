//! 🎡️ `rotate-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️turns-a-wall-and-a-column/🦀️.rs"]
mod tests_turns_a_wall_and_a_column;
#[cfg(test)]
#[path = "🧪️tests/🚛️turns-every-placed-kind/🦀️.rs"]
mod tests_turns_every_placed_kind;
#[cfg(test)]
#[path = "🧪️tests/🌀️turns-by-thirty-degrees/🦀️.rs"]
mod tests_turns_by_thirty_degrees;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/⛔️empty-selection/🦀️.rs"]
mod tests_empty_selection;
#[cfg(test)]
#[path = "🧪️tests/🛑️zero-angle/🦀️.rs"]
mod tests_zero_angle;
#[cfg(test)]
#[path = "🧪️tests/🚧️storey-has-no-placement/🦀️.rs"]
mod tests_storey_has_no_placement;
#[cfg(test)]
#[path = "🧪️tests/🧭️turns-a-ramp/🦀️.rs"]
mod tests_turns_a_ramp;
//#endregion 🔖️Leaves

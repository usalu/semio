//! 🔀️ `mirror-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️mirrors-walls-and-their-openings/🦀️.rs"]
mod tests_mirrors_walls_and_their_openings;
#[cfg(test)]
#[path = "🧪️tests/🌀️mirrors-an-arc-wall-and-a-curtain-wall/🦀️.rs"]
mod tests_mirrors_an_arc_wall_and_a_curtain_wall;
#[cfg(test)]
#[path = "🧪️tests/🚛️mirrors-every-placed-kind/🦀️.rs"]
mod tests_mirrors_every_placed_kind;
#[cfg(test)]
#[path = "🧪️tests/🌟️mirrors-a-turning-stair/🦀️.rs"]
mod tests_mirrors_a_turning_stair;
#[cfg(test)]
#[path = "🧪️tests/🔗️mirrors-as-copies/🦀️.rs"]
mod tests_mirrors_as_copies;
#[cfg(test)]
#[path = "🧪️tests/🚧️fixed-hand-stair/🦀️.rs"]
mod tests_fixed_hand_stair;
#[cfg(test)]
#[path = "🧪️tests/🛑️line-without-length/🦀️.rs"]
mod tests_line_without_length;
#[cfg(test)]
#[path = "🧪️tests/⛔️empty-selection/🦀️.rs"]
mod tests_empty_selection;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/🧲️already-symmetric/🦀️.rs"]
mod tests_already_symmetric;
//#endregion 🔖️Leaves

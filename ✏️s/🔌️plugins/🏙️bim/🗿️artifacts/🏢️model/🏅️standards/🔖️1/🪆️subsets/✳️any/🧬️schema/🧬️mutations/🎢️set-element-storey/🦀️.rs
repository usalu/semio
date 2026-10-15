//! 🎢️ `set-element-storey` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️moves-a-wall-and-its-openings-follow/🦀️.rs"]
mod tests_moves_a_wall_and_its_openings_follow;
#[cfg(test)]
#[path = "🧪️tests/🏛️moves-a-column/🦀️.rs"]
mod tests_moves_a_column;
#[cfg(test)]
#[path = "🧪️tests/🛋️moves-a-room/🦀️.rs"]
mod tests_moves_a_room;
#[cfg(test)]
#[path = "🧪️tests/⬜️moves-a-slab/🦀️.rs"]
mod tests_moves_a_slab;
#[cfg(test)]
#[path = "🧪️tests/🚪️opening-would-break/🦀️.rs"]
mod tests_opening_would_break;
#[cfg(test)]
#[path = "🧪️tests/🔝️top-no-longer-above-the-base/🦀️.rs"]
mod tests_top_no_longer_above_the_base;
#[cfg(test)]
#[path = "🧪️tests/🧲️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🚫️storey-missing/🦀️.rs"]
mod tests_storey_missing;
#[cfg(test)]
#[path = "🧪️tests/🏘️other-building/🦀️.rs"]
mod tests_other_building;
#[cfg(test)]
#[path = "🧪️tests/🪟️opening-follows-its-host/🦀️.rs"]
mod tests_opening_follows_its_host;
#[cfg(test)]
#[path = "🧪️tests/🌍️not-storey-placed/🦀️.rs"]
mod tests_not_storey_placed;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

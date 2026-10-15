//! 🕰️ `set-element-phase` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️demolishes-a-wall/🦀️.rs"]
mod tests_demolishes_a_wall;
#[cfg(test)]
#[path = "🧪️tests/🏚️keeps-a-demolished-wall-existing/🦀️.rs"]
mod tests_keeps_a_demolished_wall_existing;
#[cfg(test)]
#[path = "🧪️tests/🛋️phases-a-room/🦀️.rs"]
mod tests_phases_a_room;
#[cfg(test)]
#[path = "🧪️tests/⬜️phases-a-slab/🦀️.rs"]
mod tests_phases_a_slab;
#[cfg(test)]
#[path = "🧪️tests/🧲️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/🪟️opening-takes-the-phase-of-its-host/🦀️.rs"]
mod tests_opening_takes_the_phase_of_its_host;
#[cfg(test)]
#[path = "🧪️tests/🌍️carries-no-phase/🦀️.rs"]
mod tests_carries_no_phase;
#[cfg(test)]
#[path = "🧪️tests/⛔️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

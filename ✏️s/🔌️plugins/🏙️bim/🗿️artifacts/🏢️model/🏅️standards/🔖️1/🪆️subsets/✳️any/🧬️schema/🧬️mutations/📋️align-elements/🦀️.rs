//! 📋️ `align-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️aligns-walls-to-a-line/🦀️.rs"]
mod tests_aligns_walls_to_a_line;
#[cfg(test)]
#[path = "🧪️tests/🌟️aligns-centres/🦀️.rs"]
mod tests_aligns_centres;
#[cfg(test)]
#[path = "🧪️tests/🌀️aligns-an-arc-by-its-extremes/🦀️.rs"]
mod tests_aligns_an_arc_by_its_extremes;
#[cfg(test)]
#[path = "🧪️tests/🧲️already-aligned/🦀️.rs"]
mod tests_already_aligned;
#[cfg(test)]
#[path = "🧪️tests/🪝️openings-follow-their-host/🦀️.rs"]
mod tests_openings_follow_their_host;
#[cfg(test)]
#[path = "🧪️tests/⛔️empty-selection/🦀️.rs"]
mod tests_empty_selection;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
//#endregion 🔖️Leaves

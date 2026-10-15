//! 💣️ `delete-elements` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️cascades-openings-and-data/🦀️.rs"]
mod tests_cascades_openings_and_data;
#[cfg(test)]
#[path = "🧪️tests/🚛️removes-furnishing-kinds/🦀️.rs"]
mod tests_removes_furnishing_kinds;
#[cfg(test)]
#[path = "🧪️tests/🌀️removes-a-storey-with-everything-on-it/🦀️.rs"]
mod tests_removes_a_storey_with_everything_on_it;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/⛔️empty-selection/🦀️.rs"]
mod tests_empty_selection;
#[cfg(test)]
#[path = "🧪️tests/🚧️pinned-storey/🦀️.rs"]
mod tests_pinned_storey;
#[cfg(test)]
#[path = "🧪️tests/🧭️removes-a-wall-with-its-sweeps/🦀️.rs"]
mod tests_removes_a_wall_with_its_sweeps;
//#endregion 🔖️Leaves

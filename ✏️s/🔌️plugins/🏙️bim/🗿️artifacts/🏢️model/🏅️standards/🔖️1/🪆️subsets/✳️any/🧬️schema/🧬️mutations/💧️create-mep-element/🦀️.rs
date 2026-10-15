//! 💧️ `create-mep-element` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️routes-a-duct/🦀️.rs"]
mod tests_routes_a_duct;
#[cfg(test)]
#[path = "🧪️tests/➕️routes-a-pipe-with-a-riser/🦀️.rs"]
mod tests_routes_a_pipe_with_a_riser;
#[cfg(test)]
#[path = "🧪️tests/✨️routes-a-cable-tray/🦀️.rs"]
mod tests_routes_a_cable_tray;
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
#[path = "🧪️tests/🛑️duct-without-width/🦀️.rs"]
mod tests_duct_without_width;
#[cfg(test)]
#[path = "🧪️tests/🚷️pipe-without-diameter/🦀️.rs"]
mod tests_pipe_without_diameter;
#[cfg(test)]
#[path = "🧪️tests/🙅️path-too-short/🦀️.rs"]
mod tests_path_too_short;
#[cfg(test)]
#[path = "🧪️tests/📛️repeated-point/🦀️.rs"]
mod tests_repeated_point;
//#endregion 🔖️Leaves

//! 🪪️ `rename-element` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️renames-a-wall/🦀️.rs"]
mod tests_renames_a_wall;
#[cfg(test)]
#[path = "🧪️tests/🚛️relabels-a-grid-line/🦀️.rs"]
mod tests_relabels_a_grid_line;
#[cfg(test)]
#[path = "🧪️tests/🚫️unknown-id/🦀️.rs"]
mod tests_unknown_id;
#[cfg(test)]
#[path = "🧪️tests/🛑️same-name/🦀️.rs"]
mod tests_same_name;
//#endregion 🔖️Leaves

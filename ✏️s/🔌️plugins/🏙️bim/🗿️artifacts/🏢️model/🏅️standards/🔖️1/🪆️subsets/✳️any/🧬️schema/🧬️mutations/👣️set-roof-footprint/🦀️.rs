//! 👣️ `set-roof-footprint` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️reshapes/🦀️.rs"]
mod tests_reshapes;
#[cfg(test)]
#[path = "🧪️tests/➕️curves-an-edge/🦀️.rs"]
mod tests_curves_an_edge;
#[cfg(test)]
#[path = "🧪️tests/🚫️unchanged/🦀️.rs"]
mod tests_unchanged;
#[cfg(test)]
#[path = "🧪️tests/⛔️too-few-vertices/🦀️.rs"]
mod tests_too_few_vertices;
#[cfg(test)]
#[path = "🧪️tests/❌️self-intersecting/🦀️.rs"]
mod tests_self_intersecting;
#[cfg(test)]
#[path = "🧪️tests/🛑️missing/🦀️.rs"]
mod tests_missing;
//#endregion 🔖️Leaves

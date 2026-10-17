//! 〰️ `set-wall-axis` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/🔗️follows-by-inference/🦀️.rs"]
mod tests_follows_by_inference;
//#endregion 🔖️Leaves

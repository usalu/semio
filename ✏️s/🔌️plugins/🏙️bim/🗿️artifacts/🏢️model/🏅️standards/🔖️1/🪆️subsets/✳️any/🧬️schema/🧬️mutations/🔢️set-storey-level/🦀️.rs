//! 🔢️ `set-storey-level` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/✅️relevels/🦀️.rs"]
mod tests_relevels;
#[cfg(test)]
#[path = "🧪️tests/🚫️level-taken/🦀️.rs"]
mod tests_level_taken;
//#endregion 🔖️Leaves

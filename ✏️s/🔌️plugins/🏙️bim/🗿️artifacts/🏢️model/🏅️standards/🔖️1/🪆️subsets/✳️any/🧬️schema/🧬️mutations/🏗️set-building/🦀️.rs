//! 🏗️ `set-building` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/🪜️raises-the-absolute-levels/🦀️.rs"]
mod tests_raises_the_absolute_levels;
//#endregion 🔖️Leaves

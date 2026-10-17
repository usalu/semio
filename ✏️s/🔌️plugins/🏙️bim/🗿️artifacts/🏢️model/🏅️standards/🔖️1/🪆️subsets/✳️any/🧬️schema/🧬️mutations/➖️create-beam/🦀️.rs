//! ➖️ `create-beam` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
pub use component::*;
#[cfg(test)]
#[path = "🧪️tests/📌️authored-top-offset-is-stored/🦀️.rs"]
mod tests_authored_top_offset_is_stored;
//#endregion 🔖️Leaves

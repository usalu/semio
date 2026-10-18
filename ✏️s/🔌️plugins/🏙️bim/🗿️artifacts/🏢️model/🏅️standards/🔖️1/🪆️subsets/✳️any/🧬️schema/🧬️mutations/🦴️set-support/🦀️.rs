//! 🦴️ `set-support` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
pub use component::*;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🔖️Leaves

//! 🦴️ `create-load` mutation leaf: mounts the payload, its behavior facets and its tests under one module.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
//#region 🔖️Leaves
#[path = "🦠️mutation/🦀️.rs"]
mod component;
pub use component::*;
#[path = "🔺️diff/🦀️.rs"]
pub mod diff;
#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;
//#endregion 🔖️Leaves

//! 🔖️ v1 of `semio-s-artifact-wfc-grid2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::Grid2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Grid2dSnapshot;
pub use crate::standards::v1::subsets::any::schema::snapshot::WFC_GRID2D_DOCUMENT_SCHEMA;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcAdjacencyRule2d;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcCell2d;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcColor;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcDirection2d;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcPathSegment;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcPinnedCell2d;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcPoint2;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcTile2d;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcTileMedia2d;
pub use crate::standards::v1::subsets::any::schema::snapshot::WfcVectorPath;
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
    pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_wfc_grid2d_tests_cuts_the_pinned_corner_out_of_the_problem::{WFC_GRID2D_DIALECT};
pub use crate::standards::v1::artifact_items_2::{artifact_kind};
pub use crate::standards::v1::artifact_items_3::{ArtifactApps, artifact, definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

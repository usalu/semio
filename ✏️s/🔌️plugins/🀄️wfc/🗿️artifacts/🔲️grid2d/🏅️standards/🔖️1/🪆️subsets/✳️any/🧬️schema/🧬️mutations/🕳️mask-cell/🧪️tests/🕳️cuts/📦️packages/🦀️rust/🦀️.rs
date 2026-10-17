//! 🕳️ tests-cuts-the-pinned-corner-out-of-the-problem of `semio-s-artifact-wfc-grid2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

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
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use crate::viewer::grid2d::artifact_items::{WFC_GRID2D_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

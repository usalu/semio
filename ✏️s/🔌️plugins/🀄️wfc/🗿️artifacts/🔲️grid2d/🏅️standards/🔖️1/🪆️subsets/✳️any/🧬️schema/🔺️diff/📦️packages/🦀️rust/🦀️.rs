//! 🔺️ diff of `semio-s-artifact-wfc-grid2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::Grid2dSnapshot;
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
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::snapshot::WFC_GRID2D_DOCUMENT_SCHEMA;
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

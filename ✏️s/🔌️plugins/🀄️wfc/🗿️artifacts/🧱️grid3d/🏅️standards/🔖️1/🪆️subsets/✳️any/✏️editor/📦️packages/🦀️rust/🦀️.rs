//! ✏️ editor of `semio-s-artifact-wfc-grid3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::mutations::Grid3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Grid3dSnapshot;
pub use crate::schema::snapshot::WFC_GRID3D_DOCUMENT_SCHEMA;
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::editor::grid3d::artifact_items::{WFC_GRID3D_DIALECT};
pub use crate::editor::grid3d::artifact_items_2::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

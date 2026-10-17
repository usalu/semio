//! 💡️ inferences of `semio-s-artifact-gis-gisterrain` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::snapshot::GisTerrainSnapshot;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::standards::v1::subsets::any::schema::imported_map::artifact_items::{GISTERRAIN_DIALECT, GIS_3D_TERRAIN_SCHEMA};
pub use crate::standards::v1::subsets::any::schema::imported_map::artifact_items_3::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, gis_terrain_mesh_from_snapshot};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

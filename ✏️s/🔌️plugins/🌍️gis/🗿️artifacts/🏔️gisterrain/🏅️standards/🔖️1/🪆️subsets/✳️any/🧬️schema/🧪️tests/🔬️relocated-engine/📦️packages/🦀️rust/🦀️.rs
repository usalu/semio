//! 🔬️ relocated-engine-tests of `semio-s-artifact-gis-gisterrain` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::snapshot::GisTerrainSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::GisTerrainDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_gis_gisterrain_inferences::{GISTERRAIN_DIALECT, GIS_3D_TERRAIN_SCHEMA, gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, gis_terrain_mesh_from_snapshot};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

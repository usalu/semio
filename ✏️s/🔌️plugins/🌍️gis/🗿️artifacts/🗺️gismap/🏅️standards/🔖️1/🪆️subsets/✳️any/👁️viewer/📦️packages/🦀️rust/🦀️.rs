//! 👁️ gismap of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::snapshot::GisMapSnapshot;
pub use crate::standards::v1::subsets::any::schema::diff::GisMapDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_gis_gismap_inferences::{GISMAP_DIALECT, GIS_MAP_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

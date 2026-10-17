//! 💡️ inferences of `semio-s-artifact-gis-gismap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::GisMapSnapshot;
pub use crate::schema::feature::MapFeature;
pub use crate::schema::feature::MapFeaturePatch;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub use crate::editor::gis2d::constants_and_io::artifact_items::{GISMAP_DIALECT, GIS_MAP_SCHEMA};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

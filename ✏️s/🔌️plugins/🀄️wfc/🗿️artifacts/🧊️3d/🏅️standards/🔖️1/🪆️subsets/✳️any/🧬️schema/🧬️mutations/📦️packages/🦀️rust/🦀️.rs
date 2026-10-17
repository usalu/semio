//! 🧬️ mutations of `semio-s-artifact-wfc-3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
}
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_wfc_3d_snapshot::{unit_wedge_media};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

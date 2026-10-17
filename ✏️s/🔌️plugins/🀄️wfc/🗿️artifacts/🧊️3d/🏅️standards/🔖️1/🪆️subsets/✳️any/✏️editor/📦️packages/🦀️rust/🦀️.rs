//! ✏️ wfc3d of `semio-s-artifact-wfc-3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::mutations::Wfc3dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc3dSnapshot;
pub use crate::schema::snapshot::WFC3D_DOCUMENT_SCHEMA;
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
}
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
pub use semio_s_artifact_wfc_3d_snapshot::{unit_box_media};
pub use crate::editor::wfc3d::artifact_items::{WFC3D_DIALECT};
pub use crate::editor::wfc3d::artifact_items_2::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

//! 🔖️ v1 of `semio-s-artifact-wfc-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::mutations::Wfc2dMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc2dSnapshot;
pub use crate::schema::snapshot::WFC_2D_DOCUMENT_SCHEMA;
pub use crate::schema::snapshot::WFC_2D_DEFAULT_RELATION;
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
pub use semio_s_artifact_wfc_2d_inferences::{WFC_2D_DIALECT};
pub use crate::standards::v1::artifact_items_2::{artifact_kind};
pub use crate::standards::v1::artifact_items_3::{ArtifactApps, artifact, definition};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

//! ✏️ bitmap of `semio-s-artifact-wfc-bitmap` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::standards::v1::subsets::any::schema::mutations::BitmapMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot;
pub use crate::schema::snapshot::WFC_BITMAP_DOCUMENT_SCHEMA;
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
pub use semio_s_artifact_wfc_bitmap_snapshot::{bitmap_layers_json};
pub use semio_s_artifact_wfc_bitmap_tests_shrinks_the_output_and_cascades_a_pin::{WFC_BITMAP_DIALECT};
pub use crate::standards::v1::artifact_items_2::{artifact_kind};
pub use crate::standards::v1::artifact_items_4::{ArtifactApps, artifact, definition, pilot_languages};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

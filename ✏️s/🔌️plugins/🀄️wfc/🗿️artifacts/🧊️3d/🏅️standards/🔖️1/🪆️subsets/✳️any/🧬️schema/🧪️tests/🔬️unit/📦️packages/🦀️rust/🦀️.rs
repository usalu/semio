//! 🔬️ tests of `semio-s-artifact-wfc-3d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::standards::v1::subsets::any::schema::snapshot::Wfc3dSnapshot;
pub use crate::schema::snapshot::WFC3D_DOCUMENT_SCHEMA;
pub use crate::standards::v1::subsets::any::schema::mutations::Wfc3dMutation;
pub use crate::standards::v1::subsets::any::schema::diff::Wfc3dDiff;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::io::Wfc3dBuilderConstruction;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::io::Wfc3dAnalyzerAnalysis;
pub mod examples {
    pub use crate::standards::v1::subsets::any::examples::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
#[cfg(test)]
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
    pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
    #[cfg(test)] pub use crate::host::inferences::*;
}
#[cfg(test)]
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_wfc_3d_wfc3d::{WFC3D_DIALECT};
pub use crate::standards::v1::artifact_items_4::{ArtifactApps, artifact, definition, wfc3d_languages};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;

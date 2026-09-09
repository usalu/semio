//! 🧬️ schema leaf — the schema-first Rust mirror of the viewer's shareable live ephemeral state.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.generation.3d.viewer.presence")]
pub struct Generation3dViewPresence {
    #[state(presence)]
    pub preview_camera: Generation3dViewCamera,
    #[state(presence)]
    pub show_mode: String,
}

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::viewer::generation3d::config::Generation3dViewCamera;
//#endregion 🔁️Re-exports

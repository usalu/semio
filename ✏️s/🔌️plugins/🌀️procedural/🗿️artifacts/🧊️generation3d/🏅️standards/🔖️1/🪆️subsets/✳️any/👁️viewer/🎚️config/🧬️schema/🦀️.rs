//! 🧬️ schema leaf — the schema-first Rust mirror of `🔣️.json` / `🟦️.ts` for the viewer's own
//! persisted view state. Every field is `#[state(config)]`: a read-only surface owns no document
//! and no draft lane, so config is the only lane its `ViewEmit` can write.

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.generation.3d.viewer.config")]
pub struct Generation3dViewConfig {
    #[state(config)]
    pub lod_mode: String,
    #[state(config)]
    pub show_mode: String,
    #[state(config)]
    pub preview_camera: Generation3dViewCamera,
    #[state(config)]
    pub sun_json: String,
    #[state(config)]
    pub active_example_id: String,
}

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use super::Generation3dViewCamera;
//#endregion 🔁️Re-exports

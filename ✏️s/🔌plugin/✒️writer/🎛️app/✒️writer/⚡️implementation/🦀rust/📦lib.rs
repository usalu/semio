//! ✍️ Writer app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖Constants
pub const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";
//#endregion 🔖Constants

//#region 🔖Types
/// 📷 Editor viewport transform — session-only runtime state (see `WriterPlayRuntime::camera` in the
/// ui crate), never a `WriterProjection` document field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WriterCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for WriterCamera {
    fn default() -> Self {
        default_camera()
    }
}

pub fn default_zoom() -> f64 {
    1.0
}

pub fn default_uri() -> String {
    "writer://empty".into()
}

pub fn default_camera() -> WriterCamera {
    WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 }
}

/// 📝 The full writer document projection: identity, language and source text. The editor viewport
/// camera is session-only view state (never a document field) — see `WriterPlayRuntime::camera` in
/// the ui crate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "writer", layout = "lines")]
pub struct WriterProjection {
    pub schema: String,
    pub id: String,
    pub language_id: String,
    #[serde(default = "default_uri")]
    pub uri: String,
    #[serde(default)]
    pub text: String,
}
//#endregion 🔖Types

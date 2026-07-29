//! ✍️ Writer app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};

//#region 🔖Constants
pub const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";
//#endregion 🔖Constants

//#region 🔖Types
/// 📷 Editor viewport transform persisted in the document projection. No `#[dsl(keyword = ...)]`:
/// every field that embeds it (`WriterProjection::camera`, `WriterOperation::SetCamera::camera`)
/// is itself `#[dsl(block)]`, which already supplies the bare leading keyword.
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

pub fn default_zoom() -> f64 {
    1.0
}

pub fn default_uri() -> String {
    "writer://empty".into()
}

pub fn default_camera() -> WriterCamera {
    WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 }
}

/// 📝 The full writer document projection: identity, language, source text and camera.
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
    #[serde(default = "default_camera")]
    #[dsl(block)]
    pub camera: WriterCamera,
}
//#endregion 🔖Types

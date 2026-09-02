//! 🧬️ schema leaf
use crate::editor::procedural3d::config::Procedural3dPreviewCamera;
use flow::CameraJson;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.3d.presence")]
pub struct Procedural3dPresence {
    #[state(presence)]
    pub camera: CameraJson,
    #[state(presence)]
    pub preview_camera: Procedural3dPreviewCamera,
    #[state(presence)]
    pub active_utility_id: String,
    #[state(presence)]
    pub show_mode: String,
}

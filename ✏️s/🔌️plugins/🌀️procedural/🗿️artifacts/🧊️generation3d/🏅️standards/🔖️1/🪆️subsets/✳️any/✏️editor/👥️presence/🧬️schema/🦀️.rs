//! 🧬️ schema leaf

use ::semio_framework_schema::ArtifactSchema;
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.generation.3d.presence")]
pub struct Generation3dPresence {
    #[state(presence)]
    pub camera: CameraJson,
    #[state(presence)]
    pub preview_camera: Generation3dPreviewCamera,
    #[state(presence)]
    pub show_mode: String,
}

//#region 🔁️Re-exports

//#endregion 🔁️Re-exports

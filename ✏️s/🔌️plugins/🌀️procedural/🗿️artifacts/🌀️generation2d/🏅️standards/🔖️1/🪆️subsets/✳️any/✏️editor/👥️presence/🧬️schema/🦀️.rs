//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.generation.2d.presence")]
pub struct Generation2dPresence {
    #[state(presence)]
    pub camera: CameraJson,
    #[state(presence)]
    pub show_mode: String,
    #[state(presence)]
    pub selected_generation_id: Option<String>,
}

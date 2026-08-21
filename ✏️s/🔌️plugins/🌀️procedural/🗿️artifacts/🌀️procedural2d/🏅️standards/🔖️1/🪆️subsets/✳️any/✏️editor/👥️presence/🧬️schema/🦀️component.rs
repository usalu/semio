//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.2d.presence")]
pub struct Procedural2dPresence {
    #[state(presence)]
    pub camera: CameraJson,
    #[state(presence)]
    pub show_mode: String,
    #[state(presence)]
    pub selected_generation_id: Option<String>,
}

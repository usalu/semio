//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use flow::CameraJson;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.2d.presence")]
pub struct Procedural2dPresence {
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub camera: CameraJson,
    #[state(shared_ui)] pub show_mode: String,
    #[state(shared_ui)] pub selected_generation_id: Option<String>,
}


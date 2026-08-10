//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use flow::CameraJson;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.2d.config")]
pub struct Procedural2dConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub camera: CameraJson,
    #[state(local_ui)] pub show_mode: String,
    #[state(local_ui)] pub selected_generation_id: Option<String>,
    #[state(local_ui)] pub generation_preview_text: Option<String>,
    #[state(local_ui)] pub locale: String,
}


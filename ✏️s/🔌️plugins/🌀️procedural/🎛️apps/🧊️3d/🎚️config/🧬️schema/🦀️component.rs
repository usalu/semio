//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use flow::CameraJson;
use super::Procedural3dPreviewCamera;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.3d.config")]
pub struct Procedural3dConfig {
    #[state(local_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub show_mode: String,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub hovered_node_id: Option<String>,
    #[state(local_ui)] pub camera: CameraJson,
    #[state(local_ui)] pub preview_camera: Procedural3dPreviewCamera,
    #[state(local_ui)] pub sun_json: String,
    #[state(local_ui)] pub selected_generation_id: Option<String>,
    #[state(local_ui)] pub generation_preview_text: Option<String>,
    #[state(local_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
}


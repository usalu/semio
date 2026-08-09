//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use flow::CameraJson;
use crate::apps::procedural3d::config::Procedural3dPreviewCamera;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.3d.presence")]
pub struct Procedural3dPresence {
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_node_id: Option<String>,
    #[state(shared_ui)] pub camera: CameraJson,
    #[state(shared_ui)] pub preview_camera: Procedural3dPreviewCamera,
    #[state(shared_ui)] pub selection_method: String,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub show_mode: String,
}


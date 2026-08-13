//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use flow::CameraJson;
use crate::apps::procedural3d::config::Procedural3dPreviewCamera;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.procedural.3d.presence")]
pub struct Procedural3dPresence {
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(presence)] pub hovered_node_id: Option<String>,
    #[state(presence)] pub camera: CameraJson,
    #[state(presence)] pub preview_camera: Procedural3dPreviewCamera,
    #[state(presence)] pub selection_method: String,
    #[state(presence)] pub active_utility_id: String,
    #[state(presence)] pub show_mode: String,
}


//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly.presence")]
pub struct LowpolyPresence {
    #[state(shared_ui)] pub selection_mode: String,
    #[state(shared_ui)] pub selection_ids: Vec<u32>,
    #[state(shared_ui)] pub selection_targets_mesh: bool,
    #[state(shared_ui)] pub selection_targets_vertex: bool,
    #[state(shared_ui)] pub selection_targets_edge: bool,
    #[state(shared_ui)] pub selection_targets_face: bool,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_object_id: Option<String>,
    #[state(shared_ui)] pub hovered_target_object_id: Option<String>,
    #[state(shared_ui)] pub hovered_target_mode: Option<String>,
    #[state(shared_ui)] pub hovered_target_id: Option<u32>,
    #[state(shared_ui)] pub world_camera_position: [f64; 3],
    #[state(shared_ui)] pub world_camera_target: [f64; 3],
    #[state(shared_ui)] pub world_camera_fov: f64,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub paint_utility: String,
}

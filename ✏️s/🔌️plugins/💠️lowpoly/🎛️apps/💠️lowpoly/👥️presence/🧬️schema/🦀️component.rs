//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.lowpoly.lowpoly.presence")]
pub struct LowpolyPresence {
    #[state(presence)] pub selection_mode: String,
    #[state(presence)] pub selection_ids: Vec<u32>,
    #[state(presence)] pub selection_targets_mesh: bool,
    #[state(presence)] pub selection_targets_vertex: bool,
    #[state(presence)] pub selection_targets_edge: bool,
    #[state(presence)] pub selection_targets_face: bool,
    #[state(presence)] pub selected_object_ids: Vec<String>,
    #[state(presence)] pub hovered_object_id: Option<String>,
    #[state(presence)] pub hovered_target_object_id: Option<String>,
    #[state(presence)] pub hovered_target_mode: Option<String>,
    #[state(presence)] pub hovered_target_id: Option<u32>,
    #[state(presence)] pub world_camera_position: [f64; 3],
    #[state(presence)] pub world_camera_target: [f64; 3],
    #[state(presence)] pub world_camera_fov: f64,
    #[state(presence)] pub active_utility_id: String,
    #[state(presence)] pub paint_utility: String,
}

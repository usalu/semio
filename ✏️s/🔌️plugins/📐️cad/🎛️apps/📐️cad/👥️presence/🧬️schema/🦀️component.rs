//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.cad.cad.presence")]
pub struct CadPresence {
    #[state(presence)] pub selected_object_ids: Vec<String>,
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(presence)] pub hovered_object_id: Option<String>,
    #[state(presence)] pub hovered_target_object_id: Option<String>,
    #[state(presence)] pub hovered_target_mode: Option<String>,
    #[state(presence)] pub hovered_target_id: Option<u32>,
    #[state(presence)] pub active_object_id: Option<String>,
    #[state(presence)] pub component_selection_mode: String,
    #[state(presence)] pub component_selection_ids: Vec<u32>,
    #[state(presence)] pub component_selection_targets_mesh: bool,
    #[state(presence)] pub component_selection_targets_vertex: bool,
    #[state(presence)] pub component_selection_targets_edge: bool,
    #[state(presence)] pub component_selection_targets_face: bool,
    #[state(presence)] pub camera_position: [f64; 3],
    #[state(presence)] pub camera_target: [f64; 3],
    #[state(presence)] pub camera_zoom: f64,
    #[state(presence)] pub camera_fov: f64,
    #[state(presence)] pub active_utility_id: String,
    #[state(presence)] pub engagement_step: String,
    #[state(presence)] pub engagement_pane: Option<String>,
}

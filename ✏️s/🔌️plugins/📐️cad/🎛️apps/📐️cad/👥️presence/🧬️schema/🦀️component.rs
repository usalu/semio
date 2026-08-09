//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.cad.cad.presence")]
pub struct CadPresence {
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub hovered_object_id: Option<String>,
    #[state(shared_ui)] pub hovered_target_object_id: Option<String>,
    #[state(shared_ui)] pub hovered_target_mode: Option<String>,
    #[state(shared_ui)] pub hovered_target_id: Option<u32>,
    #[state(shared_ui)] pub active_object_id: Option<String>,
    #[state(shared_ui)] pub component_selection_mode: String,
    #[state(shared_ui)] pub component_selection_ids: Vec<u32>,
    #[state(shared_ui)] pub component_selection_targets_mesh: bool,
    #[state(shared_ui)] pub component_selection_targets_vertex: bool,
    #[state(shared_ui)] pub component_selection_targets_edge: bool,
    #[state(shared_ui)] pub component_selection_targets_face: bool,
    #[state(shared_ui)] pub camera_position: [f64; 3],
    #[state(shared_ui)] pub camera_target: [f64; 3],
    #[state(shared_ui)] pub camera_zoom: f64,
    #[state(shared_ui)] pub camera_fov: f64,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub engagement_step: String,
    #[state(shared_ui)] pub engagement_pane: Option<String>,
}

//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow.config")]
pub struct FlowConfig {
    #[state(local_ui)] pub selected_node_ids: Vec<String>,
    #[state(local_ui)] pub selected_edge_ids: Vec<String>,
    #[state(local_ui)] pub selected_handle_ids: Vec<String>,
    #[state(local_ui)] pub preview_off_node_ids: Vec<String>,
    #[state(local_ui)] pub camera: CameraJson,
    #[state(local_ui)] pub lod_mode: String,
    #[state(local_ui)] pub proximity_distance: f64,
    #[state(local_ui)] pub grid_visible: bool,
    #[state(local_ui)] pub grid_snap_enabled: bool,
    #[state(local_ui)] pub grid_factor: f64,
    #[state(local_ui)] pub catalogue_sections_json: String,
    #[state(local_ui)] pub automation_enabled_json: String,
    #[state(local_ui)] pub contributions_json: String,
    #[state(local_ui)] pub generation_json: String,
    #[state(local_ui)] pub locale: String,
}


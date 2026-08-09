//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow.presence")]
pub struct FlowPresence {
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub selected_edge_ids: Vec<String>,
    #[state(shared_ui)] pub selected_handle_ids: Vec<String>,
    #[state(shared_ui)] pub preview_off_node_ids: Vec<String>,
    #[state(shared_ui)] pub camera: CameraJson,
}

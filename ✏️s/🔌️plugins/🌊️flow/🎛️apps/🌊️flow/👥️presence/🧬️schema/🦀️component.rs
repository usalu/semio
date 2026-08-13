//! 🧬️ schema leaf
use flow::CameraJson;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.flow.flow.presence")]
pub struct FlowPresence {
    #[state(presence)] pub selected_node_ids: Vec<String>,
    #[state(presence)] pub selected_edge_ids: Vec<String>,
    #[state(presence)] pub selected_handle_ids: Vec<String>,
    #[state(presence)] pub preview_off_node_ids: Vec<String>,
    #[state(presence)] pub camera: CameraJson,
}

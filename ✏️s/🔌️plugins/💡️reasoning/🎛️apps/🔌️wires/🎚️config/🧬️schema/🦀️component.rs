//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires.config")]
pub struct WiresConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub drag_node_id: Option<String>,
    #[state(local_ui)] pub drag_last_x: f64,
    #[state(local_ui)] pub drag_last_y: f64,
    #[state(local_ui)] pub locale: String,
}


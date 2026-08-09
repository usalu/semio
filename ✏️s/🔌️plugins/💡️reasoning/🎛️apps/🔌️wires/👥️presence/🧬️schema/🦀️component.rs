//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.reasoning.wires.presence")]
pub struct WiresPresence {
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(shared_ui)] pub drag_node_id: Option<String>,
    #[state(shared_ui)] pub drag_last_x: f64,
    #[state(shared_ui)] pub drag_last_y: f64,
}

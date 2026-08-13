//! 🧬️ schema leaf
use crate::artifacts::program::registers::AdjacencyKind;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.architect.architect.presence")]
pub struct ArchitectPresence {
    #[state(presence)] pub selected_ids: Vec<String>,
    #[state(presence)] pub active_register: String,
    #[state(presence)] pub adjacency_kind_filter: Option<AdjacencyKind>,
    #[state(presence)] pub graph_camera_x: f64,
    #[state(presence)] pub graph_camera_y: f64,
    #[state(presence)] pub graph_camera_zoom: f64,
}

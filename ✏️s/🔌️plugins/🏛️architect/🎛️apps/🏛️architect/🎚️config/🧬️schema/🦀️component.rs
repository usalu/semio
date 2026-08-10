//! 🧬️ schema leaf
use crate::artifacts::program::registers::AdjacencyKind;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.architect.architect.config")]
pub struct ArchitectConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub active_register: String,
    #[state(local_ui)] pub search_query: String,
    #[state(local_ui)] pub search_history_json: String,
    #[state(local_ui)] pub active_report_json: String,
    #[state(local_ui)] pub last_result_json: String,
    #[state(local_ui)] pub last_analysis_json: String,
    #[state(local_ui)] pub adjacency_kind_filter: Option<AdjacencyKind>,
    #[state(local_ui)] pub graph_camera_x: f64,
    #[state(local_ui)] pub graph_camera_y: f64,
    #[state(local_ui)] pub graph_camera_zoom: f64,
}


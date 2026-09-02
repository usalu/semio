//! 🧬️ schema leaf
use crate::artifacts::program::registers::AdjacencyKind;
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.architect.architect.presence")]
pub struct ArchitectPresence {
    #[state(presence)]
    pub active_register: String,
    #[state(presence)]
    pub adjacency_kind_filter: Option<AdjacencyKind>,
    #[state(presence)]
    pub graph_camera_x: f64,
    #[state(presence)]
    pub graph_camera_y: f64,
    #[state(presence)]
    pub graph_camera_zoom: f64,
}

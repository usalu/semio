//! 🧬️ schema leaf
use crate::artifacts::fem2d::FemCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.fem.2d.config")]
pub struct Fem2dConfig {
    #[state(local_ui)] pub result_source_id: Option<String>,
    #[state(local_ui)] pub result_mode: String,
    #[state(local_ui)] pub result_mode_index: u32,
    #[state(local_ui)] pub camera: FemCamera,
    #[state(local_ui)] pub locale: String,
}


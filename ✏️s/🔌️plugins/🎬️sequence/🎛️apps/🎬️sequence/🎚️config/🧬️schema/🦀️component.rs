//! 🧬️ schema leaf
use crate::artifacts::sequence::SequenceCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence.config")]
pub struct SequenceConfig {
    #[state(local_ui)] pub selected_step_ids: Vec<String>,
    #[state(local_ui)] pub last_run_json: String,
    #[state(local_ui)] pub orientation: String,
    #[state(local_ui)] pub camera: SequenceCamera,
    #[state(local_ui)] pub locale: String,
}


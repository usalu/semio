//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.vcs.vcs.config")]
pub struct VcsDemoConfig {
    #[state(local_ui)] pub selected_checkpoint_ids: Vec<String>,
    #[state(local_ui)] pub locale: String,
}


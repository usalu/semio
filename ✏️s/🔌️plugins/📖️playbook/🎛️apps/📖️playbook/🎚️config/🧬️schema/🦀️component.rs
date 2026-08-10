//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.playbook.playbook.config")]
pub struct PlaybookConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
}


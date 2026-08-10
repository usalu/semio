//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms.config")]
pub struct FormsConfig {
    #[state(local_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub current_step_index: u32,
    #[state(local_ui)] pub try_values_json: String,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub contributions_json: String,
}


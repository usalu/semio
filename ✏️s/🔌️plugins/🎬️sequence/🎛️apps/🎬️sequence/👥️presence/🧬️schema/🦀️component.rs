//! 🧬️ schema leaf
use crate::artifacts::sequence::SequenceCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sequence.sequence.presence")]
pub struct SequencePresence {
    #[state(shared_ui)] pub selected_step_ids: Vec<String>,
    #[state(shared_ui)] pub orientation: String,
    #[state(shared_ui)] pub camera: SequenceCamera,
}

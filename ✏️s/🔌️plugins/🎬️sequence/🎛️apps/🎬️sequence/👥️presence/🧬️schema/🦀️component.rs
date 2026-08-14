//! 🧬️ schema leaf
use crate::artifacts::sequence::SequenceCamera;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sequence.sequence.presence")]
pub struct SequencePresence {
    #[state(presence)] pub orientation: String,
    #[state(presence)] pub camera: SequenceCamera,
}

//! 🧬️ schema leaf
use crate::SequenceCamera;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sequence.sequence.presence")]
pub struct SequencePresence {
    #[state(presence)]
    pub orientation: String,
    #[state(presence)]
    pub camera: SequenceCamera,
}

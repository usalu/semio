//! 🧬️ Playground snapshot schema — artifact-lane fields only.

use crate::PLAYGROUND_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted playground document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.demonstrator.playground")]
#[dsl(extension = "playground")]
#[dsl(layout = "lines")]
pub struct PlaygroundSnapshot {
    #[state(artifact)]
    pub schema: String,
}

impl Default for PlaygroundSnapshot {
    fn default() -> Self {
        Self { schema: PLAYGROUND_DOCUMENT_SCHEMA.into() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs


//! 🧬️ Writer snapshot schema — artifact-lane fields only.

use crate::artifacts::writer::{document_child_handle_and_cache, WriterDocumentChild, WRITER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted writer document snapshot. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`
/// wave 3 (`writer→C:document`): the inline `text: String` content field is replaced by a fixed
/// composed `s.stdio.semio.document` CHILD slot — the writer plugin no longer defines its own
/// text-block content model, it composes stdio's `document` subset instead. `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub language_id: String,
    #[state(artifact)]
    #[serde(default = "crate::artifacts::writer::default_uri")]
    pub uri: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.document")]
    pub document: WriterDocumentChild,
}

impl Default for WriterSnapshot {
    fn default() -> Self {
        Self {
            schema: WRITER_DOCUMENT_SCHEMA.into(),
            id: String::new(),
            language_id: "plaintext".into(),
            uri: crate::artifacts::writer::default_uri(),
            document: document_child_handle_and_cache("", "", "plaintext"),
        }
    }
}
//#endregion 🔖️Snapshot

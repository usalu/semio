//! 🧬️ Writer snapshot schema — artifact-lane fields only.

use crate::{document_child_handle_with_text, WriterDocumentChild, WRITER_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted writer document snapshot. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`
/// wave 3 (`writer→C:document`): the inline `text: String` content field is replaced by a fixed
/// composed `s.stdio.semio`/`document` CHILD slot — the writer plugin no longer defines its own
/// text-block content model, it composes stdio's `document` subset instead. `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub language_id: String,
    #[state(artifact)]
    #[value(default = "crate::default_uri")]
    pub uri: String,
    /// ✍️ The authored body the composed `document` child is derived from — the PERSISTED
    /// payload of that child slot, mirroring `🏭️process`'s `stock_payload`/`step_payloads`. A
    /// composed child's own content never travels inside `store::ArtifactChild` (identity only), and the
    /// react shell answers `Effect::LoadDocument` with an EMPTY member roster, so
    /// `crate::genesis_writer_child_pack` can only derive the child's block tree from a field the
    /// parent's own pack/DSL round-trips: without this field every whole-document load materialised an
    /// empty document.
    #[state(artifact)]
    #[value(default)]
    pub text: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub document: WriterDocumentChild,
}

impl Default for WriterSnapshot {
    fn default() -> Self {
        Self { schema: WRITER_DOCUMENT_SCHEMA.into(), id: String::new(), language_id: "plaintext".into(), uri: crate::default_uri(), text: String::new(), document: document_child_handle_with_text("", "", "plaintext") }
    }
}
//#endregion 🔖️Snapshot

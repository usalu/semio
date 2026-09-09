//! 🧬️ Writer diff schema — sparse field delta over the artifact.

use crate::WriterDocumentChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta; `document` carries a whole-handle replacement (content-addressed, so a
/// changed handle IS the change signal — see `📓️wave3-reports/lowpoly-report.md`'s
/// `mesh: Option<Option<ArtifactChild<…>>>` precedent; writer's `document` slot is never absent,
/// only ever replaced, so a single `Option<WriterDocumentChild>` — not the double-`Option` an
/// optional slot needs — is the sparse-vs-unchanged signal here).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.writer.writer")]
pub struct WriterDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::WriterArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub language_id: Option<String>,
    #[state(artifact)]
    pub uri: Option<String>,
    #[state(artifact)]
    pub document: Option<WriterDocumentChild>,
}
//#endregion 🔖️Diff

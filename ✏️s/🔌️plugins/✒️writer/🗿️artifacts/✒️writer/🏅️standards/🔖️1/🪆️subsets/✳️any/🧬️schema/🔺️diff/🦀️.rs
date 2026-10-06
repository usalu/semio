//! 🧬️ Writer diff schema — sparse field delta over the artifact.

use crate::WriterDocumentChild;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta; `document` carries a whole-handle replacement (content-addressed, so a
/// changed handle IS the change signal — see `📓️wave3-reports/lowpoly-report.md`'s
/// `mesh: Option<Option<ArtifactChild<…>>>` precedent; writer's `document` slot is never absent,
/// only ever replaced, so a single `Option<WriterDocumentChild>` — not the double-`Option` an
/// optional slot needs — is the sparse-vs-unchanged signal here).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]
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
    /// ✍️ The authored body the replacement `document` handle was minted from — the persisted
    /// payload of the composed child slot (`WriterSnapshot::text`). It travels WITH the handle so an
    /// applied diff leaves the parent able to re-derive its child through `genesis_writer_child_pack`.
    #[state(artifact)]
    pub text: Option<String>,
    #[state(artifact)]
    pub document: Option<WriterDocumentChild>,
}
//#endregion 🔖️Diff

use crate::schema::WriterArtifact;
use crate::document_child_handle_with_text;
use crate::WriterSnapshot;
use protocol::MutationDiff;
use super::*;
use protocol::DiffText;
use protocol::DiffBinary;

impl WriterDiff {
    /// 🧬️ Applies every sparse entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &WriterArtifact) -> protocol::MutationApplyResult<WriterArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(language_id) = &self.language_id {
                next.language_id = language_id.clone();
            }
            if let Some(uri) = &self.uri {
                next.uri = uri.clone();
            }
            if let Some(text) = &self.text {
                next.text = text.clone();
            }
            if let Some(document) = &self.document {
                next.document = document.clone();
            }
            next
        })
    }
}

impl MutationDiff<WriterSnapshot> for WriterDiff {
    fn apply(&self, snapshot: &WriterSnapshot) -> protocol::MutationApplyResult<WriterSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(language_id) = &self.language_id {
                next.language_id = language_id.clone();
            }
            if let Some(uri) = &self.uri {
                next.uri = uri.clone();
            }
            if let Some(text) = &self.text {
                next.text = text.clone();
            }
            if let Some(document) = &self.document {
                next.document = document.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.id.is_some() {
            self.id = other.id;
        }
        if other.language_id.is_some() {
            self.language_id = other.language_id;
        }
        if other.uri.is_some() {
            self.uri = other.uri;
        }
        if other.text.is_some() {
            self.text = other.text;
        }
        if other.document.is_some() {
            self.document = other.document;
        }
    }
}

pub fn diff_set_snapshot(snapshot: &WriterSnapshot) -> WriterDiff {
    WriterDiff { artifact: Some(Box::new(WriterArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

/// 🔺️ Mints a new content-addressed `document` handle for the whole-body replacement `text` and
/// attaches the artifact-instance text owner (`document_child_handle_with_text`) — real handcrafted
/// construction, never apply-then-capture. `id`/`language_id` come from `base` since the handle's
/// target/content both need them.
pub fn diff_set_text(text: &str, id: &str, language_id: &str) -> WriterDiff {
    WriterDiff { text: Some(text.to_string()), document: Some(document_child_handle_with_text(id, text, language_id)), ..Default::default() }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

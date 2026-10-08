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

use crate::document_child_handle_with_text;
use crate::WriterSnapshot;
use protocol::MutationDiff;
use super::*;

impl MutationDiff<WriterSnapshot> for WriterDiff {
    fn apply(&self, snapshot: &WriterSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<WriterSnapshot> {
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
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(language_id);
        take!(uri);
        take!(text);
        take!(document);
    }
}

impl protocol::DiffAlgebra<WriterSnapshot> for WriterDiff {
    fn inverse(&self, base: &WriterSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            language_id: self.language_id.as_ref().map(|_| base.language_id.clone()),
            uri: self.uri.as_ref().map(|_| base.uri.clone()),
            text: self.text.as_ref().map(|_| base.text.clone()),
            document: self.document.as_ref().map(|_| base.document.clone()),
        }
    }
    fn between(base: &WriterSnapshot, other: &WriterSnapshot) -> Self {
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            id: (base.id != other.id).then(|| other.id.clone()),
            language_id: (base.language_id != other.language_id).then(|| other.language_id.clone()),
            uri: (base.uri != other.uri).then(|| other.uri.clone()),
            text: (base.text != other.text).then(|| other.text.clone()),
            document: (base.document != other.document).then(|| other.document.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.id.is_none() && self.language_id.is_none() && self.uri.is_none() && self.text.is_none() && self.document.is_none()
    }
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

#[cfg(test)]
use protocol::{DiffBinary,DiffText};

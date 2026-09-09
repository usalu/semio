//! 🔺️ Writer artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::WriterArtifact;
use crate::{document_child_handle_with_text, WriterSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::schema::diff::*;

//#region 🔖️Apply
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
        if other.document.is_some() {
            self.document = other.document;
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
pub fn diff_set_snapshot(snapshot: &WriterSnapshot) -> WriterDiff {
    WriterDiff { artifact: Some(Box::new(WriterArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

/// 🔺️ Mints a new content-addressed `document` handle for the whole-body replacement `text` and
/// attaches the artifact-instance text owner (`document_child_handle_with_text`) — real handcrafted
/// construction, never apply-then-capture. `id`/`language_id` come from `base` since the handle's
/// target/content both need them.
pub fn diff_set_text(text: &str, id: &str, language_id: &str) -> WriterDiff {
    WriterDiff { document: Some(document_child_handle_with_text(id, text, language_id)), ..Default::default() }
}
//#endregion 🔖️Builders

impl protocol::DiffCodec for WriterDiff {
    fn print_diff(&self) -> String {
        dsl::os_pack::json::to_json_string(self)
    }

    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        dsl::os_pack::json::from_json_str(line).map_err(|error| dsl::__rt::field_error(error.to_string()))
    }

    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.print_diff().into_bytes())
    }

    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed { what: "diff utf8", offset: 0, detail: error.to_string() })?;
        Self::parse_diff(line).map_err(|error| protocol::ProtocolError::Malformed { what: "diff json", offset: 0, detail: error.to_string() })
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;

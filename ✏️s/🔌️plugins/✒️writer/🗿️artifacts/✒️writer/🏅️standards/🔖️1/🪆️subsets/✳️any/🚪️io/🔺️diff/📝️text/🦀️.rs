//! 🔺️ Writer artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::writer::schema::WriterArtifact;
use crate::artifacts::writer::{document_child_handle_with_text, WriterSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::writer::schema::diff::*;

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
            if let Some(selection) = &self.editor_selection {
                next.editor_selection = selection.clone();
            }
            if let Some(settings) = &self.editor_settings {
                next.editor_settings = settings.clone();
            }
            if let Some(value) = self.format_signal {
                next.format_signal = value;
            }
            if let Some(value) = self.lint_signal {
                next.lint_signal = value;
            }
            if let Some(value) = self.revision {
                next.revision = value;
            }
            if let Some(value) = &self.engagement_input {
                next.engagement_input = value.clone();
            }
            if let Some(value) = self.camera_x {
                next.camera_x = value;
            }
            if let Some(value) = self.camera_y {
                next.camera_y = value;
            }
            if let Some(value) = self.camera_zoom {
                next.camera_zoom = value;
            }
            if let Some(value) = &self.locale {
                next.locale = value.clone();
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
        if other.editor_selection.is_some() {
            self.editor_selection = other.editor_selection;
        }
        if other.editor_settings.is_some() {
            self.editor_settings = other.editor_settings;
        }
        if other.format_signal.is_some() {
            self.format_signal = other.format_signal;
        }
        if other.lint_signal.is_some() {
            self.lint_signal = other.lint_signal;
        }
        if other.revision.is_some() {
            self.revision = other.revision;
        }
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.camera_x.is_some() {
            self.camera_x = other.camera_x;
        }
        if other.camera_y.is_some() {
            self.camera_y = other.camera_y;
        }
        if other.camera_zoom.is_some() {
            self.camera_zoom = other.camera_zoom;
        }
        if other.locale.is_some() {
            self.locale = other.locale;
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
        serde_json::to_string(self).expect("serialize writer diff")
    }

    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|error| dsl::__rt::field_error(error.to_string()))
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
mod tests {
    use super::*;
    use protocol::DiffCodec;

    fn jack_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_diff_print_parse_round_trips() {
        let diffs = vec![diff_set_text("hello", "jack", "jack"), diff_set_snapshot(&jack_snapshot()), WriterDiff::default()];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
            let parsed = WriterDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff failed for {printed:?}: {e}"));
            assert_eq!(parsed, diff, "DiffCodec text round trip diverged for {printed:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_diff_encode_decode_round_trips_and_matches_text() {
        let diffs = vec![diff_set_text("hello", "jack", "jack"), diff_set_snapshot(&jack_snapshot()), WriterDiff::default()];
        for diff in diffs {
            let bytes = diff.encode_diff().expect("encode_diff");
            let decoded = WriterDiff::decode_diff(&bytes).expect("decode_diff");
            assert_eq!(decoded, diff, "DiffCodec binary round trip diverged");
        }
    }

    /// 🔺️ `diff_set_text` mints a real content-addressed `document` handle and seeds the working-
    /// scene cache, honestly replacing the retired byte-range-edit law (composed-child handles are
    /// whole-value replacements, not sub-string patches — see this file's `🔖️Builders` doc comment).
    #[semio_framework_async_macros::async_test]
    async fn diff_set_text_mints_a_document_handle_and_caches_its_text() {
        let base = WriterSnapshot::default();
        let diff = diff_set_text("hio", "jack", "plaintext");
        let next = diff.apply(&base).expect("valid mutation diff");
        assert_eq!(crate::artifacts::writer::writer_text(&next), "hio");
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

//! 🔺️ Writer artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::writer::schema::diff::{
    WriterDiff, WriterStringList, WriterTextDelta, WriterTextRangeEdit,
};
use crate::artifacts::writer::schema::WriterArtifact;
use crate::artifacts::writer::WriterSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::writer::schema::diff::*;

//#region 🔖️TextApply
fn apply_text_delta(base: &str, delta: &WriterTextDelta) -> String {
    if let Some(replacement) = &delta.replacement {
        return replacement.clone();
    }
    let mut text = base.to_string();
    for edit in &delta.edits {
        let start = edit.start as usize;
        let end = edit.end as usize;
        let safe_start = start.min(text.len());
        let safe_end = end.min(text.len()).max(safe_start);
        let mut next = String::new();
        next.push_str(&text[..safe_start]);
        next.push_str(&edit.insert);
        next.push_str(&text[safe_end..]);
        text = next;
    }
    text
}

fn absorb_text_delta(dst: &mut WriterTextDelta, src: WriterTextDelta) {
    if src.replacement.is_some() {
        dst.replacement = src.replacement;
        dst.edits.clear();
        return;
    }
    dst.edits.extend(src.edits);
}
//#endregion 🔖️TextApply

//#region 🔖️Apply
impl WriterDiff {
    /// 🧬️ Applies every sparse entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &WriterArtifact) -> WriterArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
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
            next.text = apply_text_delta(&next.text, text);
        }
        if let Some(list) = &self.selected_ast_ids {
            next.selected_ast_ids = list.values.clone();
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
        if let Some(value) = &self.tree_hovered_ast_id {
            next.tree_hovered_ast_id = value.clone();
        }
        if let Some(value) = &self.editor_hover_offset {
            next.editor_hover_offset = value.clone();
        }
        next
    }
}

impl MutationDiff<WriterSnapshot> for WriterDiff {
    fn apply(&self, snapshot: &WriterSnapshot) -> WriterSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
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
            next.text = apply_text_delta(&next.text, text);
        }
        next
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
        if let Some(text) = other.text {
            match &mut self.text {
                Some(dst) => absorb_text_delta(dst, text),
                None => self.text = Some(text),
            }
        }
        if other.selected_ast_ids.is_some() {
            self.selected_ast_ids = other.selected_ast_ids;
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
        if other.tree_hovered_ast_id.is_some() {
            self.tree_hovered_ast_id = other.tree_hovered_ast_id;
        }
        if other.editor_hover_offset.is_some() {
            self.editor_hover_offset = other.editor_hover_offset;
        }
    }
}
//#endregion 🔖️Apply

//#region 🔖️Builders
pub fn diff_set_snapshot(snapshot: &WriterSnapshot) -> WriterDiff {
    WriterDiff {
        artifact: Some(Box::new(WriterArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}

pub fn diff_set_text(text: &str) -> WriterDiff {
    WriterDiff {
        text: Some(WriterTextDelta {
            replacement: Some(text.to_string()),
            edits: Vec::new(),
        }),
        ..Default::default()
    }
}

pub fn diff_text_range_edit(start: u32, end: u32, insert: &str) -> WriterDiff {
    WriterDiff {
        text: Some(WriterTextDelta {
            replacement: None,
            edits: vec![WriterTextRangeEdit {
                start,
                end,
                insert: insert.to_string(),
            }],
        }),
        ..Default::default()
    }
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
        let line = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Malformed {
            what: "diff utf8",
            offset: 0,
            detail: error.to_string(),
        })?;
        Self::parse_diff(line).map_err(|error| protocol::ProtocolError::Malformed {
            what: "diff json",
            offset: 0,
            detail: error.to_string(),
        })
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::DiffCodec;

    fn jack_snapshot() -> WriterSnapshot {
        WriterSnapshot {
            schema: "writer.document".into(),
            id: "jack".into(),
            language_id: "jack".into(),
            uri: "writer://jack".into(),
            text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into(),
        }
    }

    #[test]
    fn writer_diff_print_parse_round_trips() {
        let diffs = vec![
            diff_set_text("hello"),
            diff_set_snapshot(&jack_snapshot()),
            WriterDiff::default(),
        ];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
            let parsed = WriterDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff failed for {printed:?}: {e}"));
            assert_eq!(parsed, diff, "DiffCodec text round trip diverged for {printed:?}");
        }
    }

    #[test]
    fn writer_diff_encode_decode_round_trips_and_matches_text() {
        let diffs = vec![
            diff_set_text("hello"),
            diff_set_snapshot(&jack_snapshot()),
            WriterDiff::default(),
        ];
        for diff in diffs {
            let bytes = diff.encode_diff().expect("encode_diff");
            let decoded = WriterDiff::decode_diff(&bytes).expect("decode_diff");
            assert_eq!(decoded, diff, "DiffCodec binary round trip diverged");
        }
    }

    #[test]
    fn text_range_edit_honestly_patches_substring() {
        let base = WriterSnapshot {
            text: "hello".into(),
            ..WriterSnapshot::default()
        };
        let diff = diff_text_range_edit(1, 4, "i");
        let next = diff.apply(&base);
        assert_eq!(next.text, "hio");
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}

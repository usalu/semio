//! 🔺️ Writer artifact — the operation diff (constitutional: diff).

use crate::artifacts::writer::WriterProjection;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDiff)]
#[serde(rename_all = "camelCase")]
pub struct WriterDiff {
    pub text: Option<String>,
    #[dsl(block)]
    pub document: Option<WriterProjection>,
}

impl OperationDiff<WriterProjection> for WriterDiff {
    fn apply(&self, projection: &WriterProjection) -> WriterProjection {
        if let Some(document) = &self.document {
            return document.clone();
        }
        WriterProjection { text: self.text.clone().unwrap_or_else(|| projection.text.clone()), ..projection.clone() }
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = other;
            return;
        }
        if other.text.is_some() {
            self.text = other.text;
        }
    }
}
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::DiffCodec;

    /// ✍️ Hand-built representative document — used across the artifact's own component tests.
    fn jack_projection() -> WriterProjection {
        WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
    }

    #[test]
    fn writer_diff_print_parse_round_trips() {
        let diffs = vec![WriterDiff { text: Some("hello".into()), document: None }, WriterDiff { text: None, document: Some(jack_projection()) }, WriterDiff::default()];
        for diff in diffs {
            let printed = diff.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line: {printed:?}");
            let parsed = WriterDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff failed for {printed:?}: {e}"));
            assert_eq!(parsed, diff, "DiffCodec text round trip diverged for {printed:?}");
        }
    }

    #[test]
    fn writer_diff_encode_decode_round_trips_and_matches_text() {
        let diffs = vec![WriterDiff { text: Some("hello".into()), document: None }, WriterDiff { text: None, document: Some(jack_projection()) }, WriterDiff::default()];
        for diff in diffs {
            let bytes = diff.encode_diff().expect("encode_diff");
            let decoded = WriterDiff::decode_diff(&bytes).expect("decode_diff");
            assert_eq!(decoded, diff, "DiffCodec binary round trip diverged");
        }
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


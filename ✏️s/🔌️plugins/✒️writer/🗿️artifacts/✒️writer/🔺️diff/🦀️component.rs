//! 🔺️ Writer artifact — the operation diff (constitutional: diff).

use crate::artifacts::writer::WriterProjection;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

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

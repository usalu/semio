//! ⚡ Writer app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use writer::WriterProjection;

//#region 🔖Types
/// 📐 Typed content mutation for a `WriterProjection`. The editor viewport camera is session-only
/// runtime state now (never a document operation — see `WriterPlayRuntime::camera` in the ui crate).
/// Each variant's op keyword is the auto-derived kebab-case of its own name (`SetText` -> `set-text`,
/// ...) — see {@link protocol::OpText}.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum WriterOperation {
    SetText { text: String },
    SetDocument {
        #[dsl(block)]
        document: WriterProjection,
    },
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterDiff {
    pub text: Option<String>,
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

impl Operation<WriterProjection> for WriterOperation {
    type Diff = WriterDiff;

    fn diff(&self, _projection: &WriterProjection) -> WriterDiff {
        match self {
            WriterOperation::SetText { text } => WriterDiff { text: Some(text.clone()), ..Default::default() },
            WriterOperation::SetDocument { document } => WriterDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &WriterProjection) -> Vec<Self> {
        match self {
            WriterOperation::SetText { .. } => vec![WriterOperation::SetText { text: projection.text.clone() }],
            WriterOperation::SetDocument { .. } => vec![WriterOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖Types

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::{create_document_envelope, DocumentCommand};

    type WriterStore = store::DocumentStore<WriterProjection, WriterOperation>;

    fn seeded_store() -> WriterStore {
        WriterStore::new(create_document_envelope("writer.document", "writer", writer_engine::empty_writer_projection(), None))
    }

    #[test]
    fn writer_document_vcs_replays_text_operations() {
        let mut store = seeded_store();
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").text, "hello");
    }

    #[test]
    fn writer_document_vcs_replays_document_operation() {
        let mut store = seeded_store();
        let replacement = WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a) RETURN a".into() };
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetDocument { document: replacement }], description: None }).expect("apply document");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.id, "jack");
        assert_eq!(projection.text, "MATCH (a) RETURN a");
    }

    #[test]
    fn writer_document_vcs_undoes_text_operation() {
        let mut store = seeded_store();
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").text, "");
    }

    /// ✍️ Hand-built representative document — verbatim from the original file's `🔖DslAndOpText`
    /// test region (duplicated per-crate since each constitutional crate's tests compile independently).
    fn jack_projection() -> WriterProjection {
        WriterProjection {
            schema: "writer.document".into(),
            id: "jack".into(),
            language_id: "jack".into(),
            uri: "writer://jack".into(),
            text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into(),
        }
    }

    #[test]
    fn writer_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&WriterOperation::SetText { text: "line one\nline two".into() });
        store::test_support::assert_op_line_round_trip(&WriterOperation::SetDocument { document: jack_projection() });
    }
}
//#endregion 🧪Tests

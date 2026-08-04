//! ⚖️ Writer app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use writer::WriterCamera;
use writer_op::WriterOperation;

/// 📦️ Encodes a `WriterOperation` to its binary command form.
pub fn encode_op(operation: &WriterOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `WriterOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<WriterOperation, protocol::ProtocolError> {
    WriterOperation::decode_op(bytes)
}

//#region 🔖️WriterCommand
/// 🎯️ B1: `WriterPlayApp::Command` — the SOLE dispatch surface for writer's own behavior (mirrors
/// `shooting_protocol::ShootingCommand`'s doc comment: covers every declared action, decoded once by
/// `VcsDocumentApp::dispatch_typed_command` via `OpBinary::decode_op`). Field shapes mirror each
/// former `handle_action` action's real `args` object exactly, except `CommitRename` (which now reads
/// the rename target off `WriterConfig::editor_selection` instead of a redundant client-sent
/// `occurrences` array — the config already carries it, see `writer_ui::WriterPlayApp::handle`).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
pub enum WriterCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "text-edit")]
    TextEdit { text: String },
    #[dsl(key = "set-text")]
    SetText { text: String },
    #[dsl(key = "document")]
    SetDocument {
        #[dsl(block)]
        document: writer::WriterProjection,
    },
    #[dsl(key = "document-json")]
    SetDocumentJson { json: String },
    #[dsl(key = "fixture-json")]
    SetFixtureJson { json: String },
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "format-document")]
    FormatDocument,
    #[dsl(key = "commit-rename")]
    CommitRename { text: String },

    // 👁️ Config-only (was ephemeral `WriterPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: WriterCamera,
    },
    #[dsl(key = "request-completions")]
    RequestCompletions,
    #[dsl(key = "lint-document")]
    LintDocument,
    #[dsl(key = "text-select")]
    TextSelect { start: usize, end: usize },
    #[dsl(key = "editor-selection")]
    SetEditorSelection { start: usize, end: usize },
    #[dsl(key = "select-ast-node")]
    SelectAstNode { id: String, start: usize, end: usize },
    #[dsl(key = "ast-selection")]
    SetAstSelection { ids: Vec<String> },
    #[dsl(key = "ast-hover")]
    SetAstHover { id: Option<String> },
    #[dsl(key = "text-hover")]
    TextHover { start: Option<usize>, end: Option<usize> },
    #[dsl(key = "toggle-line-numbers")]
    ToggleLineNumbers,
    #[dsl(key = "font-px")]
    SetFontPx { value: u32 },
    #[dsl(key = "line-height")]
    SetLineHeight { value: u32 },
    #[dsl(key = "tab-size")]
    SetTabSize { value: u32 },
    #[dsl(key = "engagement-input")]
    EngagementInput { value: String },
    #[dsl(key = "engagement-submit")]
    EngagementSubmit { value: Option<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️WriterCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::{create_document_envelope, DocumentCommand, DocumentStore};
    use writer::WriterProjection;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = WriterOperation::SetText { text: "hello".into() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// ✍️ Hand-built representative document — verbatim from the original file's `🔖️DslAndOpText`
    /// test region (duplicated per-crate since each constitutional crate's tests compile independently).
    fn jack_projection() -> WriterProjection {
        WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
    }

    #[test]
    fn writer_document_text_round_trips_through_the_store() {
        let mut store = DocumentStore::<WriterProjection, WriterOperation>::new(create_document_envelope("writer.document", "writer", writer_engine::empty_writer_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetDocument { document: jack_projection() }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    //#region 🔖️CommandTests
    #[test]
    fn writer_command_binary_matches_text_for_every_shape() {
        store::test_support::assert_op_text_binary_equivalence(&WriterCommand::TextEdit { text: "hello".into() });
        store::test_support::assert_op_text_binary_equivalence(&WriterCommand::SetDocument { document: jack_projection() });
        store::test_support::assert_op_text_binary_equivalence(&WriterCommand::SetCamera { camera: WriterCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_text_binary_equivalence(&WriterCommand::TextHover { start: Some(3), end: None });
        store::test_support::assert_op_text_binary_equivalence(&WriterCommand::SetAstSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_text_binary_equivalence(&WriterCommand::EngagementSubmit { value: None });
        store::test_support::assert_op_text_binary_equivalence(&WriterCommand::ToggleLineNumbers);
    }
    //#endregion 🔖️CommandTests
}
//#endregion 🧪️Tests

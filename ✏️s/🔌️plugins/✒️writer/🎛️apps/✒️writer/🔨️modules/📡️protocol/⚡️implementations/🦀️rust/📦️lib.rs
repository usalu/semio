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

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `WriterOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing pack round-trip law (same pattern as `mathematical`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let mut store = DocumentStore::<WriterProjection, WriterOperation>::new(create_document_envelope("writer.document", "writer", writer_engine::empty_writer_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetDocument { document: jack_projection() }], description: None }).expect("apply");
        let edit: &Edit<WriterOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<WriterProjection, WriterOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests

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

    //#region [DEBUG] WireBaseline
    /// [DEBUG] Temporary wire-baseline dump for the taxonomy migration (ticket
    /// 26/08/05/WRITER-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION) — captures print_op text
    /// and hex bytes for one representative value per WriterCommand variant, in declaration order, so
    /// the post-migration app_commands! decomposition can be diffed against it byte-for-byte. Delete
    /// this test (and the [DEBUG] region) once the diff is clean.
    #[test]
    fn dump_wire_baseline() {
        fn jack_projection() -> WriterProjection {
            WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
        }
        let commands: Vec<WriterCommand> = vec![
            WriterCommand::TextEdit { text: "hello".into() },
            WriterCommand::SetText { text: "MATCH (a) RETURN a".into() },
            WriterCommand::SetDocument { document: jack_projection() },
            WriterCommand::SetDocumentJson { json: "{}".into() },
            WriterCommand::SetFixtureJson { json: "{}".into() },
            WriterCommand::SetActiveExample { example_id: "jack".into() },
            WriterCommand::FormatDocument,
            WriterCommand::CommitRename { text: "piece".into() },
            WriterCommand::SetCamera { camera: WriterCamera { x: 1.0, y: 2.0, zoom: 1.5 } },
            WriterCommand::RequestCompletions,
            WriterCommand::LintDocument,
            WriterCommand::TextSelect { start: 3, end: 7 },
            WriterCommand::SetEditorSelection { start: 3, end: 7 },
            WriterCommand::SelectAstNode { id: "jack-ast-1".into(), start: 0, end: 5 },
            WriterCommand::SetAstSelection { ids: vec!["a".into(), "b".into()] },
            WriterCommand::SetAstHover { id: Some("jack-ast-1".into()) },
            WriterCommand::TextHover { start: Some(3), end: None },
            WriterCommand::ToggleLineNumbers,
            WriterCommand::SetFontPx { value: 16 },
            WriterCommand::SetLineHeight { value: 24 },
            WriterCommand::SetTabSize { value: 4 },
            WriterCommand::EngagementInput { value: "format".into() },
            WriterCommand::EngagementSubmit { value: None },
            WriterCommand::SetLocale { value: "de-DE".into() },
        ];
        for (i, command) in commands.iter().enumerate() {
            let text = protocol::OpText::print_op(command);
            let bytes = protocol::OpBinary::encode_op(command).expect("encode");
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            println!("[DEBUG] {i:02} | {text} | len={} | {hex}", bytes.len());
        }
    }
    //#endregion [DEBUG] WireBaseline
}
//#endregion 🧪️Tests

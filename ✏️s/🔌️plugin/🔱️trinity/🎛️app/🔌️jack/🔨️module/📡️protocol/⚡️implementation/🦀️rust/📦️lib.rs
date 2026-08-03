//! ⚖️ Trinity Jack app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use serde::{Deserialize, Serialize};
use trinity_jack_op::Operation;

/// 📦️ Encodes a Trinity graph `Operation` to its binary command form.
pub fn encode_op(operation: &Operation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a Trinity graph `Operation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Operation, protocol::ProtocolError> {
    Operation::decode_op(bytes)
}

//#region 🔖️TrinityJackCommand
/// 🎯️ B1: `TrinityJackPlayApp::Command` — the SOLE dispatch surface for jack's own behavior,
/// replacing the deleted stringly-typed `handle_action`. Field shapes mirror each pre-B1 action's real
/// args exactly; `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`) AND text (`OpText`) codec,
/// matching `shooting_protocol::ShootingCommand`'s derive/attribute conventions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum TrinityJackCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "set-fixture-json")]
    SetFixtureJson { json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "patch-nodes")]
    PatchNodes { node_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "run-query")]
    RunQuery { query: Option<String> },
    #[dsl(key = "load-example-query")]
    LoadExampleQuery { query: String },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },

    // 👁️ Config-only — was ephemeral `TrinityJackRuntime` state, now emits `config_operations`.
    #[dsl(key = "set-viewport")]
    SetViewport { viewport_json: String },
    #[dsl(key = "text-edit")]
    TextEdit { text: String },
    #[dsl(key = "text-select")]
    TextSelect { start: u64, end: u64 },
    #[dsl(key = "request-completions")]
    RequestCompletions,
    #[dsl(key = "format-document")]
    FormatDocument,
    #[dsl(key = "set-lod-mode")]
    SetLodMode { window_id: String, value: String },
    #[dsl(key = "editor-engagement-input")]
    EditorEngagementInput { value: String },
    #[dsl(key = "graph-engagement-input")]
    GraphEngagementInput { value: String },
    #[dsl(key = "results-engagement-input")]
    ResultsEngagementInput { value: String },
    #[dsl(key = "graph-pointer-down")]
    GraphPointerDown { node_id: Option<String> },
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "set-locale")]
    SetLocale { value: String },
}
//#endregion 🔖️TrinityJackCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_op_binary_round_trips_and_agrees_with_text() {
        let operation = Operation::Rename { id: "node-1".into(), name: "Renamed".into() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn nakagin_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<trinity_ram::GraphFixture, Operation>(
            trinity_ram::TRINITY_GRAPH_SCHEMA,
            "doc-text-test",
            trinity_jack_engine::empty_jack_document(),
            None,
        );
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![Operation::Rename { id: "node-1".into(), name: "Renamed".into() }],
                description: None,
            })
            .ok();
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    #[test]
    fn trinity_jack_command_text_and_binary_round_trip() {
        use protocol::OpText;
        let commands = vec![
            TrinityJackCommand::SetFixtureJson { json: "{}".into() },
            TrinityJackCommand::DeleteSelection,
            TrinityJackCommand::PatchNodes { node_ids: vec!["a".into()], field: "name".into(), value: "Renamed".into() },
            TrinityJackCommand::Reorganize,
            TrinityJackCommand::RunQuery { query: Some("MATCH (a:Piece) RETURN a".into()) },
            TrinityJackCommand::RunQuery { query: None },
            TrinityJackCommand::SetActiveExample { example_id: "branch-chain".into() },
            TrinityJackCommand::SetViewport { viewport_json: "{\"x\":1.0,\"y\":2.0,\"zoom\":1.0}".into() },
            TrinityJackCommand::TextSelect { start: 3, end: 9 },
            TrinityJackCommand::SetLodMode { window_id: "trinity-jack-graph".into(), value: "compact".into() },
            TrinityJackCommand::GraphPointerDown { node_id: Some("n1".into()) },
            TrinityJackCommand::SetSelection { ids: vec!["n1".into(), "n2".into()] },
            TrinityJackCommand::SetLocale { value: "de-DE".into() },
        ];
        for command in commands {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(TrinityJackCommand::decode_op(&bytes).expect("decode"), command);
            let text = command.print_op();
            assert_eq!(TrinityJackCommand::parse_op(&text).expect("parse"), command);
        }
    }
}
//#endregion 🧪️Tests

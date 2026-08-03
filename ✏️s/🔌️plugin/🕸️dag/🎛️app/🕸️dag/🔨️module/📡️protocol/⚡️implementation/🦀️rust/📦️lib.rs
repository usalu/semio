//! ⚖️ DAG app — binary command protocol surface + laws (constitutional: protocol).
//!
//! `protocol::OpBinary for DagOperation` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`); see `s/plugin/dag/app/op/rs/lib.rs` for why. This crate only
//! adds the thin app-facing `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law.

use dag_op::DagOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `DagOperation` to its binary command form.
pub fn encode_op(operation: &DagOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DagOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DagOperation, protocol::ProtocolError> {
    DagOperation::decode_op(bytes)
}

//#region 🔖️DagNodeGraphEditOp
/// 🎯️ One batched edit inside a `DagCommand::NodeGraphEdit` — mirrors the pre-B1 `nodeGraphEdit`
/// action's `operations` JSON array (`"setFixture"`/`"deleteSelection"`/`"connect"` sub-kinds), now
/// closed and typed instead of stringly-tagged JSON. See `dag_ui`'s `DocumentApp::handle` for the
/// dispatch of each variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
pub enum DagNodeGraphEditOp {
    #[dsl(key = "set-fixture")]
    SetFixture { fixture_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
}
//#endregion 🔖️DagNodeGraphEditOp

//#region 🔖️DagCommand
/// 🎯️ B1: `DagPlayApp::Command` — the SOLE dispatch surface for dag's own behavior, now covering
/// EVERY declared action (the pre-B1 legacy `{kind,name,args}` wire-value envelope fallback is gone —
/// `DocumentApp::handle_action` no longer exists; see `dag_ui`'s `DagPlayApp::handle`). Field shapes
/// mirror each action's real args exactly, matching `shooting_protocol::ShootingCommand`'s conventions
/// — one variant per action id declared in `create_dag_app`, even where several ids used to share a
/// `handle_action` match arm (`setSelection`/`selectNode`/`nodeGraphSelect`), so `command_id`/the
/// `AppActionRegistry` kind-discipline check stay 1:1 with the manifest.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum DagCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-node")]
    AddNode { kind: String, x: Option<f64>, y: Option<f64> },
    #[dsl(key = "remove-node")]
    RemoveNode { node_id: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit {
        #[dsl(statements)]
        operations: Vec<DagNodeGraphEditOp>,
    },
    #[dsl(key = "connect-media-ports")]
    ConnectMediaPorts { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "disconnect")]
    Disconnect { edge_id: String },
    #[dsl(key = "move-media-node")]
    MoveMediaNode { node_id: String, x: f64, y: f64 },
    #[dsl(key = "rename-dag-node")]
    RenameDagNode { old_id: String, value: String },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "patch-dag-nodes")]
    PatchDagNodes { node_ids: Vec<String>, field: String, value: String },

    // 👁️ Config-only (was ephemeral `DagPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "select-node")]
    SelectNode { node_id: String },
    #[dsl(key = "node-graph-select")]
    NodeGraphSelect { node_ids: Vec<String> },
    #[dsl(key = "node-graph-hover")]
    NodeGraphHover,
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport { x: f64, y: f64, zoom: f64 },
    #[dsl(key = "graph-pointer-down")]
    GraphPointerDown,
    /// 🗣️ Host-driven locale changes — see `dag_engine::DagConfig::locale`. Not declared as a
    /// manifest action (mirrors `shooting_protocol::ShootingCommand::SetLocale`, which is likewise
    /// undeclared: locale is host-pushed, not a user-facing app action needing a palette entry).
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️DagCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = DagOperation::SetNodes { nodes: Vec::new() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn dag_command_text_binary_round_trips_document_mutating_variants() {
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::AddNode { kind: "slider".into(), x: Some(10.0), y: None });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::RemoveNode { node_id: "n1".into() });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::DeleteSelection);
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::NodeGraphEdit {
            operations: vec![
                DagNodeGraphEditOp::SetFixture { fixture_json: "{}".into() },
                DagNodeGraphEditOp::DeleteSelection,
                DagNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
            ],
        });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::Disconnect { edge_id: "e1".into() });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::RenameDagNode { old_id: "n1".into(), value: "renamed".into() });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::Reorganize);
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::PatchDagNodes { node_ids: vec!["n1".into(), "n2".into()], field: "value".into(), value: "5".into() });
    }

    #[test]
    fn dag_command_text_binary_round_trips_config_only_variants() {
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::SetSelection { ids: vec!["n1".into()] });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::SelectNode { node_id: "n1".into() });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::NodeGraphSelect { node_ids: vec!["n1".into(), "n2".into()] });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::NodeGraphHover);
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::NodeGraphViewport { x: 1.0, y: 2.0, zoom: 1.5 });
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::GraphPointerDown);
        store::test_support::assert_op_text_binary_equivalence(&DagCommand::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests

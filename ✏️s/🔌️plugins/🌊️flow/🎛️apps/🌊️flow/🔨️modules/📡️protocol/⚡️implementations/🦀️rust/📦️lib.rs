//! ⚖️ Flow app — binary command protocol surface + laws (constitutional: protocol).
//!
//! `protocol::OpBinary for FlowOperation` is implemented directly in the flow kernel crate (`flow_core`);
//! see `s/plugin/flow/app/op/rs/lib.rs` for why. This crate only adds the thin app-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law and a whole-store round trip.

use flow_core::CameraJson;
use flow_op::FlowOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `FlowOperation` to its binary command form.
pub fn encode_op(operation: &FlowOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `FlowOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<FlowOperation, protocol::ProtocolError> {
    FlowOperation::decode_op(bytes)
}

//#region 🔖️FlowNodeGraphEditOp
/// 🎯️ One batched edit inside a `FlowCommand::NodeGraphEdit`/`SpotlightCommit` — mirrors the pre-B1
/// `nodeGraphEdit`/`spotlightCommit` actions' `operations` JSON array (`"setFixture"`/
/// `"deleteSelection"`/`"connect"` sub-kinds), now closed and typed instead of stringly-tagged JSON.
/// Mirrors `dag_protocol::DagNodeGraphEditOp` exactly. See `flow_ui`'s `DocumentApp::handle` for the
/// dispatch of each variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
pub enum FlowNodeGraphEditOp {
    #[dsl(key = "set-fixture")]
    SetFixture { fixture_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "connect")]
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
}
//#endregion 🔖️FlowNodeGraphEditOp

//#region 🔖️FlowCommand
/// 🎯️ B1: `FlowPlayApp::Command` — the SOLE dispatch surface for flow's own behavior, covering EVERY
/// declared action (the pre-B1 legacy `{kind,name,args}` wire-value envelope/`handle_action` string
/// dispatch is gone — see `flow_ui`'s `FlowPlayApp::handle`). Field shapes mirror each action's real
/// args exactly, matching `shooting_protocol::ShootingCommand`/`dag_protocol::DagCommand`/
/// `procedural_3d_protocol::Procedural3dCommand`'s conventions — one variant per action id declared in
/// `create_flow_app`, even where several ids used to share a `handle_action` match arm.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FlowCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-widget")]
    AddWidget { kind: String, neuron_kind: Option<String>, x: Option<f64>, y: Option<f64> },
    #[dsl(key = "remove-widget")]
    RemoveWidget { widget_id: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "disconnect")]
    Disconnect { synapse_id: String },
    #[dsl(key = "connect-media-ports")]
    ConnectMediaPorts { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    #[dsl(key = "move-media-node")]
    MoveMediaNode { node_id: String, x: f64, y: f64 },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "patch-flow-widgets")]
    PatchFlowWidgets { widget_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "rename-flow-widget")]
    RenameFlowWidget { old_id: String, value: String },
    #[dsl(key = "node-graph-edit")]
    NodeGraphEdit {
        #[dsl(statements)]
        operations: Vec<FlowNodeGraphEditOp>,
    },
    #[dsl(key = "spotlight-commit")]
    SpotlightCommit {
        #[dsl(statements)]
        operations: Vec<FlowNodeGraphEditOp>,
    },
    /// 🧩️ Dynamic extension-provided action — `action_id` resolved at runtime against
    /// `FLOW_EXTENSIONS`; declared `in_palette: false` in the manifest (see `create_flow_app`).
    #[dsl(key = "run-extension-action")]
    RunExtensionAction { action_id: String },

    // 👁️ Config-only (was ephemeral `FlowPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "evaluate")]
    Evaluate,
    #[dsl(key = "select-all")]
    SelectAll,
    #[dsl(key = "focus-selection")]
    FocusSelection,
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String>, edge_ids: Vec<String>, handle_ids: Vec<String> },
    #[dsl(key = "select-node")]
    SelectNode { node_id: String },
    #[dsl(key = "node-graph-select")]
    NodeGraphSelect { node_ids: Vec<String> },
    #[dsl(key = "node-graph-hover")]
    NodeGraphHover,
    #[dsl(key = "graph-pointer-down")]
    GraphPointerDown,
    #[dsl(key = "node-graph-viewport")]
    NodeGraphViewport {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "set-lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "set-proximity-distance")]
    SetProximityDistance { value: f64 },
    #[dsl(key = "set-grid-visible")]
    SetGridVisible { pressed: Option<bool> },
    #[dsl(key = "set-grid-snap-enabled")]
    SetGridSnapEnabled { pressed: Option<bool> },
    #[dsl(key = "set-grid-factor")]
    SetGridFactor { value: f64 },
    #[dsl(key = "clear-selection")]
    ClearSelection,
    #[dsl(key = "context-menu-at")]
    ContextMenuAt { id: String },
    #[dsl(key = "set-preview-off")]
    SetPreviewOff { ids: Vec<String>, value: bool },
    #[dsl(key = "open-spotlight")]
    OpenSpotlight,
    #[dsl(key = "replace-image")]
    ReplaceImage { id: String },
    #[dsl(key = "set-catalogue-sections")]
    SetCatalogueSections { sections_json: String },
    #[dsl(key = "toggle-extension")]
    ToggleExtension { id: String, enabled: bool },
    #[dsl(key = "add-generation")]
    AddGeneration,
    #[dsl(key = "remove-generation")]
    RemoveGeneration { id: String },
    #[dsl(key = "select-generation")]
    SelectGeneration { id: String },
    #[dsl(key = "rename-generation")]
    RenameGeneration { id: String, name: String },
    #[dsl(key = "update-generation-values")]
    UpdateGenerationValues { generation_id: Option<String>, question_id: String, value: dsl::DslValue },
    /// 🗣️ Host-driven locale changes — see `flow_engine::FlowConfig::locale`. Not declared as a
    /// manifest action (mirrors `shooting_protocol::ShootingCommand::SetLocale`/
    /// `dag_protocol::DagCommand::SetLocale`, likewise undeclared: locale is host-pushed, not a
    /// user-facing app action needing a palette entry).
    #[dsl(key = "locale")]
    SetLocale { value: String },
    /// 🧵️ One budgeted evaluation step (see `flow_core::FlowEvalDriver::tick`), off the main thread —
    /// self-chained via `HostEffect::DispatchAction` from `Evaluate`/`FlowEvalTick`/`pending_effects`
    /// until the fixture's dirty set is empty. Not declared as a manifest action — an internal chain
    /// link, never user-facing.
    #[dsl(key = "flow-eval-tick")]
    FlowEvalTick,
    #[dsl(key = "flow-eval-resolve")]
    FlowEvalResolve { node_hash: u64, output_json: String },
}
//#endregion 🔖️FlowCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow::FlowFixture;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = FlowOperation::SetLayout { entries: Vec::new() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn flow_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<FlowFixture, FlowOperation>("flow.fixture", "doc-text-test", FlowFixture::default(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![FlowOperation::SetLayout { entries: Vec::new() }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    /// 🧪️ [DEBUG] TEMPORARY wire-shape baseline dump for the W1 taxonomy migration — prints every
    /// `FlowCommand` variant's `print_op()` line plus its `encode_op()` byte length so the post-merge
    /// `app_commands!`-generated enum can be byte-diffed against this exact output.
    #[test]
    fn debug_dump_flow_command_wire_baseline() {
        for command in baseline_commands() {
            let printed = protocol::OpText::print_op(&command);
            let bytes = protocol::OpBinary::encode_op(&command).expect("encode");
            println!("[DEBUG][WIRE] {} | {} | {}", printed, bytes.len(), bytes.iter().map(|b| format!("{b:02x}")).collect::<String>());
        }
    }

    fn baseline_commands() -> Vec<FlowCommand> {
        vec![
            FlowCommand::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None },
            FlowCommand::AddWidget { kind: "neuron".into(), neuron_kind: Some("math.add".into()), x: None, y: None },
            FlowCommand::RemoveWidget { widget_id: "n1".into() },
            FlowCommand::DeleteSelection,
            FlowCommand::Disconnect { synapse_id: "s1".into() },
            FlowCommand::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
            FlowCommand::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 },
            FlowCommand::Reorganize,
            FlowCommand::PatchFlowWidgets { widget_ids: vec!["n1".into(), "n2".into()], field: "value".into(), value: "5".into() },
            FlowCommand::RenameFlowWidget { old_id: "n1".into(), value: "renamed".into() },
            FlowCommand::NodeGraphEdit {
                operations: vec![
                    FlowNodeGraphEditOp::SetFixture { fixture_json: "{}".into() },
                    FlowNodeGraphEditOp::DeleteSelection,
                    FlowNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
                ],
            },
            FlowCommand::SpotlightCommit { operations: vec![FlowNodeGraphEditOp::DeleteSelection] },
            FlowCommand::RunExtensionAction { action_id: "flow.extension.reorganize".into() },
            FlowCommand::Evaluate,
            FlowCommand::SelectAll,
            FlowCommand::FocusSelection,
            FlowCommand::SetSelection { ids: vec!["n1".into()], edge_ids: vec!["e1".into()], handle_ids: Vec::new() },
            FlowCommand::SelectNode { node_id: "n1".into() },
            FlowCommand::NodeGraphSelect { node_ids: vec!["n1".into(), "n2".into()] },
            FlowCommand::NodeGraphHover,
            FlowCommand::GraphPointerDown,
            FlowCommand::NodeGraphViewport { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 } },
            FlowCommand::SetLodMode { value: "micro".into() },
            FlowCommand::SetProximityDistance { value: 48.0 },
            FlowCommand::SetGridVisible { pressed: Some(true) },
            FlowCommand::SetGridVisible { pressed: None },
            FlowCommand::SetGridSnapEnabled { pressed: None },
            FlowCommand::SetGridFactor { value: 10.0 },
            FlowCommand::ClearSelection,
            FlowCommand::ContextMenuAt { id: "n1".into() },
            FlowCommand::SetPreviewOff { ids: vec!["n1".into()], value: true },
            FlowCommand::OpenSpotlight,
            FlowCommand::ReplaceImage { id: "n1".into() },
            FlowCommand::SetCatalogueSections { sections_json: "[]".into() },
            FlowCommand::ToggleExtension { id: "auto-layout".into(), enabled: true },
            FlowCommand::AddGeneration,
            FlowCommand::RemoveGeneration { id: "g1".into() },
            FlowCommand::SelectGeneration { id: "g1".into() },
            FlowCommand::RenameGeneration { id: "g1".into(), name: "Copy".into() },
            FlowCommand::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) },
            FlowCommand::SetLocale { value: "de-DE".into() },
            FlowCommand::FlowEvalTick,
            FlowCommand::FlowEvalResolve { node_hash: 42, output_json: "{}".into() },
        ]
    }

    #[test]
    fn flow_command_text_binary_round_trips_document_mutating_variants() {
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::AddWidget { kind: "neuron".into(), neuron_kind: Some("math.add".into()), x: None, y: None });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::RemoveWidget { widget_id: "n1".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::DeleteSelection);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::Disconnect { synapse_id: "s1".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::Reorganize);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::PatchFlowWidgets { widget_ids: vec!["n1".into(), "n2".into()], field: "value".into(), value: "5".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::RenameFlowWidget { old_id: "n1".into(), value: "renamed".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::NodeGraphEdit {
            operations: vec![
                FlowNodeGraphEditOp::SetFixture { fixture_json: "{}".into() },
                FlowNodeGraphEditOp::DeleteSelection,
                FlowNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
            ],
        });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SpotlightCommit { operations: vec![FlowNodeGraphEditOp::DeleteSelection] });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::RunExtensionAction { action_id: "flow.extension.reorganize".into() });
    }

    #[test]
    fn flow_command_text_binary_round_trips_config_only_variants() {
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::Evaluate);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SelectAll);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::FocusSelection);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetSelection { ids: vec!["n1".into()], edge_ids: vec!["e1".into()], handle_ids: Vec::new() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SelectNode { node_id: "n1".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::NodeGraphSelect { node_ids: vec!["n1".into(), "n2".into()] });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::NodeGraphHover);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::GraphPointerDown);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::NodeGraphViewport { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetLodMode { value: "micro".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetProximityDistance { value: 48.0 });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetGridVisible { pressed: Some(true) });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetGridVisible { pressed: None });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetGridSnapEnabled { pressed: None });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetGridFactor { value: 10.0 });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::ClearSelection);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::ContextMenuAt { id: "n1".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetPreviewOff { ids: vec!["n1".into()], value: true });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::OpenSpotlight);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::ReplaceImage { id: "n1".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetCatalogueSections { sections_json: "[]".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::ToggleExtension { id: "auto-layout".into(), enabled: true });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::AddGeneration);
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::RemoveGeneration { id: "g1".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SelectGeneration { id: "g1".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::RenameGeneration { id: "g1".into(), name: "Copy".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_text_binary_equivalence(&FlowCommand::FlowEvalTick);
    }
}
//#endregion 🧪️Tests

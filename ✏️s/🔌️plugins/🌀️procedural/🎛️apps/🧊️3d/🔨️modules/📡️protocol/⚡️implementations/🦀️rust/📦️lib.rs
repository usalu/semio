//! ⚖️ Procedural 3D app — binary command protocol surface + laws (constitutional: protocol). Also
//! hosts the `Procedural3dEnvelope`/`Procedural3dStore` type aliases — moved here from
//! `procedural_3d_op` to match `shooting_protocol`'s layout, the first constitutional crate in the
//! stack where `Procedural3dOperation` (from `procedural_3d_op`) is available alongside
//! `Procedural3dDocument` (from `procedural_3d`).
//!
//! 🎯️ Also hosts `Procedural3dCommand` — the app-engine `DocumentApp::Command` binary command envelope
//! covering every action `procedural_3d_ui::create_procedural3d_app` declares. See
//! `procedural_3d_ui::Procedural3dPlayApp::handle` for the dispatch.

use flow_core::CameraJson;
use procedural_3d::Procedural3dDocument;
use procedural_3d_engine::Procedural3dPreviewCamera;
use procedural_3d_op::Procedural3dOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `Procedural3dOperation` to its binary command form.
pub fn encode_op(operation: &Procedural3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural3dOperation, protocol::ProtocolError> {
    Procedural3dOperation::decode_op(bytes)
}

//#region 🔖️Procedural3dCommand
/// 🎯️ B1: `Procedural3dPlayApp::Command` — the SOLE dispatch surface for procedural3d's own behavior,
/// covering EVERY declared action. Field shapes mirror each action's real `args` object, except
/// `NodeGraphEdit::operations_json` (the free-form node-graph sub-op array stays a JSON string, out of
/// scope for a typed sub-op enum) and `UpdateGenerationValues::value` (a schema-flexible form-question
/// value, carried as `dsl::DslValue` exactly like `procedural_3d_op::Procedural3dOperationDsl`'s
/// `GenerationUpdateValues` variant). `#[derive(dsl::DslOps)]` gives this a binary (`OpBinary`) AND text
/// (`OpText`) codec, matching `Procedural3dOperationDsl`'s derive/attribute conventions exactly, even
/// though this enum is never dispatched through `store::DocumentCommand` (it is not a `protocol::Operation`
/// — no `diff`/`backwards` — purely a command-channel wire codec).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Procedural3dCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "graph-edit")]
    NodeGraphEdit { operations_json: String },
    #[dsl(key = "delete-selection")]
    DeleteSelection,
    #[dsl(key = "remove-widget")]
    RemoveWidget { widget_id: String },
    #[dsl(key = "move-node")]
    MoveMediaNode { node_id: String, x: f64, y: f64 },
    #[dsl(key = "add-widget")]
    AddWidget { kind: String, x: Option<f64>, y: Option<f64> },
    #[dsl(key = "patch-flow-widgets")]
    PatchFlowWidgets { widget_ids: Vec<String>, field: String, value: Option<f64> },
    #[dsl(key = "reorganize")]
    Reorganize,
    #[dsl(key = "translate-selection")]
    TranslateSelection { node_ids: Vec<String>, dx: f64, dy: f64, dz: f64 },
    #[dsl(key = "rotate-selection")]
    RotateSelection { node_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    #[dsl(key = "scale-selection")]
    ScaleSelection { node_ids: Vec<String>, sx: f64, sy: f64, sz: f64 },
    #[dsl(key = "add-generation")]
    AddGeneration,
    #[dsl(key = "remove-generation")]
    RemoveGeneration { id: String },
    #[dsl(key = "rename-generation")]
    RenameGeneration { id: String, name: String },
    #[dsl(key = "update-generation-values")]
    UpdateGenerationValues { generation_id: Option<String>, question_id: String, value: dsl::DslValue },

    // 👁️ Config-only (was ephemeral `Procedural3dRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "viewport")]
    NodeGraphViewport {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "set-selection")]
    SetSelection { node_ids: Vec<String> },
    #[dsl(key = "select-node")]
    SelectNode { node_ids: Vec<String> },
    #[dsl(key = "graph-select")]
    NodeGraphSelect { node_ids: Vec<String> },
    #[dsl(key = "graph-hover")]
    NodeGraphHover { widget_id: Option<String> },
    #[dsl(key = "set-hover")]
    SetHover { object_id: Option<String> },
    #[dsl(key = "world-pointer-down")]
    WorldPointerDown,
    #[dsl(key = "graph-pointer-down")]
    GraphPointerDown,
    #[dsl(key = "world-select")]
    WorldSelect { ids: Vec<String>, merge: String },
    #[dsl(key = "world-hover")]
    WorldHover { id: Option<String> },
    #[dsl(key = "selection-method")]
    SetSelectionMethod { method: String },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "show-mode")]
    SetShowMode { value: String },
    #[dsl(key = "toggle-sun")]
    ToggleSun,
    #[dsl(key = "sun-azimuth")]
    SetSunAzimuth { value: f64 },
    #[dsl(key = "sun-elevation")]
    SetSunElevation { value: f64 },
    #[dsl(key = "sun-intensity")]
    SetSunIntensity { value: f64 },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: Procedural3dPreviewCamera,
    },
    #[dsl(key = "select-generation")]
    SelectGeneration { id: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },

    // 🧵️ Off-main-thread evaluation
    // dirty set is empty (see `Procedural3dPlayApp::handle`/`pending_effects`).
    #[dsl(key = "flow-eval-tick")]
    FlowEvalTick,
    #[dsl(key = "flow-eval-resolve")]
    FlowEvalResolve { node_hash: u64, output_json: String },
}
//#endregion 🔖️Procedural3dCommand

//#region 🔖️Store
pub type Procedural3dEnvelope = DocumentEnvelope<Procedural3dDocument, Procedural3dOperation>;
pub type Procedural3dStore = DocumentStore<Procedural3dDocument, Procedural3dOperation>;
//#endregion 🔖️Store

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::Widget;
    use procedural_3d::PROCEDURAL_3D_SCHEMA;
    use procedural_3d_engine::empty_procedural3d_projection;
    use store::{create_document_envelope, test_support, DocumentCommand};

    //#region 🔖️DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DocumentTextTests

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `Procedural3dOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing pack round-trip law (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{DocumentId, Edit, SchemaId};

        let mut store = Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        let edit: &Edit<Procedural3dOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        test_support::assert_command_envelope_round_trip::<Procedural3dDocument, Procedural3dOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests

    //#region 🔖️CommandTests
    #[test]
    fn command_op_binary_round_trips_and_agrees_with_text() {
        let command = Procedural3dCommand::SetActiveExample { example_id: "hexagonal-mushroom-column".into() };
        test_support::assert_op_text_binary_equivalence(&command);
    }

    #[test]
    fn command_binary_round_trips_every_document_mutating_variant() {
        let commands = vec![
            Procedural3dCommand::SetActiveExample { example_id: "hexagonal-mushroom-column".into() },
            Procedural3dCommand::NodeGraphEdit { operations_json: "[]".into() },
            Procedural3dCommand::DeleteSelection,
            Procedural3dCommand::RemoveWidget { widget_id: "extrude".into() },
            Procedural3dCommand::MoveMediaNode { node_id: "extrude".into(), x: 1.0, y: 2.0 },
            Procedural3dCommand::AddWidget { kind: "inputSlider".into(), x: Some(10.0), y: None },
            Procedural3dCommand::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) },
            Procedural3dCommand::Reorganize,
            Procedural3dCommand::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 },
            Procedural3dCommand::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 },
            Procedural3dCommand::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 },
            Procedural3dCommand::AddGeneration,
            Procedural3dCommand::RemoveGeneration { id: "generation-1".into() },
            Procedural3dCommand::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() },
        ];
        for command in commands {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(Procedural3dCommand::decode_op(&bytes).expect("decode"), command);
        }
    }

    #[test]
    fn command_binary_round_trips_every_config_only_variant() {
        let commands = vec![
            Procedural3dCommand::NodeGraphViewport { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } },
            Procedural3dCommand::SetSelection { node_ids: vec!["a".into()] },
            Procedural3dCommand::SelectNode { node_ids: vec!["a".into()] },
            Procedural3dCommand::NodeGraphSelect { node_ids: vec!["a".into()] },
            Procedural3dCommand::NodeGraphHover { widget_id: Some("extrude".into()) },
            Procedural3dCommand::SetHover { object_id: None },
            Procedural3dCommand::WorldPointerDown,
            Procedural3dCommand::GraphPointerDown,
            Procedural3dCommand::WorldSelect { ids: vec!["a".into()], merge: "replace".into() },
            Procedural3dCommand::WorldHover { id: Some("a".into()) },
            Procedural3dCommand::SetSelectionMethod { method: "lasso".into() },
            Procedural3dCommand::SetLodMode { value: "coarse".into() },
            Procedural3dCommand::SetShowMode { value: "wireframe".into() },
            Procedural3dCommand::ToggleSun,
            Procedural3dCommand::SetSunAzimuth { value: 90.0 },
            Procedural3dCommand::SetSunElevation { value: 45.0 },
            Procedural3dCommand::SetSunIntensity { value: 1.0 },
            Procedural3dCommand::SetCamera { camera: Procedural3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 } },
            Procedural3dCommand::SelectGeneration { id: "generation-1".into() },
            Procedural3dCommand::SetActiveUtility { utility_id: "rotate".into() },
            Procedural3dCommand::SetLocale { value: "de-DE".into() },
            Procedural3dCommand::FlowEvalTick,
        ];
        for command in commands {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(Procedural3dCommand::decode_op(&bytes).expect("decode"), command);
        }
    }
    //#endregion 🔖️CommandTests
}
//#endregion 🧪️Tests

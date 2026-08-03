//! ⚖️ Draw app — binary command protocol surface + laws (constitutional: protocol).

use draw::DrawCamera;
use draw_op::DrawOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `DrawOperation` to its binary command form.
pub fn encode_op(operation: &DrawOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `DrawOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<DrawOperation, protocol::ProtocolError> {
    DrawOperation::decode_op(bytes)
}

//#region 🔖️DrawCommand
/// 🎯️ B1: `DrawPlayApp::Command` — the SOLE dispatch surface for draw's own behavior, covering every
/// action `create_draw_app` declares (the pre-B1 `handle_action` string dispatch is gone). Field
/// shapes mirror each action's real `args` object; `PatchLayer`/`PatchLayers` carry `value` as a JSON
/// TEXT string (parsed via `serde_json::from_str`, falling back to a plain JSON string when that
/// fails) so one wire field covers every heterogeneous layer-field type (bool/number/string) — mirrors
/// `shooting_protocol::ShootingCommand`'s `PatchShots`/`PatchAssets` shape.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
pub enum DrawCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "set-document")]
    SetDocument {
        #[dsl(block)]
        document: draw::DrawDocument,
    },
    #[dsl(key = "commit-document")]
    CommitDocument {
        #[dsl(block)]
        document: draw::DrawDocument,
    },
    #[dsl(key = "fixture-json")]
    SetFixtureJson { json: String },
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "selected-opacity")]
    SetSelectedOpacity { value: f64 },
    #[dsl(key = "engagement-submit")]
    EngagementSubmit { value: Option<String> },
    #[dsl(key = "add-layer")]
    AddLayer { kind: String },
    #[dsl(key = "drop-layer-kind")]
    DropLayerKind { kind: String, target_row_id: String, drop_position: String },
    #[dsl(key = "move-layer")]
    MoveLayer { layer_id: String, target_row_id: String, drop_position: String },
    #[dsl(key = "delete-layer")]
    DeleteLayer { layer_id: String },
    #[dsl(key = "duplicate-layer")]
    DuplicateLayer { layer_id: String },
    #[dsl(key = "toggle-layer-visible")]
    ToggleLayerVisible { layer_id: String },
    #[dsl(key = "combine-boolean")]
    CombineBoolean { operation: String, ids: Vec<String> },
    #[dsl(key = "patch-layer")]
    PatchLayer { layer_id: String, field: String, value: String },
    #[dsl(key = "patch-layers")]
    PatchLayers { layer_ids: Vec<String>, field: String, value: String },

    // 👁️ Config-only — emit `config_operations`, never document operations.
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: DrawCamera,
    },
    #[dsl(key = "camera-zoom")]
    SetCameraZoom { value: f64 },
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "set-hover")]
    SetHover { id: Option<String> },
    #[dsl(key = "select-all")]
    SelectAll,
    #[dsl(key = "clear-selection")]
    ClearSelection,
    #[dsl(key = "engagement-input")]
    EngagementInput { value: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },

    // 🖱️ Internal pointer/gesture vocabulary — commit-time variants emit operations, the rest are View.
    #[dsl(key = "canvas-pointer-down")]
    CanvasPointerDown { x: f64, y: f64, width: f64, height: f64, shift: bool, ctrl: bool, meta: bool },
    #[dsl(key = "canvas-pointer-move")]
    CanvasPointerMove { x: f64, y: f64, width: f64, height: f64 },
    #[dsl(key = "canvas-pointer-up")]
    CanvasPointerUp { x: f64, y: f64, width: f64, height: f64, shift: bool, ctrl: bool, meta: bool },
    #[dsl(key = "canvas-double-click")]
    CanvasDoubleClick,
    #[dsl(key = "canvas-commit-draft")]
    CanvasCommitDraft,
    #[dsl(key = "canvas-escape")]
    CanvasEscape,
}
//#endregion 🔖️DrawCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use draw::{DrawDocument, DRAW_DOCUMENT_SCHEMA};
    use draw_engine::{create_draw_shape_layer_rect, default_draw_document, layer_id};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = default_draw_document("doc-text-test", None);
        let operation = DrawOperation::AddLayer { parent_id: None, index: Some(document.layers.len()), layer: Box::new(create_draw_shape_layer_rect("Op Binary Test")) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_a_store_with_an_applied_operation() {
        let initial = default_draw_document("doc-text-test", None);
        let envelope = store::create_document_envelope::<DrawDocument, DrawOperation>(DRAW_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
        let mut doc_store = store::DocumentStore::new(envelope);
        let layer = create_draw_shape_layer_rect("Added Rect");
        let layer_id_value = layer_id(&layer).to_string();
        doc_store
            .dispatch(store::DocumentCommand::Apply { operations: vec![DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(layer) }], description: Some("add rect".into()) })
            .expect("apply add layer");
        doc_store
            .dispatch(store::DocumentCommand::Apply { operations: vec![DrawOperation::SetLayerOpacity { layer_id: layer_id_value, opacity: 0.5 }], description: Some("set opacity".into()) })
            .expect("apply set opacity");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
        store::test_support::assert_live_equals_replay(&doc_store);
    }

    #[test]
    fn draw_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetDocument { document: default_draw_document("cmd-doc", None) });
        store::test_support::assert_op_line_round_trip(&DrawCommand::CommitDocument { document: default_draw_document("cmd-doc-2", None) });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetFixtureJson { json: "{}".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetActiveExample { example_id: "semio".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetSelectedOpacity { value: 0.5 });
        store::test_support::assert_op_line_round_trip(&DrawCommand::EngagementSubmit { value: Some("Renamed \"layer\"".into()) });
        store::test_support::assert_op_line_round_trip(&DrawCommand::EngagementSubmit { value: None });
        store::test_support::assert_op_line_round_trip(&DrawCommand::AddLayer { kind: "shape:rect".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::DropLayerKind { kind: "path".into(), target_row_id: "draw-play-layers".into(), drop_position: "inside".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::MoveLayer { layer_id: "layer-1".into(), target_row_id: "draw-play-layers".into(), drop_position: "after".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::DeleteLayer { layer_id: "layer-1".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::DuplicateLayer { layer_id: "layer-1".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::ToggleLayerVisible { layer_id: "layer-1".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::CombineBoolean { operation: "union".into(), ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&DrawCommand::PatchLayer { layer_id: "layer-1".into(), field: "opacity".into(), value: "0.4".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "blendMode".into(), value: "\"multiply\"".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetActiveUtility { utility_id: "pen".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetCamera { camera: DrawCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetCameraZoom { value: 2.0 });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetHover { id: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetHover { id: None });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SelectAll);
        store::test_support::assert_op_line_round_trip(&DrawCommand::ClearSelection);
        store::test_support::assert_op_line_round_trip(&DrawCommand::EngagementInput { value: "typing".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&DrawCommand::CanvasPointerDown { x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: true, ctrl: false, meta: false });
        store::test_support::assert_op_line_round_trip(&DrawCommand::CanvasPointerMove { x: 1.0, y: 2.0, width: 800.0, height: 600.0 });
        store::test_support::assert_op_line_round_trip(&DrawCommand::CanvasPointerUp { x: 1.0, y: 2.0, width: 800.0, height: 600.0, shift: false, ctrl: true, meta: false });
        store::test_support::assert_op_line_round_trip(&DrawCommand::CanvasDoubleClick);
        store::test_support::assert_op_line_round_trip(&DrawCommand::CanvasCommitDraft);
        store::test_support::assert_op_line_round_trip(&DrawCommand::CanvasEscape);
    }

    #[test]
    fn draw_command_op_binary_round_trips() {
        let command = DrawCommand::AddLayer { kind: "path".into() };
        store::test_support::assert_op_text_binary_equivalence(&command);
    }
}
//#endregion 🧪️Tests

//! ⚖️ Raster app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use raster::RasterCamera;
use raster_op::RasterOperation;

/// 📦️ Encodes a `RasterOperation` to its binary command form.
pub fn encode_op(operation: &RasterOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `RasterOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<RasterOperation, protocol::ProtocolError> {
    RasterOperation::decode_op(bytes)
}

//#region 🔖️RasterCommand
/// 🎯️ B1: `RasterPlayApp::Command` — the SOLE dispatch surface for raster's own behavior, covering
/// every action `create_raster_app` declares. `PatchLayer`/`PatchLayers` carry `value` as JSON TEXT
/// (parsed via `serde_json::from_str`, falling back to a plain JSON string) — mirrors
/// `draw_protocol::DrawCommand`'s identical shape.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
pub enum RasterCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "set-document")]
    SetDocument {
        #[dsl(block)]
        document: raster::RasterProjection,
    },
    #[dsl(key = "active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "add-layer")]
    AddLayer { kind: String },
    #[dsl(key = "drop-layer-kind")]
    DropLayerKind { kind: String },
    #[dsl(key = "set-layer-visible")]
    SetLayerVisible { layer_id: String, visible: Option<bool> },
    #[dsl(key = "toggle-layer-visible")]
    ToggleLayerVisible { layer_id: String },
    #[dsl(key = "delete-layer")]
    DeleteLayer { layer_id: String },
    #[dsl(key = "duplicate-layer")]
    DuplicateLayer { layer_id: String },
    #[dsl(key = "patch-layer")]
    PatchLayer { layer_id: String, field: String, value: String },
    #[dsl(key = "patch-layers")]
    PatchLayers { layer_ids: Vec<String>, field: String, value: String },
    #[dsl(key = "move-layer")]
    MoveLayer { layer_id: String, target_row_id: String, drop_position: String },

    // 👁️ Config-only — emit `config_operations`, never document operations.
    #[dsl(key = "set-selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "set-hover")]
    SetHover { id: Option<String> },
    #[dsl(key = "select-all")]
    SelectAll,
    #[dsl(key = "brush-size")]
    SetBrushSize { value: f64 },
    #[dsl(key = "brush-opacity")]
    SetBrushOpacity { value: f64 },
    #[dsl(key = "composite-viewport")]
    SetCompositeViewport { width: f64, height: f64 },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: RasterCamera,
    },
    #[dsl(key = "camera-zoom")]
    SetCameraZoom { zoom: f64 },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}
//#endregion 🔖️RasterCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use raster::{RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = raster_engine::empty_raster_document();
        let operation = RasterOperation::AddLayer {
            parent_id: None,
            index: document.layers.len(),
            layer: Box::new(RasterLayerNode::Pixel {
                id: "op-binary-test".into(),
                name: "Op Binary Test".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: None,
                width: Some(64),
                height: Some(64),
                image_key: None,
            }),
        };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn raster_document_text_round_trips_store_with_applied_operation() {
        use raster::{RasterLayerNode, RasterProjection, RasterTransform};

        let envelope = store::create_document_envelope::<RasterProjection, RasterOperation>(RASTER_DOCUMENT_SCHEMA, "doc-text-test", raster_engine::empty_raster_document(), None);
        let mut store = store::DocumentStore::new(envelope);
        store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![RasterOperation::AddLayer {
                    parent_id: None,
                    index: 1,
                    layer: Box::new(RasterLayerNode::Adjustment {
                        id: "adjust-text".into(),
                        name: "Levels".into(),
                        visible: true,
                        opacity: 1.0,
                        blend_mode: "normal".into(),
                        transform: RasterTransform::default(),
                        adjustment_kind: "levels".into(),
                        params: std::collections::BTreeMap::new(),
                    }),
                }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }

    #[test]
    fn raster_command_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetDocument { document: raster_engine::empty_raster_document() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetActiveExample { example_id: "semio".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::AddLayer { kind: "pixel".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::DropLayerKind { kind: "group".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetLayerVisible { layer_id: "l1".into(), visible: Some(true) });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetLayerVisible { layer_id: "l1".into(), visible: None });
        store::test_support::assert_op_line_round_trip(&RasterCommand::ToggleLayerVisible { layer_id: "l1".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::DeleteLayer { layer_id: "l1".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::DuplicateLayer { layer_id: "l1".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::PatchLayer { layer_id: "l1".into(), field: "opacity".into(), value: "0.4".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::PatchLayers { layer_ids: vec!["a".into(), "b".into()], field: "name".into(), value: "Renamed".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::MoveLayer { layer_id: "l1".into(), target_row_id: "raster-play-layers".into(), drop_position: "after".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetSelection { ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetHover { id: Some("a".into()) });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetHover { id: None });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SelectAll);
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetBrushSize { value: 40.0 });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetBrushOpacity { value: 0.5 });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetCompositeViewport { width: 640.0, height: 480.0 });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetCamera { camera: RasterCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetCameraZoom { zoom: 2.0 });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetActiveUtility { utility_id: "paintBrush".into() });
        store::test_support::assert_op_line_round_trip(&RasterCommand::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn raster_command_op_binary_round_trips() {
        store::test_support::assert_op_text_binary_equivalence(&RasterCommand::AddLayer { kind: "pixel".into() });
    }
}
//#endregion 🧪️Tests

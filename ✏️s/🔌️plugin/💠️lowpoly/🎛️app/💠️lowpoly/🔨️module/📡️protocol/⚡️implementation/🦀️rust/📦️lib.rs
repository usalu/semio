//! ⚖️ Lowpoly app — binary command protocol surface + laws (constitutional: protocol).

use lowpoly_op::LowpolyOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};

/// 📦️ Encodes a `LowpolyOperation` to its binary command form.
pub fn encode_op(operation: &LowpolyOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LowpolyOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<LowpolyOperation, protocol::ProtocolError> {
    LowpolyOperation::decode_op(bytes)
}

//#region 🔖️LowpolyCommand
/// 🎯️ B1: `LowpolyPlayApp::Command` — the SOLE dispatch surface for lowpoly's own behavior, covering
/// every declared action (`create_lowpoly_app`'s `.operation`/`.view_action`/`.action_with` calls).
/// Mirrors `shooting_op::ShootingCommand`'s pattern exactly (see that crate's `📡️protocol` for the
/// pilot). Field shapes mirror each action's real `args` object exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum LowpolyCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-primitive")]
    AddPrimitive { kind: Option<String> },
    #[dsl(key = "patch-object")]
    PatchObject { object_id: String, field: String, value_json: Option<String> },
    #[dsl(key = "extrude")]
    Extrude { extrude_distance: Option<f32> },
    #[dsl(key = "inset")]
    Inset { inset_amount: Option<f32> },
    #[dsl(key = "bevel")]
    Bevel { bevel_amount: Option<f32>, bevel_segments: Option<u32> },
    #[dsl(key = "loop-cut")]
    LoopCut { loop_cuts: Option<u32> },
    #[dsl(key = "subdivide")]
    Subdivide,
    #[dsl(key = "triangulate")]
    Triangulate,
    #[dsl(key = "mirror")]
    Mirror { axis: Option<String> },
    #[dsl(key = "decimate")]
    Decimate { decimate_ratio: Option<f32> },
    #[dsl(key = "flip-faces")]
    FlipFaces { face_ids: Vec<u32> },
    #[dsl(key = "merge")]
    Merge,
    #[dsl(key = "dissolve")]
    Dissolve,
    #[dsl(key = "snap")]
    Snap,
    #[dsl(key = "toggle-smooth")]
    ToggleSmooth,
    #[dsl(key = "unwrap-active")]
    UnwrapActive,
    #[dsl(key = "mark-uv-seam")]
    MarkUvSeam { seam: Option<bool>, edge_ids: Option<Vec<u32>> },
    #[dsl(key = "clear-seam")]
    ClearSeam,
    #[dsl(key = "translate-selection")]
    TranslateSelection { mode: Option<String>, ids: Option<Vec<u32>>, dx: f32, dy: f32, dz: f32 },
    #[dsl(key = "rotate-selection")]
    RotateSelection { mode: Option<String>, ids: Option<Vec<u32>>, ax: f32, ay: f32, az: f32, angle: f32 },
    #[dsl(key = "scale-selection")]
    ScaleSelection { mode: Option<String>, ids: Option<Vec<u32>>, sx: f32, sy: f32, sz: f32 },
    #[dsl(key = "add-paint-layer")]
    AddPaintLayer { object_id: Option<String>, name: Option<String> },
    #[dsl(key = "paint-stroke-end")]
    PaintStrokeEnd,
    #[dsl(key = "paint-fill")]
    PaintFill { object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32> },
    #[dsl(key = "fill-bucket")]
    FillBucket { object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32> },
    #[dsl(key = "transform-end")]
    TransformEnd,
    #[dsl(key = "set-projection-json")]
    SetProjectionJson { json: String },
    #[dsl(key = "set-fixture-json")]
    SetFixtureJson { json: String },
    #[dsl(key = "engagement-submit")]
    EngagementSubmit { value: Option<String> },

    // 👁️ Config-only (was ephemeral `LowpolyPlayRuntime` state) — emit `config_operations`, never
    // document operations.
    #[dsl(key = "set-active-object")]
    SetActiveObject { object_id: String },
    #[dsl(key = "set-selection")]
    SetSelection { mode: String, ids: Vec<u32> },
    #[dsl(key = "toggle-selection-kind")]
    ToggleSelectionKind { kind: String },
    #[dsl(key = "toggle-selection-target")]
    ToggleSelectionTarget { object_id: String, mode: String, id: u32, merge: String },
    #[dsl(key = "set-active-paint-layer")]
    SetActivePaintLayer { layer_index: u32 },
    #[dsl(key = "set-utility-param")]
    SetUtilityParam { key: String, value_json: String },
    #[dsl(key = "engagement-input")]
    EngagementInput { value: String },
    #[dsl(key = "toggle-show-edges")]
    ToggleShowEdges,
    #[dsl(key = "toggle-sun")]
    ToggleSun,
    #[dsl(key = "set-sun-azimuth")]
    SetSunAzimuth { value: f64 },
    #[dsl(key = "set-sun-elevation")]
    SetSunElevation { value: f64 },
    #[dsl(key = "set-sun-intensity")]
    SetSunIntensity { value: f64 },
    #[dsl(key = "set-selection-method")]
    SetSelectionMethod { value: String },
    #[dsl(key = "set-selection-mode-default")]
    SetSelectionModeDefault { value: String },
    #[dsl(key = "set-camera")]
    SetCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
    #[dsl(key = "world-select")]
    WorldSelect { ids: Vec<String>, merge: String },
    #[dsl(key = "world-hover")]
    WorldHover { object_id: Option<String> },
    #[dsl(key = "set-hover")]
    SetHover { object_id: Option<String>, mode: Option<String>, id: Option<u32> },
    #[dsl(key = "world-pick")]
    WorldPick { granularity: String, merge: String, id: Option<u32> },
    #[dsl(key = "paint-stroke-begin")]
    PaintStrokeBegin,
    #[dsl(key = "paint-sample")]
    PaintSample { object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32> },
    #[dsl(key = "paint-stroke")]
    PaintStroke { object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32> },
    #[dsl(key = "paint-at")]
    PaintAt { object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32> },
    #[dsl(key = "canvas-pointer-down")]
    CanvasPointerDown { object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32> },
    #[dsl(key = "canvas-pointer-move")]
    CanvasPointerMove { object_id: Option<String>, u: Option<f32>, v: Option<f32>, x: Option<f32>, y: Option<f32> },
    #[dsl(key = "transform-begin")]
    TransformBegin,
    #[dsl(key = "set-active-utility")]
    SetActiveUtility { utility_id: String },
}
//#endregion 🔖️LowpolyCommand

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = LowpolyOperation::ObjectsMove { id: "obj-1".into(), to_index: 2 };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_after_applying_an_operation() {
        use lowpoly::LOWPOLY_DOCUMENT_SCHEMA;

        let projection = lowpoly_engine::default_projection();
        let object_id = projection.objects[0].id.clone();
        let envelope = store::create_document_envelope::<lowpoly::LowpolyProjection, LowpolyOperation>(LOWPOLY_DOCUMENT_SCHEMA, "test-doc", projection, None);
        let mut doc_store = store::DocumentStore::new(envelope);
        let operation = LowpolyOperation::PatchPaintLayer { object_id, index: 0, patch: lowpoly_op::LowpolyPaintLayerPatch { name: Some("Renamed Layer".into()), visible: None, opacity: None, blend_mode: None } };
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![operation], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests

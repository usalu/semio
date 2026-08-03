//! ⚖️ Cad app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! the `CadEnvelope`/`CadStore` type aliases and the WASM VCS bridge — both need `CadOperation`
//! (from `cad_document_op`) alongside `CadScene` (from `cad_document`), so this is the first
//! constitutional crate in the stack where that pairing is available.

use cad_document::{empty_cad_projection, CadCamera, CadScene, CAD_DOCUMENT_SCHEMA};
use cad_document_op::CadOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `CadOperation` to its binary command form.
pub fn encode_op(operation: &CadOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `CadOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<CadOperation, protocol::ProtocolError> {
    CadOperation::decode_op(bytes)
}

//#region 🔖️CadCommand
/// 🎯️ WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2: `CadPlayApp::Command` — the SOLE dispatch surface for
/// cad's own behavior (mirrors `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`'s shooting
/// pilot, `shooting_protocol::ShootingCommand`), covering every declared action in `create_cad_app`'s
/// static manifest. Field shapes mirror each action's real JSON `args` object, typed instead of loose
/// `serde_json::Value`: numeric/boolean edits that used to accept either an absolute JSON value or a
/// JSON delta (`cad_ui::resolve_number_edit`) become a plain string `value` (parsed per-field, same as
/// `PatchShots`/`PatchAssets` already do in `shooting_protocol`) plus a typed `delta: Option<f64>`;
/// window/pane targeting that used to read the host-pushed `ViewState.window_id` (deleted by B1) is now
/// an explicit `pane` field (the pane-suffix convention `cad_ui::cad_pane_id_from_suffix` already uses),
/// defaulting to the Shape pane exactly like the pre-B1 fallback did.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum CadCommand {
    // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
    #[dsl(key = "add-object")]
    AddObject { typology: Option<String> },
    #[dsl(key = "patch-object")]
    PatchObject { object_id: String, field: String, value: Option<String>, delta: Option<f64> },
    #[dsl(key = "patch-selection")]
    PatchSelection { object_ids: Vec<String>, field: String, value: Option<String>, delta: Option<f64> },
    #[dsl(key = "delete-object")]
    DeleteObject { object_id: String },
    #[dsl(key = "duplicate-object")]
    DuplicateObject { object_id: String },
    #[dsl(key = "add-node")]
    AddNode { kind: String },
    #[dsl(key = "rename-node")]
    RenameNode { node_id: String, value: String },
    #[dsl(key = "translate-selection")]
    TranslateSelection { object_ids: Vec<String>, dx: f64, dy: f64, dz: f64 },
    #[dsl(key = "rotate-selection")]
    RotateSelection { object_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    #[dsl(key = "scale-selection")]
    ScaleSelection { object_ids: Vec<String>, sx: f64, sy: f64, sz: f64 },
    #[dsl(key = "apply-transformation")]
    ApplyTransformation { qid: String },
    #[dsl(key = "import-cad-file")]
    ImportCadFile { name: String, payload: String },
    #[dsl(key = "patch-cad-play-reference")]
    PatchCadPlayReference { model_definition_id: String, reference_id: String, field: String, value: Option<String>, delta: Option<f64> },
    #[dsl(key = "engagement-submit")]
    EngagementSubmit { pane: Option<String> },
    #[dsl(key = "focus-model-definition")]
    FocusModelDefinition { model_definition_id: String },
    #[dsl(key = "set-active-example")]
    SetActiveExample { example_id: String },
    #[dsl(key = "world-pointer-down")]
    WorldPointerDown { pane: Option<String>, surface_id: Option<String>, x: Option<f64>, y: Option<f64>, z: Option<f64> },

    // 👁️ Config-only (was ephemeral `CadPlayRuntime` state) — emit `config_operations`, never document
    // operations.
    #[dsl(key = "camera")]
    SetCamera { pane: Option<String>, #[dsl(block)] camera: CadCamera },
    /// 🧮️ `value_str`/`value_num` mirror `semio_framework_plugin::apply_world3d_projection_action`'s
    /// dual-typed JSON `value` key (a select field like `orthographicView` sends a string, a slider
    /// param sends a number) — split into two typed optionals instead of one loose `serde_json::Value`.
    #[dsl(key = "projection")]
    SetProjection { pane: Option<String>, field: Option<String>, value_str: Option<String>, value_num: Option<f64>, param: Option<String> },
    #[dsl(key = "projection-param")]
    SetProjectionParam { pane: Option<String>, field: Option<String>, value_str: Option<String>, value_num: Option<f64>, param: Option<String> },
    #[dsl(key = "dislocate-option")]
    SetDislocateOption { pane: Option<String>, option: String, pressed: Option<bool> },
    #[dsl(key = "set-selection")]
    SetSelection { mode: String, ids: Vec<u32>, object_id: Option<String>, merge: String },
    #[dsl(key = "set-node-selection")]
    SetNodeSelection { node_ids: Vec<String> },
    #[dsl(key = "world-select")]
    WorldSelect { ids: Vec<String>, merge: String },
    #[dsl(key = "world-hover")]
    WorldHover { object_id: Option<String> },
    #[dsl(key = "set-hover")]
    SetHover { object_id: Option<String>, mode: Option<String>, id: Option<u32> },
    #[dsl(key = "world-pick")]
    WorldPick { id: Option<u64>, merge: String, granularity: String, object_id: Option<String>, surface_id: Option<String>, pane: Option<String> },
    #[dsl(key = "selection-method")]
    SetSelectionMethod { method: String },
    #[dsl(key = "reference-selection")]
    SetReferenceSelection { pane: Option<String>, model_definition_id: Option<String>, reference_id: Option<String> },
    #[dsl(key = "reference-hover")]
    ReferenceHover { reference_id: Option<String> },
    #[dsl(key = "engagement-input")]
    EngagementInput { value: String, pane: Option<String> },
    #[dsl(key = "engagement-possible-select")]
    EngagementPossibleSelect { pane: Option<String>, possible_id: String },
    #[dsl(key = "engagement-repeat-last")]
    EngagementRepeatLast { pane: Option<String> },
    #[dsl(key = "engagement-abort")]
    EngagementAbort,
    #[dsl(key = "world-pointer-move")]
    WorldPointerMove { x: Option<f64>, y: Option<f64>, z: Option<f64> },
    #[dsl(key = "set-primitive-selection")]
    SetPrimitiveSelection { object_id: String, primitive_id: Option<String>, kind: Option<String> },
    #[dsl(key = "toggle-sun")]
    ToggleSun,
    #[dsl(key = "sun-azimuth")]
    SetSunAzimuth { value: f64 },
    #[dsl(key = "sun-elevation")]
    SetSunElevation { value: f64 },
    #[dsl(key = "sun-intensity")]
    SetSunIntensity { value: f64 },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "terminology")]
    SetTerminology { value: String },

    // 🐚️ Shell effects — export/import round-trips through the host, no operations either way.
    #[dsl(key = "save-selected")]
    SaveSelected,
    #[dsl(key = "save-in-play")]
    SaveInPlay,
    #[dsl(key = "save-current")]
    SaveCurrent { format: Option<String> },
    #[dsl(key = "load-raw-request")]
    LoadRawRequest,
}
//#endregion 🔖️CadCommand

//#region 🔖️Store
pub type CadEnvelope = DocumentEnvelope<CadScene, CadOperation>;
pub type CadStore = DocumentStore<CadScene, CadOperation>;
//#endregion 🔖️Store

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct CadDocumentVcs {
        store: RefCell<CadStore>,
    }

    #[wasm_bindgen]
    impl CadDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<CadDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: CadEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    CadStore::new(envelope)
                }
                None => CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cad_document::{empty_cad_projection, CadNode, CadObject, CadPaneId, CadPrimitiveSlot, CAD_DOCUMENT_SCHEMA};
    use cad_document_op::CadOperation;
    use store::{create_document_envelope, DocumentCommand};

    #[test]
    fn cad_command_op_text_round_trips_a_representative_sample() {
        store::test_support::assert_op_line_round_trip(&CadCommand::AddObject { typology: Some("spatial.shape.primitive.box".into()) });
        store::test_support::assert_op_line_round_trip(&CadCommand::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: Some(1.5) });
        store::test_support::assert_op_line_round_trip(&CadCommand::WorldSelect { ids: vec!["object-1".into(), "object-2".into()], merge: "replace".into() });
        store::test_support::assert_op_line_round_trip(&CadCommand::SetCamera { pane: Some("building".into()), camera: CadCamera::default() });
        store::test_support::assert_op_line_round_trip(&CadCommand::SetActiveUtility { utility_id: "rotate".into() });
        store::test_support::assert_op_line_round_trip(&CadCommand::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&CadCommand::EngagementAbort);
        store::test_support::assert_op_line_round_trip(&CadCommand::SaveCurrent { format: Some("step".into()) });
    }

    #[test]
    fn cad_command_op_binary_round_trips_and_agrees_with_text() {
        let command = CadCommand::TranslateSelection { object_ids: vec!["object-1".into()], dx: 1.0, dy: -2.0, dz: 3.5 };
        store::test_support::assert_op_text_binary_equivalence(&command);
        let bytes = command.encode_op().expect("encode");
        assert_eq!(CadCommand::decode_op(&bytes).expect("decode"), command);
    }

    #[test]
    fn cad_projection_defaults() {
        let store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        assert_eq!(store.projection().expect("projection").id, "cad");
    }

    #[test]
    fn add_object_round_trips_through_store() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        let object = CadObject {
            id: "object-1".into(),
            label: "Box".into(),
            typology: "spatial.shape.box".into(),
            visible: true,
            locked: false,
            origin: [0.0, 0.0, 0.0],
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            mesh_url: None,
            extent: None,
            solid_handle: None,
            primitives: vec![CadPrimitiveSlot { slot: "solid".into(), primitive_id: "solid-1".into(), kind: "solid".into() }],
        };
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::AddObject { pane: CadPaneId::Shape, object }], description: None }).expect("apply");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].primitives[0].kind, "solid");
    }

    #[test]
    fn translate_objects_updates_origin() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![CadOperation::AddObject {
                    pane: CadPaneId::Shape,
                    object: CadObject {
                        id: "object-1".into(),
                        label: "Box".into(),
                        typology: "spatial.shape.box".into(),
                        visible: true,
                        locked: false,
                        origin: [1.0, 2.0, 3.0],
                        orientation: None,
                        scale: None,
                        mesh_url: None,
                        extent: None,
                        solid_handle: None,
                        primitives: Vec::new(),
                    },
                }],
                description: None,
            })
            .expect("apply");
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::TranslateObjects { object_ids: vec!["object-1".into()], dx: 1.0, dy: -1.0, dz: 0.5 }], description: None }).expect("translate");
        let scene = store.projection().expect("projection");
        assert_eq!(scene.objects[0].origin, [2.0, 1.0, 3.5]);
    }

    #[test]
    fn set_scene_replaces_projection_and_inverts() {
        let mut store = CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        let mut replacement = empty_cad_projection();
        replacement.id = "replaced".into();
        replacement.nodes.push(CadNode { id: "node-1".into(), label: "Root".into(), kind: "group".into() });
        store.dispatch(DocumentCommand::Apply { operations: vec![CadOperation::SetScene { scene: Box::new(replacement) }], description: None }).expect("set scene");
        assert_eq!(store.projection().expect("projection").id, "replaced");
        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").id, "cad");
        assert!(store.projection().expect("projection").nodes.is_empty());
    }

}
//#endregion 🧪️Tests

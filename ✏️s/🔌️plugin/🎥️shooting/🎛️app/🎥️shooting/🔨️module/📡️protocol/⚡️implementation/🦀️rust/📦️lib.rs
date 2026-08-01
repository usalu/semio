//! ⚖️ Shooting app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! the `ShootingEnvelope`/`ShootingStore` type aliases and the WASM VCS bridge — both need
//! `ShootingOperation` (from `shooting_op`) alongside `ShootingFixture` (from `shooting`), so this is
//! the first constitutional crate in the stack where that pairing is available.
//!
//! 🎯️ Also hosts `ShootingCommand` — the app-engine `AppCommand::Command` binary command envelope
//! (`HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`, Wave 1 pilot conversion). One variant
//! per a representative subset of `create_shooting_app`'s real declared actions (the ones with
//! non-trivial typed args); every OTHER declared action keeps working unchanged through the legacy
//! `{kind,name,args}` wire-value envelope fallback `VcsDocumentApp::dispatch_command_frame` falls back
//! to whenever `DocumentApp::handle_typed_command` returns `None` — see `shooting_ui`'s
//! `ShootingPlayApp::handle_typed_command` for the dispatch and exactly which actions are covered.

use shooting::ShootingFixture;
use shooting_op::ShootingOperation;
use protocol::OpBinary;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `ShootingOperation` to its binary command form.
pub fn encode_op(operation: &ShootingOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `ShootingOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<ShootingOperation, protocol::ProtocolError> {
    ShootingOperation::decode_op(bytes)
}

//#region 🔖️ShootingCommand
/// 🎯️ Typed binary command envelope for the shooting app's app-engine channel — one variant per a
/// representative subset of `create_shooting_app`'s real declared actions (see the module doc). Field
/// shapes mirror each action's real `args` object exactly (`shooting_ui`'s `handle_action` match arms
/// are the ground truth): e.g. `SetActiveShot.shot_id` mirrors `"setActiveShot"`'s `value`/`id` arg,
/// `AddShot.{format,shape}` mirrors its `.action_args` defaults. `#[derive(dsl::DslOps)]` gives this a
/// binary (`OpBinary`) AND text (`OpText`) codec, matching `ShootingOperationDsl`'s (`shooting_op`)
/// derive/attribute conventions exactly, even though this enum is never dispatched through
/// `store::DocumentCommand` (it is not a `protocol::Operation` — no `diff`/`backwards` — purely a
/// command-channel wire codec).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum ShootingCommand {
    #[dsl(key = "active-shot")]
    SetActiveShot { shot_id: Option<String> },
    #[dsl(key = "active-asset")]
    SetActiveAsset { asset_id: Option<String> },
    #[dsl(key = "sun-azimuth")]
    SetSunAzimuth { value: f64 },
    #[dsl(key = "sun-elevation")]
    SetSunElevation { value: f64 },
    #[dsl(key = "sun-intensity")]
    SetSunIntensity { value: f64 },
    #[dsl(key = "ambient-intensity")]
    SetAmbientIntensity { value: f64 },
    #[dsl(key = "material-roughness")]
    SetMaterialRoughness { value: f64 },
    #[dsl(key = "shadow-enabled")]
    SetShadowEnabled { value: bool },
    #[dsl(key = "toggle-sun")]
    ToggleSun { value: bool },
    #[dsl(key = "add-shot")]
    AddShot { format: String, shape: String },
    #[dsl(key = "add-asset")]
    AddAsset { format: String },
    #[dsl(key = "translate-selection")]
    TranslateSelection { asset_ids: Vec<String>, dx: f64, dy: f64, dz: f64 },
    #[dsl(key = "rotate-selection")]
    RotateSelection { asset_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64 },
    #[dsl(key = "scale-selection")]
    ScaleSelection { asset_ids: Vec<String>, sx: f64, sy: f64, sz: f64 },
    #[dsl(key = "set-selection")]
    SetSelection { shot_ids: Vec<String>, asset_ids: Vec<String> },
    #[dsl(key = "selection-method")]
    SetSelectionMethod { method: String },
    #[dsl(key = "reset-fixture")]
    ResetFixture,
}
//#endregion 🔖️ShootingCommand

//#region 🔖️Store
pub type ShootingEnvelope = DocumentEnvelope<ShootingFixture, ShootingOperation>;
pub type ShootingStore = DocumentStore<ShootingFixture, ShootingOperation>;
//#endregion 🔖️Store

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use shooting::SHOOTING_FIXTURE_SCHEMA;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct ShootingDocumentVcs {
        store: RefCell<ShootingStore>,
    }

    #[wasm_bindgen]
    impl ShootingDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<ShootingDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: ShootingEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    ShootingStore::new(envelope)
                }
                None => ShootingStore::new(store::create_document_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", shooting::empty_shooting_fixture(), None)),
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
    use protocol::CollectionOperation;
    use shooting::{ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingShot, SHOOTING_FIXTURE_SCHEMA};
    use store::DocumentCommand;

    fn sample_asset(id: &str) -> ShootingAsset {
        ShootingAsset { id: id.into(), name: format!("Asset {id}"), url: format!("/mesh/{id}.glb"), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None }
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = ShootingOperation::SetActiveShot { shot_id: Some("s1".into()) };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn shooting_projection_round_trip() {
        let mut store = ShootingStore::new(store::create_document_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", shooting::empty_shooting_fixture(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![ShootingOperation::Assets(CollectionOperation::Add { id: "a1".into(), item: sample_asset("a1"), at: 0 })], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").assets.len(), 1);
    }

    /// 🎥️ The free/live viewport camera is session-only runtime state now (never a VCS-tracked document
    /// field — see `ShootingPlayRuntime::camera` in the ui crate), so this no longer exercises `SetCamera`
    /// (removed); it instead seeds a shot that references a saved camera and drags *that* pose via
    /// `SetShotCamera`, preserving the original "coalesced drag produces one edit" store-level intent.
    #[test]
    fn coalesced_camera_drag_produces_one_edit() {
        let mut store = ShootingStore::new(store::create_document_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", shooting::empty_shooting_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![
                    ShootingOperation::SavedCameras(CollectionOperation::Add { id: "cam1".into(), item: ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera::default() }, at: 0 }),
                    ShootingOperation::Shots(CollectionOperation::Add {
                        id: "s1".into(),
                        item: ShootingShot { id: "s1".into(), label: "Shot".into(), width: 256, height: 256, format: "png".into(), shape: "rectangle".into(), background: None, camera_id: Some("cam1".into()) },
                        at: 0,
                    }),
                ],
                description: None,
            })
            .expect("seed saved camera + referencing shot");
        let edits_before = store.envelope().vcs.edits.len();
        store.dispatch(DocumentCommand::AmendLast { operations: vec![ShootingOperation::SetShotCamera { shot_id: "s1".into(), camera: ShootingCamera { position: [1.0, 0.0, 0.0], ..Default::default() } }], coalesce_key: Some("camera".into()) }).expect("first drag tick");
        store.dispatch(DocumentCommand::AmendLast { operations: vec![ShootingOperation::SetShotCamera { shot_id: "s1".into(), camera: ShootingCamera { position: [2.0, 0.0, 0.0], ..Default::default() } }], coalesce_key: Some("camera".into()) }).expect("second drag tick");
        assert_eq!(store.envelope().vcs.edits.len(), edits_before + 1, "coalesced drag must produce exactly one edit");
        assert_eq!(store.projection().expect("projection").saved_cameras[0].camera.position, [2.0, 0.0, 0.0]);
    }

    #[test]
    fn shooting_document_text_round_trips_store_with_applied_operation() {
        let mut store = ShootingStore::new(store::create_document_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", shooting::empty_shooting_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![ShootingOperation::Assets(CollectionOperation::Add { id: "a1".into(), item: sample_asset("a1"), at: 0 })],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests

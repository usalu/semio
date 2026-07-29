//! ⚖️ Shooting app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! the `ShootingEnvelope`/`ShootingStore` type aliases and the WASM VCS bridge — both need
//! `ShootingOperation` (from `shooting_op`) alongside `ShootingFixture` (from `shooting`), so this is
//! the first constitutional crate in the stack where that pairing is available.

use shooting::{ShootingFixture, SHOOTING_FIXTURE_SCHEMA};
use shooting_op::ShootingOperation;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦 Encodes a `ShootingOperation` to its binary command form.
pub fn encode_op(operation: &ShootingOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `ShootingOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<ShootingOperation, protocol::ProtocolError> {
    ShootingOperation::decode_op(bytes)
}

//#region 🔖Store
pub type ShootingEnvelope = DocumentEnvelope<ShootingFixture, ShootingOperation>;
pub type ShootingStore = DocumentStore<ShootingFixture, ShootingOperation>;
//#endregion 🔖Store

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
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
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::CollectionOperation;
    use shooting::{ShootingAsset, ShootingCamera};
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

    #[test]
    fn coalesced_camera_drag_produces_one_edit() {
        let mut store = ShootingStore::new(store::create_document_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", shooting::empty_shooting_fixture(), None));
        store.dispatch(DocumentCommand::AmendLast { operations: vec![ShootingOperation::SetCamera { camera: ShootingCamera { position: [1.0, 0.0, 0.0], ..Default::default() } }], coalesce_key: Some("camera".into()) }).expect("first drag tick");
        store.dispatch(DocumentCommand::AmendLast { operations: vec![ShootingOperation::SetCamera { camera: ShootingCamera { position: [2.0, 0.0, 0.0], ..Default::default() } }], coalesce_key: Some("camera".into()) }).expect("second drag tick");
        assert_eq!(store.envelope().vcs.edits.len(), 1, "coalesced drag must produce exactly one edit");
        assert_eq!(store.projection().expect("projection").camera.position, [2.0, 0.0, 0.0]);
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
//#endregion 🧪Tests

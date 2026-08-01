//! ⚖️ Cad app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! the `CadEnvelope`/`CadStore` type aliases and the WASM VCS bridge — both need `CadOperation`
//! (from `cad_document_op`) alongside `CadScene` (from `cad_document`), so this is the first
//! constitutional crate in the stack where that pairing is available.

use cad_document::CadScene;
use cad_document_op::CadOperation;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦️ Encodes a `CadOperation` to its binary command form.
pub fn encode_op(operation: &CadOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `CadOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<CadOperation, protocol::ProtocolError> {
    CadOperation::decode_op(bytes)
}

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
    use cad_document::{CadNode, CadObject, CadPaneId, CadPrimitiveSlot};
    use cad_document_op::CadOperation;
    use store::{create_document_envelope, DocumentCommand};

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

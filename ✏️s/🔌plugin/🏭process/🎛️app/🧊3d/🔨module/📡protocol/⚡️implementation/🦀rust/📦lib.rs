//! ⚖️ Process 3d app — binary command protocol surface + laws (constitutional: protocol). Also hosts
//! the `Process3dEnvelope`/`Process3dStore` type aliases and the WASM VCS bridge — both need
//! `Process3dOperation` (from `process_3d_op`) alongside `Process3dDocument` (from `process_3d`), so
//! this is the first constitutional crate in the stack where that pairing is available.

use process_3d::Process3dDocument;
use process_3d_op::Process3dOperation;
use protocol::OpBinary;
use store::{DocumentEnvelope, DocumentStore};

/// 📦 Encodes a `Process3dOperation` to its binary command form.
pub fn encode_op(operation: &Process3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `Process3dOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Process3dOperation, protocol::ProtocolError> {
    Process3dOperation::decode_op(bytes)
}

//#region 🔖Store
pub type Process3dEnvelope = DocumentEnvelope<Process3dDocument, Process3dOperation>;
pub type Process3dStore = DocumentStore<Process3dDocument, Process3dOperation>;
//#endregion 🔖Store

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use process_3d::PROCESS_3D_SCHEMA;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Process3dDocumentVcs {
        store: RefCell<Process3dStore>,
    }

    #[wasm_bindgen]
    impl Process3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Process3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Process3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Process3dStore::new(envelope)
                }
                None => Process3dStore::new(create_document_envelope(PROCESS_3D_SCHEMA, "process3d", process_3d::empty_process3d_projection(), None)),
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

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use process_3d::{empty_process3d_projection, Pose, ProcessMeasure, ProcessStep, ProcessStepPatch, SolidSpec, Stock, StepOrigin, PROCESS_3D_SCHEMA};
    use protocol::CollectionOperation;
    use store::{create_document_envelope, test_support, DocumentCommand};
    use vcs::Author;

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn drill_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Drill".into(), enabled: true, origin: Some(StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() }), measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() } }
    }

    fn new_store() -> Process3dStore {
        Process3dStore::new(create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None))
    }

    #[test]
    fn adds_and_removes_steps() {
        let mut store = new_store();
        store.dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { id: "cut-1".into(), item: cut_step("cut-1"), at: 0 } }], description: None }).expect("add step");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.steps[0].id, "cut-1");

        store.dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "cut-1".into() } }], description: None }).expect("remove step");
        assert!(store.projection().expect("projection").steps.is_empty());
    }

    #[test]
    fn patches_a_step_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { id: "cut-1".into(), item: cut_step("cut-1"), at: 0 } }], description: None }).expect("add step");
        store
            .dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch: ProcessStepPatch { enabled: Some(false), ..Default::default() } } }], description: None })
            .expect("patch step");
        assert!(!store.projection().expect("projection").steps[0].enabled);

        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert!(store.projection().expect("projection").steps[0].enabled);
    }

    #[test]
    fn patches_origin_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { id: "cut-1".into(), item: cut_step("cut-1"), at: 0 } }], description: None }).expect("add step");
        assert!(store.projection().expect("projection").steps[0].origin.is_none());

        let origin = StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() };
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch: ProcessStepPatch { origin: Some(Some(origin.clone())), ..Default::default() } } }],
                description: None,
            })
            .expect("patch origin");
        assert_eq!(store.projection().expect("projection").steps[0].origin, Some(origin));

        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert!(store.projection().expect("projection").steps[0].origin.is_none());
    }

    #[test]
    fn moves_and_clamps_cursor() {
        let mut store = new_store();
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![
                    Process3dOperation::Steps { collection: CollectionOperation::Add { id: "a".into(), item: cut_step("a"), at: 0 } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { id: "b".into(), item: cut_step("b"), at: 1 } },
                    Process3dOperation::SetCursor { resolved_up_to: Some(2) },
                ],
                description: None,
            })
            .expect("build steps + cursor");
        assert_eq!(store.projection().expect("projection").resolved_up_to, Some(2));

        store.dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "b".into() } }], description: None }).expect("remove step clamps cursor");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.resolved_up_to, Some(1));
    }

    #[test]
    fn sets_stock_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.projection().expect("projection").stock;
        let new_stock = Stock { id: "beam".into(), label: "Beam".into(), solid: SolidSpec::Cylinder { radius: 0.2, height: 2.0 }, pose: Pose::default() };
        store.dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::SetStock { stock: new_stock.clone() }], description: None }).expect("set stock");
        assert_eq!(store.projection().expect("projection").stock, new_stock);

        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").stock, original_stock);
    }

    #[test]
    fn sets_stock_to_imported_solid_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.projection().expect("projection").stock;
        let imported_stock = Stock { id: "stock".into(), label: "Imported STEP".into(), solid: SolidSpec::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() };
        store.dispatch(DocumentCommand::Apply { operations: vec![Process3dOperation::SetStock { stock: imported_stock.clone() }], description: None }).expect("set imported stock");
        assert_eq!(store.projection().expect("projection").stock, imported_stock);

        store.dispatch(DocumentCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").stock, original_stock);
    }

    //#region 🔖DocumentTextTests
    #[test]
    fn process3d_document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None);
        let mut store = Process3dStore::new(envelope);
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![
                    Process3dOperation::SetStock { stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose::default() } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { id: "cut-1".into(), item: cut_step("cut-1"), at: 0 } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { id: "drill-1".into(), item: drill_step("drill-1"), at: 1 } },
                    Process3dOperation::SetCursor { resolved_up_to: Some(1) },
                ],
                description: Some("build timeline".into()),
            })
            .expect("apply");
        store
            .dispatch(DocumentCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }],
            })
            .expect("commit");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests

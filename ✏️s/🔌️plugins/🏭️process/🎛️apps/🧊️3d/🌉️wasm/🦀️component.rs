//! 🌉️ Process 3d play app — the `Process3dEnvelope`/`Process3dStore` VCS type aliases and the
//! standalone WASM bridge (`Process3dSnapshotVcs`) — moved out of the old `📡️protocol` crate, which is
//! where they lived because that crate was the first constitutional crate in the old stack with both
//! `Process3dSnapshot` (artifact) and `Process3dMutation` (artifact `🔧️op`) available together. Now
//! that everything is one crate, this is simply the app's `🦀️wasm.rs` bridge file.

use crate::artifacts::process3d::op::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type Process3dEnvelope = ArtifactEnvelope<Process3dSnapshot, Process3dMutation>;
pub type Process3dStore = ArtifactStore<Process3dSnapshot, Process3dMutation>;
//#endregion 🔖️Store

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use crate::artifacts::process3d::PROCESS_3D_SCHEMA;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Process3dSnapshotVcs {
        store: RefCell<Process3dStore>,
    }

    #[wasm_bindgen]
    impl Process3dSnapshotVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Process3dSnapshotVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Process3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Process3dStore::new(envelope)
                }
                None => Process3dStore::new(create_document_envelope(PROCESS_3D_SCHEMA, "process3d", crate::artifacts::process3d::empty_process3d_snapshot(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::process3d::mutations::change_cursor::mutation::ChangeCursor;
    use crate::artifacts::process3d::mutations::change_step_enabled::mutation::ChangeStepEnabled;
    use crate::artifacts::process3d::mutations::change_step_origin::mutation::ChangeStepOrigin;
    use crate::artifacts::process3d::mutations::change_stock_label::mutation::ChangeStockLabel;
    use crate::artifacts::process3d::mutations::create_step::mutation::CreateStep;
    use crate::artifacts::process3d::mutations::delete_step::mutation::DeleteStep;
    use crate::artifacts::process3d::mutations::replace_stock_solid::mutation::ReplaceStockSolid;
    use crate::artifacts::process3d::op::Process3dMutation;
    use crate::artifacts::process3d::{empty_process3d_snapshot, Pose, ProcessMeasure, ProcessStep, SolidSpec, StepOrigin, PROCESS_3D_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};
    use vcs::Author;

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn drill_step(id: &str) -> ProcessStep {
        ProcessStep {
            id: id.into(),
            label: "Drill".into(),
            enabled: true,
            origin: Some(StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() }),
            measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() },
        }
    }

    fn new_store() -> Process3dStore {
        Process3dStore::new(create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None))
    }

    #[test]
    fn adds_and_removes_steps() {
        let mut store = new_store();
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") })], description: None }).expect("add step");
        let projection = store.snapshot().expect("snapshot");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.steps[0].id, "cut-1");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::DeleteStep(DeleteStep { id: "cut-1".into() })], description: None }).expect("remove step");
        assert!(store.snapshot().expect("snapshot").steps.is_empty());
    }

    #[test]
    fn patches_a_step_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") })], description: None }).expect("add step");
        store
            .dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "cut-1".into(), new_enabled: false })], description: None })
            .expect("patch step");
        assert!(!store.snapshot().expect("snapshot").steps[0].enabled);

        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert!(store.snapshot().expect("snapshot").steps[0].enabled);
    }

    #[test]
    fn patches_origin_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") })], description: None }).expect("add step");
        assert!(store.snapshot().expect("snapshot").steps[0].origin.is_none());

        let origin = StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() };
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "cut-1".into(), new_origin: Some(origin.clone()) })],
                description: None,
            })
            .expect("patch origin");
        assert_eq!(store.snapshot().expect("snapshot").steps[0].origin, Some(origin));

        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert!(store.snapshot().expect("snapshot").steps[0].origin.is_none());
    }

    #[test]
    fn moves_and_clamps_cursor() {
        let mut store = new_store();
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![
                    Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("a") }),
                    Process3dMutation::CreateStep(CreateStep { index: 1, step: cut_step("b") }),
                    Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(2) }),
                ],
                description: None,
            })
            .expect("build steps + cursor");
        assert_eq!(store.snapshot().expect("snapshot").resolved_up_to, Some(2));

        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::DeleteStep(DeleteStep { id: "b".into() })], description: None }).expect("remove step clamps cursor");
        let projection = store.snapshot().expect("snapshot");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.resolved_up_to, Some(1));
    }

    /// 🧬️ `Stock`'s `id` has no semantic mutation of its own (it is a fixed singleton-facet key, never
    /// a user-addressed identity field) — only `solid`/`label`/`pose` each carry their own mutation
    /// now (`ReplaceStockSolid`/`ChangeStockLabel`/`MoveStock`), so these two tests compose the fields
    /// that actually change instead of replacing the whole `Stock` record.
    #[test]
    fn sets_stock_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.snapshot().expect("snapshot").stock;
        let new_solid = SolidSpec::Cylinder { radius: 0.2, height: 2.0 };
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: new_solid.clone() }), Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() })],
                description: None,
            })
            .expect("set stock");
        let updated = store.snapshot().expect("snapshot").stock;
        assert_eq!(updated.solid, new_solid);
        assert_eq!(updated.label, "Beam");

        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").stock, original_stock);
    }

    #[test]
    fn sets_stock_to_imported_solid_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.snapshot().expect("snapshot").stock;
        let imported_solid = SolidSpec::ImportedSolid { solid_handle: "solid-7".into() };
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: imported_solid.clone() }), Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Imported STEP".into() })],
                description: None,
            })
            .expect("set imported stock");
        let updated = store.snapshot().expect("snapshot").stock;
        assert_eq!(updated.solid, imported_solid);
        assert_eq!(updated.label, "Imported STEP");

        store.dispatch(ArtifactCommand::Undo).expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").stock, original_stock);
    }

    //#region 🔖️DocumentTextTests
    #[test]
    fn process3d_document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None);
        let mut store = Process3dStore::new(envelope);
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![
                    Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 } }),
                    Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Timber Beam".into() }),
                    Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") }),
                    Process3dMutation::CreateStep(CreateStep { index: 1, step: drill_step("drill-1") }),
                    Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(1) }),
                ],
                description: Some("build timeline".into()),
            })
            .expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).expect("commit");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖️DocumentTextTests
}
//#endregion 🧪️Tests

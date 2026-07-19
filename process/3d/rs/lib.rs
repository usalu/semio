//! 🪚 Process 3d document VCS on `vcs` — subtractive/additive processing steps on a stock solid.

use serde::{Deserialize, Serialize};
use vcs::{apply_collection_op, invert_collection_op, CollectionOp, DocumentVcsEnvelope, DocumentVcsStore, Identified, OperationDiff, Patchable};

pub const PROCESS_3D_SCHEMA: &str = "process.3d";

//#region 🔖Document
fn default_axis_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

fn default_true() -> bool {
    true
}

/// 🧭 Position + axis-angle rotation applied via the brep kernel's `rotate_sync`/`translate_sync`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pose {
    #[serde(default)]
    pub position: [f64; 3],
    #[serde(default = "default_axis_z")]
    pub axis: [f64; 3],
    #[serde(default)]
    pub angle: f64,
}

/// 📦 Primitive solid spec resolvable via `BrepkitKernel::*_prim_sync`, or a non-parametric imported
/// reference (mesh or real B-Rep solid) resolved by the app's own kernel session instead of a primitive.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SolidSpec {
    Box {
        width: f64,
        depth: f64,
        height: f64,
    },
    Cylinder {
        radius: f64,
        height: f64,
    },
    Sphere {
        radius: f64,
    },
    /// 🖼️ Non-parametric GLB-imported reference mesh — tessellation-only, no real B-Rep topology
    /// (mirrors `cad`'s `meshUrl` pattern); cannot serve as a Cut/Drill/Attach tool.
    ImportedMesh {
        mesh_url: String,
    },
    /// 🧊 STEP/OBJ/STL-imported solid with real B-Rep topology, resolved through the app's kernel
    /// session by handle id (mirrors `cad`'s `solidHandle` pattern); ephemeral to that session.
    ImportedSolid {
        solid_handle: String,
    },
}

/// 🪵 The raw workpiece the process starts from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stock {
    pub id: String,
    pub label: String,
    pub solid: SolidSpec,
    #[serde(default)]
    pub pose: Pose,
}

impl Default for Stock {
    fn default() -> Self {
        Self { id: "stock".into(), label: "Stock".into(), solid: SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }
    }
}

/// 🪚 One processing measure: subtractive (cut/drill via `cut_sync`) or additive (attach via `fuse_sync`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "measure", rename_all = "camelCase")]
pub enum ProcessMeasure {
    /// ✂️ Subtractive: subtracts an arbitrary tool solid (e.g. a thin box as a saw blade).
    Cut { tool: SolidSpec, pose: Pose },
    /// 🕳️ Subtractive: a cylinder of `radius`×`depth` subtracted at `pose` (axis = drill direction).
    Drill { radius: f64, depth: f64, pose: Pose },
    /// 🔩 Additive: fuses another component solid at `pose`.
    Attach { component: SolidSpec, pose: Pose },
}

/// 🏭 Provenance: which module/machine/modification-kind produced a step (display + future re-validation).
/// Purely informational — kernel replay only ever reads `ProcessMeasure`, never resolves this back to a
/// catalog entry, so an older/renamed catalog can never retroactively change already-authored geometry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOrigin {
    pub module_id: String,
    pub machine_id: String,
    pub modification_kind_id: String,
}

/// 🎞️ One ordered step of the process timeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStep {
    pub id: String,
    pub label: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<StepOrigin>,
    #[serde(flatten)]
    pub measure: ProcessMeasure,
}

impl Identified<String> for ProcessStep {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹 Sparse edit for a `ProcessStep` — `None` fields are left untouched.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStepPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measure: Option<ProcessMeasure>,
    /// 🏭 Outer `Option` = "this patch touches origin"; inner `Option` = the new value (`None` clears it).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Option<StepOrigin>>,
}

impl Patchable<ProcessStepPatch> for ProcessStep {
    fn apply_patch(&mut self, patch: &ProcessStepPatch) -> ProcessStepPatch {
        let inverse = ProcessStepPatch {
            label: patch.label.as_ref().map(|_| self.label.clone()),
            enabled: patch.enabled.as_ref().map(|_| self.enabled),
            measure: patch.measure.as_ref().map(|_| self.measure.clone()),
            origin: patch.origin.as_ref().map(|_| self.origin.clone()),
        };
        if let Some(label) = &patch.label {
            self.label = label.clone();
        }
        if let Some(enabled) = patch.enabled {
            self.enabled = enabled;
        }
        if let Some(measure) = &patch.measure {
            self.measure = measure.clone();
        }
        if let Some(origin) = &patch.origin {
            self.origin = origin.clone();
        }
        inverse
    }
}

/// 🪚 Process 3d projection: stock + ordered steps + timeline cursor.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDocument {
    #[serde(default)]
    pub stock: Stock,
    #[serde(default)]
    pub steps: Vec<ProcessStep>,
    /// ⏱️ Number of enabled steps replayed (0..=steps.len()); `None` applies all.
    #[serde(default)]
    pub resolved_up_to: Option<usize>,
}

pub fn empty_process3d_projection() -> Process3dDocument {
    Process3dDocument::default()
}
//#endregion 🔖Document

//#region 🔖Operations
/// 🪚 Process 3d document operation: an ordered-step collection edit, a stock swap, or a cursor move.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Process3dOp {
    Steps {
        collection: CollectionOp<String, ProcessStep, ProcessStepPatch>,
    },
    SetStock {
        stock: Stock,
    },
    SetCursor {
        resolved_up_to: Option<usize>,
    },
    /// 🔁 Wholesale document swap (loading a different example fixture) — a true inverse restores the
    /// exact prior document, mirroring `ShootingOp::SetFixture`.
    SetDocument {
        document: Process3dDocument,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<CollectionOp<String, ProcessStep, ProcessStepPatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stock: Option<Stock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Option<usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Process3dDocument>,
}

impl OperationDiff<Process3dDocument> for Process3dDiff {
    fn apply(&self, projection: &Process3dDocument) -> Process3dDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(op) = &self.steps {
            apply_collection_op(&mut next.steps, op);
        }
        if let Some(stock) = &self.stock {
            next.stock = stock.clone();
        }
        if let Some(cursor) = &self.cursor {
            next.resolved_up_to = *cursor;
        }
        if let Some(cursor) = next.resolved_up_to {
            next.resolved_up_to = Some(cursor.min(next.steps.len()));
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
            self.steps = None;
            self.stock = None;
            self.cursor = None;
            return;
        }
        if other.steps.is_some() {
            self.steps = other.steps;
        }
        if other.stock.is_some() {
            self.stock = other.stock;
        }
        if other.cursor.is_some() {
            self.cursor = other.cursor;
        }
    }
}

impl vcs::Operation<Process3dDocument> for Process3dOp {
    type Diff = Process3dDiff;

    fn diff(&self, _projection: &Process3dDocument) -> Self::Diff {
        match self {
            Process3dOp::Steps { collection } => Process3dDiff { steps: Some(collection.clone()), ..Default::default() },
            Process3dOp::SetStock { stock } => Process3dDiff { stock: Some(stock.clone()), ..Default::default() },
            Process3dOp::SetCursor { resolved_up_to } => Process3dDiff { cursor: Some(*resolved_up_to), ..Default::default() },
            Process3dOp::SetDocument { document } => Process3dDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &Process3dDocument) -> Vec<Self> {
        match self {
            Process3dOp::Steps { collection } => {
                vec![Process3dOp::Steps { collection: invert_collection_op(&projection.steps, collection) }]
            }
            Process3dOp::SetStock { .. } => vec![Process3dOp::SetStock { stock: projection.stock.clone() }],
            Process3dOp::SetCursor { .. } => vec![Process3dOp::SetCursor { resolved_up_to: projection.resolved_up_to }],
            Process3dOp::SetDocument { .. } => vec![Process3dOp::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Process3dEnvelope = DocumentVcsEnvelope<Process3dDocument, Process3dOp>;
pub type Process3dStore = DocumentVcsStore<Process3dDocument, Process3dOp>;
//#endregion 🔖Operations

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use vcs::create_document_vcs_envelope;
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
                None => Process3dStore::new(create_document_vcs_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use vcs::{create_document_vcs_envelope, DocumentVcsCommand, Operation};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn new_store() -> Process3dStore {
        Process3dStore::new(create_document_vcs_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None))
    }

    #[test]
    fn adds_and_removes_steps() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::Steps { collection: CollectionOp::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.steps[0].id, "cut-1");

        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::Steps { collection: CollectionOp::Remove { id: "cut-1".into() } }], description: None }).expect("remove step");
        assert!(store.projection().expect("projection").steps.is_empty());
    }

    #[test]
    fn patches_a_step_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::Steps { collection: CollectionOp::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        store
            .dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::Steps { collection: CollectionOp::Patch { id: "cut-1".into(), patch: ProcessStepPatch { enabled: Some(false), ..Default::default() } } }], description: None })
            .expect("patch step");
        assert!(!store.projection().expect("projection").steps[0].enabled);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert!(store.projection().expect("projection").steps[0].enabled);
    }

    #[test]
    fn patches_origin_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::Steps { collection: CollectionOp::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        assert!(store.projection().expect("projection").steps[0].origin.is_none());

        let origin = StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() };
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Process3dOp::Steps { collection: CollectionOp::Patch { id: "cut-1".into(), patch: ProcessStepPatch { origin: Some(Some(origin.clone())), ..Default::default() } } }],
                description: None,
            })
            .expect("patch origin");
        assert_eq!(store.projection().expect("projection").steps[0].origin, Some(origin));

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert!(store.projection().expect("projection").steps[0].origin.is_none());
    }

    #[test]
    fn legacy_step_json_without_origin_deserializes_with_none() {
        let legacy_json = r#"{"id":"cut-1","label":"Cut","enabled":true,"measure":"cut","tool":{"kind":"box","width":0.1,"depth":0.1,"height":0.1},"pose":{"position":[0.0,0.0,0.0],"axis":[0.0,0.0,1.0],"angle":0.0}}"#;
        let step: ProcessStep = serde_json::from_str(legacy_json).expect("legacy step json");
        assert!(step.origin.is_none());
        assert_eq!(step.id, "cut-1");
    }

    #[test]
    fn moves_and_clamps_cursor() {
        let mut store = new_store();
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![
                    Process3dOp::Steps { collection: CollectionOp::Add { index: 0, item: cut_step("a") } },
                    Process3dOp::Steps { collection: CollectionOp::Add { index: 1, item: cut_step("b") } },
                    Process3dOp::SetCursor { resolved_up_to: Some(2) },
                ],
                description: None,
            })
            .expect("build steps + cursor");
        assert_eq!(store.projection().expect("projection").resolved_up_to, Some(2));

        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::Steps { collection: CollectionOp::Remove { id: "b".into() } }], description: None }).expect("remove step clamps cursor");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.resolved_up_to, Some(1));
    }

    #[test]
    fn sets_stock_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.projection().expect("projection").stock;
        let new_stock = Stock { id: "beam".into(), label: "Beam".into(), solid: SolidSpec::Cylinder { radius: 0.2, height: 2.0 }, pose: Pose::default() };
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::SetStock { stock: new_stock.clone() }], description: None }).expect("set stock");
        assert_eq!(store.projection().expect("projection").stock, new_stock);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").stock, original_stock);
    }

    #[test]
    fn imported_mesh_solid_spec_round_trips_json() {
        let solid = SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() };
        let json = serde_json::to_value(&solid).expect("serialize");
        assert_eq!(json["kind"], "importedMesh");
        assert_eq!(json["meshUrl"], "data:model/gltf-binary;base64,AAAA");
        let parsed: SolidSpec = serde_json::from_value(json).expect("deserialize");
        assert_eq!(parsed, solid);
    }

    #[test]
    fn imported_solid_solid_spec_round_trips_json() {
        let solid = SolidSpec::ImportedSolid { solid_handle: "solid-42".into() };
        let json = serde_json::to_value(&solid).expect("serialize");
        assert_eq!(json["kind"], "importedSolid");
        assert_eq!(json["solidHandle"], "solid-42");
        let parsed: SolidSpec = serde_json::from_value(json).expect("deserialize");
        assert_eq!(parsed, solid);
    }

    #[test]
    fn sets_stock_to_imported_solid_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.projection().expect("projection").stock;
        let imported_stock = Stock { id: "stock".into(), label: "Imported STEP".into(), solid: SolidSpec::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() };
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOp::SetStock { stock: imported_stock.clone() }], description: None }).expect("set imported stock");
        assert_eq!(store.projection().expect("projection").stock, imported_stock);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").stock, original_stock);
    }

    #[test]
    fn backwards_of_add_is_remove() {
        let projection = empty_process3d_projection();
        let op = Process3dOp::Steps { collection: CollectionOp::Add { index: 0, item: cut_step("a") } };
        let inverse = op.backwards(&projection);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dOp::Steps { collection: CollectionOp::Remove { id } } => assert_eq!(id, "a"),
            _ => panic!("expected Steps::Remove"),
        }
    }
}

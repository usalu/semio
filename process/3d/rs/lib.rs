//! 🪚 Process 3d document VCS on `vcs` — subtractive/additive processing steps on a stock solid.

use serde::{Deserialize, Serialize};
use vcs::{apply_collection_operation, invert_collection_operation, CollectionOperation, DocumentVcsEnvelope, DocumentVcsStore, Identified, OperationDiff, Patchable};

pub const PROCESS_3D_SCHEMA: &str = "process.3d";

//#region 🔖Document
fn default_axis_z() -> [f64; 3] {
    [0.0, 0.0, 1.0]
}

fn default_true() -> bool {
    true
}

/// 🧭 Position + axis-angle rotation applied via the brep kernel's `rotate_sync`/`translate_sync`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum SolidSpec {
    #[dsl(key = "box")]
    Box {
        width: f64,
        depth: f64,
        height: f64,
    },
    #[dsl(key = "cylinder")]
    Cylinder {
        radius: f64,
        height: f64,
    },
    #[dsl(key = "sphere")]
    Sphere {
        radius: f64,
    },
    /// 🖼️ Non-parametric GLB-imported reference mesh — tessellation-only, no real B-Rep topology
    /// (mirrors `cad`'s `meshUrl` pattern); cannot serve as a Cut/Drill/Attach tool.
    #[dsl(key = "importedMesh")]
    ImportedMesh {
        mesh_url: String,
    },
    /// 🧊 STEP/OBJ/STL-imported solid with real B-Rep topology, resolved through the app's kernel
    /// session by handle id (mirrors `cad`'s `solidHandle` pattern); ephemeral to that session.
    #[dsl(key = "importedSolid")]
    ImportedSolid {
        solid_handle: String,
    },
}

/// 🌉 `#[derive(dsl::DslEnum)]` only gives `SolidSpec` a `dsl::DslVariants` binding (a tagged-record
/// table), not `dsl::DslField` — so it can't sit directly in a plain (non-`Option`/`Vec`) field on
/// its own. Every real usage site (`Stock::solid`, `ProcessMeasure::Cut::tool`,
/// `ProcessMeasure::Attach::component`) is a REQUIRED, never-optional, never-collection single value,
/// which the derive macro would normally solve via `#[dsl(statements)] Box<SolidSpec>` — but boxing
/// would change the field's Rust-visible type and break `process/plugin`'s existing pattern
/// matches/struct literals against a bare `SolidSpec`. This hand impl reuses the exact same "exactly
/// one tagged statement" idiom the derive's `Box<T>`-required-statements codegen uses internally,
/// applied directly to `SolidSpec` so every real field stays unboxed.
impl dsl::DslField for SolidSpec {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<SolidSpec as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<SolidSpec as dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <SolidSpec as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged solid value, found {other:?}")),
        }
    }
}

/// 🪵 The raw workpiece the process starts from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Stock {
    pub id: String,
    pub label: String,
    pub solid: SolidSpec,
    #[serde(default)]
    #[dsl(block)]
    pub pose: Pose,
}

impl Default for Stock {
    fn default() -> Self {
        Self { id: "stock".into(), label: "Stock".into(), solid: SolidSpec::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() }
    }
}

/// 🪚 One processing measure: subtractive (cut/drill via `cut_sync`) or additive (attach via `fuse_sync`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "measure", rename_all = "camelCase")]
pub enum ProcessMeasure {
    /// ✂️ Subtractive: subtracts an arbitrary tool solid (e.g. a thin box as a saw blade).
    #[dsl(key = "cut")]
    Cut { tool: SolidSpec, #[dsl(block)] pose: Pose },
    /// 🕳️ Subtractive: a cylinder of `radius`×`depth` subtracted at `pose` (axis = drill direction).
    #[dsl(key = "drill")]
    Drill { radius: f64, depth: f64, #[dsl(block)] pose: Pose },
    /// 🔩 Additive: fuses another component solid at `pose`.
    #[dsl(key = "attach")]
    Attach { component: SolidSpec, #[dsl(block)] pose: Pose },
}

/// 🌉 Same reasoning/idiom as `SolidSpec`'s hand `dsl::DslField` impl — `ProcessMeasure` is a
/// `DslEnum` (`DslVariants` only), and `ProcessStep::measure` is a REQUIRED, never-optional field
/// that must stay a bare `ProcessMeasure` (not `Box<ProcessMeasure>`) for `process/plugin`'s existing
/// `match &mut step.measure { ProcessMeasure::Cut { .. } => .. }` usage to keep compiling untouched.
impl dsl::DslField for ProcessMeasure {
    fn shape() -> dsl::Shape {
        dsl::Shape::Statements(<ProcessMeasure as dsl::DslVariants>::variants())
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Statements(vec![<ProcessMeasure as dsl::DslVariants>::to_named_record(self)])
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Statements(items) if items.len() == 1 => <ProcessMeasure as dsl::DslVariants>::from_named_record(&items[0].0, &items[0].1).map_err(|e| e.message),
            other => Err(format!("expected exactly 1 tagged measure value, found {other:?}")),
        }
    }
}

/// 🏭 Provenance: which module/machine/modification-kind produced a step (display + future re-validation).
/// Purely informational — kernel replay only ever reads `ProcessMeasure`, never resolves this back to a
/// catalog entry, so an older/renamed catalog can never retroactively change already-authored geometry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct StepOrigin {
    pub module_id: String,
    pub machine_id: String,
    pub modification_kind_id: String,
}

/// 🎞️ One ordered step of the process timeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStep {
    pub id: String,
    pub label: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub origin: Option<StepOrigin>,
    /// 🌉 `#[serde(flatten)]` is a JSON-only concern (`dsl(flatten)` is a dead/unimplemented derive
    /// flag) — the DSL grammar just gives `measure` its own ordinary tagged shape via `ProcessMeasure`'s
    /// hand `dsl::DslField` impl (see its doc comment), printed as its own `cut|drill|attach ...`
    /// statement on the step's line.
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "process3d", layout = "lines")]
pub struct Process3dDocument {
    #[serde(default)]
    #[dsl(block)]
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
pub enum Process3dOperation {
    Steps {
        collection: CollectionOperation<String, ProcessStep, ProcessStepPatch>,
    },
    SetStock {
        stock: Stock,
    },
    SetCursor {
        resolved_up_to: Option<usize>,
    },
    /// 🔁 Wholesale document swap (loading a different example fixture) — a true inverse restores the
    /// exact prior document, mirroring `ShootingOperation::SetFixture`.
    SetDocument {
        document: Process3dDocument,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<CollectionOperation<String, ProcessStep, ProcessStepPatch>>,
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
        if let Some(operation) = &self.steps {
            apply_collection_operation(&mut next.steps, operation);
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

impl vcs::Operation<Process3dDocument> for Process3dOperation {
    type Diff = Process3dDiff;

    fn diff(&self, _projection: &Process3dDocument) -> Self::Diff {
        match self {
            Process3dOperation::Steps { collection } => Process3dDiff { steps: Some(collection.clone()), ..Default::default() },
            Process3dOperation::SetStock { stock } => Process3dDiff { stock: Some(stock.clone()), ..Default::default() },
            Process3dOperation::SetCursor { resolved_up_to } => Process3dDiff { cursor: Some(*resolved_up_to), ..Default::default() },
            Process3dOperation::SetDocument { document } => Process3dDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &Process3dDocument) -> Vec<Self> {
        match self {
            Process3dOperation::Steps { collection } => {
                vec![Process3dOperation::Steps { collection: invert_collection_operation(&projection.steps, collection) }]
            }
            Process3dOperation::SetStock { .. } => vec![Process3dOperation::SetStock { stock: projection.stock.clone() }],
            Process3dOperation::SetCursor { .. } => vec![Process3dOperation::SetCursor { resolved_up_to: projection.resolved_up_to }],
            Process3dOperation::SetDocument { .. } => vec![Process3dOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Process3dEnvelope = DocumentVcsEnvelope<Process3dDocument, Process3dOperation>;
pub type Process3dStore = DocumentVcsStore<Process3dDocument, Process3dOperation>;
//#endregion 🔖Operations

//#region 🔖Dsl
// `impl vcs::DocumentDsl for Process3dDocument` is emitted automatically by the
// `#[derive(dsl::DslDocument)]` on `Process3dDocument` itself (see `🔖Document`) — no manual impl
// needed here. The former hand-rolled `mod process3d_text` lexer/parser/printer (and the temporary
// fixture-regeneration test that used it) have been removed now that both example fixtures have
// been regenerated through the derive-generated printer.
//#endregion 🔖Dsl

//#region 🔖OpText
/// 🩹 `ProcessStepPatch.origin: Option<Option<StepOrigin>>` needs a real 3-state tag (untouched /
/// explicitly cleared / set to a new value) that the DSL engine's plain `Option<T>` can't express in
/// one level — `StepOriginPatch` is that local 2-variant tag; `None` at the wrapping
/// `Option<StepOriginPatch>` level means "untouched", `Some(Clear)` means "explicitly cleared", and
/// `Some(Set { .. })` carries the new value.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum StepOriginPatch {
    #[dsl(key = "clear")]
    Clear,
    #[dsl(key = "set")]
    Set {
        #[dsl(block)]
        origin: StepOrigin,
    },
}

/// 🩹 Local DSL-only mirror of `ProcessStepPatch` — the real type's `origin: Option<Option<StepOrigin>>`
/// shape has no direct `dsl::DslField` binding (see `StepOriginPatch`), so this twin carries the same
/// four fields through the `steps-patch` operation grammar and converts at that boundary only; this
/// shape is never fixture-visible (`ProcessStepPatch` never appears in a `.process3d` document, only
/// in the op log), so its exact wire form has no compatibility obligation beyond its own round trip.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct ProcessStepPatchDsl {
    label: Option<String>,
    enabled: Option<bool>,
    #[dsl(statements)]
    measure: Option<ProcessMeasure>,
    #[dsl(statements, block)]
    origin: Option<StepOriginPatch>,
}

fn process_step_patch_to_dsl(patch: &ProcessStepPatch) -> ProcessStepPatchDsl {
    ProcessStepPatchDsl {
        label: patch.label.clone(),
        enabled: patch.enabled,
        measure: patch.measure.clone(),
        origin: match &patch.origin {
            None => None,
            Some(None) => Some(StepOriginPatch::Clear),
            Some(Some(origin)) => Some(StepOriginPatch::Set { origin: origin.clone() }),
        },
    }
}

fn process_step_patch_from_dsl(patch: ProcessStepPatchDsl) -> ProcessStepPatch {
    ProcessStepPatch {
        label: patch.label,
        enabled: patch.enabled,
        measure: patch.measure,
        origin: match patch.origin {
            None => None,
            Some(StepOriginPatch::Clear) => Some(None),
            Some(StepOriginPatch::Set { origin }) => Some(Some(origin)),
        },
    }
}

/// ✂️ Local DSL-only mirror of `Process3dOperation` — `vcs::CollectionOperation<K,V,P>` is declared
/// in the `vcs` crate (foreign type), so it cannot itself gain a `dsl::DslField`/`dsl::DslVariants`
/// binding here (orphan rule: neither the trait nor the type is local to this crate). This twin
/// flattens the `Steps { collection }` wrapper into its own four keyworded variants — mirroring
/// `imperative_core::ImperativeOperationDsl`'s identical fix for the same foreign-`CollectionOperation`
/// problem — and converts at the `vcs::OpText` boundary only; `Process3dOperation` itself, and every
/// consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Process3dOperationDsl {
    #[dsl(key = "steps-add")]
    StepsAdd {
        index: usize,
        #[dsl(block)]
        item: ProcessStep,
    },
    #[dsl(key = "steps-remove")]
    StepsRemove { id: String },
    #[dsl(key = "steps-move")]
    StepsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    #[dsl(key = "steps-patch")]
    StepsPatch {
        id: String,
        #[dsl(block)]
        patch: ProcessStepPatchDsl,
    },
    #[dsl(key = "stock")]
    SetStock {
        #[dsl(block)]
        stock: Stock,
    },
    #[dsl(key = "cursor")]
    SetCursor { value: Option<usize> },
    #[dsl(key = "document")]
    SetDocument {
        #[dsl(block)]
        document: Process3dDocument,
    },
}

fn process3d_operation_to_dsl(operation: &Process3dOperation) -> Process3dOperationDsl {
    match operation {
        Process3dOperation::Steps { collection: CollectionOperation::Add { index, item } } => Process3dOperationDsl::StepsAdd { index: *index, item: item.clone() },
        Process3dOperation::Steps { collection: CollectionOperation::Remove { id } } => Process3dOperationDsl::StepsRemove { id: id.clone() },
        Process3dOperation::Steps { collection: CollectionOperation::Move { id, to_index } } => Process3dOperationDsl::StepsMove { id: id.clone(), to_index: *to_index },
        Process3dOperation::Steps { collection: CollectionOperation::Patch { id, patch } } => {
            Process3dOperationDsl::StepsPatch { id: id.clone(), patch: process_step_patch_to_dsl(patch) }
        }
        Process3dOperation::SetStock { stock } => Process3dOperationDsl::SetStock { stock: stock.clone() },
        Process3dOperation::SetCursor { resolved_up_to } => Process3dOperationDsl::SetCursor { value: *resolved_up_to },
        Process3dOperation::SetDocument { document } => Process3dOperationDsl::SetDocument { document: document.clone() },
    }
}

fn process3d_operation_from_dsl(operation: Process3dOperationDsl) -> Process3dOperation {
    match operation {
        Process3dOperationDsl::StepsAdd { index, item } => Process3dOperation::Steps { collection: CollectionOperation::Add { index, item } },
        Process3dOperationDsl::StepsRemove { id } => Process3dOperation::Steps { collection: CollectionOperation::Remove { id } },
        Process3dOperationDsl::StepsMove { id, to_index } => Process3dOperation::Steps { collection: CollectionOperation::Move { id, to_index } },
        Process3dOperationDsl::StepsPatch { id, patch } => Process3dOperation::Steps { collection: CollectionOperation::Patch { id, patch: process_step_patch_from_dsl(patch) } },
        Process3dOperationDsl::SetStock { stock } => Process3dOperation::SetStock { stock },
        Process3dOperationDsl::SetCursor { value } => Process3dOperation::SetCursor { resolved_up_to: value },
        Process3dOperationDsl::SetDocument { document } => Process3dOperation::SetDocument { document },
    }
}

impl vcs::OpText for Process3dOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        Ok(process3d_operation_from_dsl(<Process3dOperationDsl as vcs::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <Process3dOperationDsl as vcs::OpText>::print_op(&process3d_operation_to_dsl(self))
    }
}
//#endregion 🔖OpText

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

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::{create_document_vcs_envelope, test_support, Author, DocumentDsl, DocumentVcsCommand, Operation};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn new_store() -> Process3dStore {
        Process3dStore::new(create_document_vcs_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None))
    }

    #[test]
    fn adds_and_removes_steps() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.steps[0].id, "cut-1");

        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "cut-1".into() } }], description: None }).expect("remove step");
        assert!(store.projection().expect("projection").steps.is_empty());
    }

    #[test]
    fn patches_a_step_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        store
            .dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch: ProcessStepPatch { enabled: Some(false), ..Default::default() } } }], description: None })
            .expect("patch step");
        assert!(!store.projection().expect("projection").steps[0].enabled);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert!(store.projection().expect("projection").steps[0].enabled);
    }

    #[test]
    fn patches_origin_and_undo_restores_it() {
        let mut store = new_store();
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } }], description: None }).expect("add step");
        assert!(store.projection().expect("projection").steps[0].origin.is_none());

        let origin = StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() };
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch: ProcessStepPatch { origin: Some(Some(origin.clone())), ..Default::default() } } }],
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
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("a") } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 1, item: cut_step("b") } },
                    Process3dOperation::SetCursor { resolved_up_to: Some(2) },
                ],
                description: None,
            })
            .expect("build steps + cursor");
        assert_eq!(store.projection().expect("projection").resolved_up_to, Some(2));

        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "b".into() } }], description: None }).expect("remove step clamps cursor");
        let projection = store.projection().expect("projection");
        assert_eq!(projection.steps.len(), 1);
        assert_eq!(projection.resolved_up_to, Some(1));
    }

    #[test]
    fn sets_stock_and_backwards_restores() {
        let mut store = new_store();
        let original_stock = store.projection().expect("projection").stock;
        let new_stock = Stock { id: "beam".into(), label: "Beam".into(), solid: SolidSpec::Cylinder { radius: 0.2, height: 2.0 }, pose: Pose::default() };
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::SetStock { stock: new_stock.clone() }], description: None }).expect("set stock");
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
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![Process3dOperation::SetStock { stock: imported_stock.clone() }], description: None }).expect("set imported stock");
        assert_eq!(store.projection().expect("projection").stock, imported_stock);

        store.dispatch(DocumentVcsCommand::Undo).expect("undo");
        assert_eq!(store.projection().expect("projection").stock, original_stock);
    }

    #[test]
    fn backwards_of_add_is_remove() {
        let projection = empty_process3d_projection();
        let operation = Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("a") } };
        let inverse = operation.backwards(&projection);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dOperation::Steps { collection: CollectionOperation::Remove { id } } => assert_eq!(id, "a"),
            _ => panic!("expected Steps::Remove"),
        }
    }

    //#region 🔖DslTests
    fn drill_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Drill".into(), enabled: true, origin: Some(StepOrigin { module_id: "wood".into(), machine_id: "circularSaw".into(), modification_kind_id: "crosscut".into() }), measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() } }
    }

    fn attach_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Attach".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.05 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } } }
    }

    fn imported_mesh_stock() -> Stock {
        Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }, pose: Pose::default() }
    }

    /// 📜 A document exercising every `SolidSpec`/`ProcessMeasure` shape and both `origin` states, so
    /// the DSL round trip covers the full grammar, not just the happy path.
    fn sample_document() -> Process3dDocument {
        Process3dDocument {
            stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
            resolved_up_to: Some(2),
        }
    }

    #[test]
    fn process3d_dsl_round_trips() {
        test_support::assert_dsl_round_trip(&sample_document());
        test_support::assert_dsl_round_trip(&empty_process3d_projection());
    }

    #[test]
    fn process3d_dsl_round_trips_imported_solid_shapes() {
        let mut document = sample_document();
        document.stock = imported_mesh_stock();
        document.steps.push(ProcessStep { id: "imported-tool".into(), label: "Imported Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::ImportedSolid { solid_handle: "solid-7".into() }, pose: Pose::default() } });
        test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn process3d_dsl_round_trips_with_no_resolved_cursor() {
        let mut document = sample_document();
        document.resolved_up_to = None;
        test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn timber_example_fixture_parses_and_round_trips() {
        let text = include_str!("../example/timber-beam-joinery.process3d");
        let document = Process3dDocument::parse_dsl(text).expect("parse timber example");
        assert_eq!(document.steps.len(), 4);
        assert!(document.resolved_up_to.is_none());
        test_support::assert_dsl_round_trip(&document);
    }

    #[test]
    fn drilled_plate_example_fixture_parses_and_round_trips() {
        let text = include_str!("../example/drilled-plate.process3d");
        let document = Process3dDocument::parse_dsl(text).expect("parse drilled plate example");
        assert_eq!(document.steps.len(), 3);
        assert_eq!(document.resolved_up_to, Some(2));
        test_support::assert_dsl_round_trip(&document);
    }
    //#endregion 🔖DslTests

    //#region 🔖OpTextTests
    #[test]
    fn process3d_op_text_round_trips_steps_add() {
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_remove() {
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "cut-1".into() } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_move() {
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Move { id: "cut-1".into(), to_index: 2 } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_full() {
        let patch = ProcessStepPatch {
            label: Some("Renamed".into()),
            enabled: Some(false),
            measure: Some(ProcessMeasure::Drill { radius: 0.03, depth: 0.4, pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } }),
            origin: Some(Some(StepOrigin { module_id: "wood".into(), machine_id: "tableSaw".into(), modification_kind_id: "crosscut".into() })),
        };
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_clearing_origin() {
        let patch = ProcessStepPatch { label: None, enabled: None, measure: None, origin: Some(None) };
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_empty() {
        let patch = ProcessStepPatch::default();
        test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_set_stock() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetStock { stock: imported_mesh_stock() });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_some() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetCursor { resolved_up_to: Some(3) });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_none() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetCursor { resolved_up_to: None });
    }

    #[test]
    fn process3d_op_text_round_trips_set_document() {
        test_support::assert_op_line_round_trip(&Process3dOperation::SetDocument { document: sample_document() });
    }
    //#endregion 🔖OpTextTests

    //#region 🔖DocumentTextTests
    #[test]
    fn process3d_document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_vcs_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_projection(), None);
        let mut store = Process3dStore::new(envelope);
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![
                    Process3dOperation::SetStock { stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose::default() } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 0, item: cut_step("cut-1") } },
                    Process3dOperation::Steps { collection: CollectionOperation::Add { index: 1, item: drill_step("drill-1") } },
                    Process3dOperation::SetCursor { resolved_up_to: Some(1) },
                ],
                description: Some("build timeline".into()),
            })
            .expect("apply");
        store
            .dispatch(DocumentVcsCommand::CommitCheckpoint {
                message: Some("c1".into()),
                authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }],
            })
            .expect("commit");
        test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests

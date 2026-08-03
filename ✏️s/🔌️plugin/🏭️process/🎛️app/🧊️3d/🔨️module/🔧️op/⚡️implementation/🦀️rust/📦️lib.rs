//! ⚡️ Process 3d app — operation enum + laws (constitutional: op).

use process_3d::{Process3dDocument, ProcessMeasure, ProcessStep, ProcessStepPatch, Stock, StepOrigin, WorkshopMachine, WorkshopMachinePatch};
use protocol::{apply_collection_operation, invert_collection_operation, CollectionOperation, Operation, OperationDiff, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Operations
/// 🪚️ Process 3d document operation: an ordered-step collection edit, a workshop-machines collection
/// edit, a stock swap, or a cursor move.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Process3dOperation {
    Steps {
        collection: CollectionOperation<String, ProcessStep, ProcessStepPatch>,
    },
    Machines {
        collection: CollectionOperation<String, WorkshopMachine, WorkshopMachinePatch>,
    },
    SetStock {
        stock: Stock,
    },
    SetCursor {
        resolved_up_to: Option<usize>,
    },
    /// 🔁️ Wholesale document swap (loading a different example fixture) — a true inverse restores the
    /// exact prior document, mirroring `ShootingOperation::SetFixture`.
    SetDocument {
        document: Process3dDocument,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process3dDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<CollectionOperation<String, ProcessStep, ProcessStepPatch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub machines: Option<CollectionOperation<String, WorkshopMachine, WorkshopMachinePatch>>,
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
        if let Some(operation) = &self.machines {
            apply_collection_operation(&mut next.workshop.machines, operation);
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
            self.machines = None;
            self.stock = None;
            self.cursor = None;
            return;
        }
        if other.steps.is_some() {
            self.steps = other.steps;
        }
        if other.machines.is_some() {
            self.machines = other.machines;
        }
        if other.stock.is_some() {
            self.stock = other.stock;
        }
        if other.cursor.is_some() {
            self.cursor = other.cursor;
        }
    }
}

impl Operation<Process3dDocument> for Process3dOperation {
    type Diff = Process3dDiff;

    fn diff(&self, _projection: &Process3dDocument) -> Self::Diff {
        match self {
            Process3dOperation::Steps { collection } => Process3dDiff { steps: Some(collection.clone()), ..Default::default() },
            Process3dOperation::Machines { collection } => Process3dDiff { machines: Some(collection.clone()), ..Default::default() },
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
            Process3dOperation::Machines { collection } => {
                vec![Process3dOperation::Machines { collection: invert_collection_operation(&projection.workshop.machines, collection) }]
            }
            Process3dOperation::SetStock { .. } => vec![Process3dOperation::SetStock { stock: projection.stock.clone() }],
            Process3dOperation::SetCursor { .. } => vec![Process3dOperation::SetCursor { resolved_up_to: projection.resolved_up_to }],
            Process3dOperation::SetDocument { .. } => vec![Process3dOperation::SetDocument { document: projection.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🔖️OpText
/// 🩹️ `ProcessStepPatch.origin: Option<Option<StepOrigin>>` needs a real 3-state tag (untouched /
/// explicitly cleared / set to a new value) that the DSL engine's plain `Option<T>` can't express in
/// one level — `StepOriginPatch` is that local 2-variant tag; `None` at the wrapping
/// `Option<StepOriginPatch>` level means "untouched", `Some(Clear)` means "explicitly cleared", and
/// `Some(Set { .. })` carries the new value.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum StepOriginPatch {
    Clear,
    Set {
        #[dsl(block)]
        origin: StepOrigin,
    }
}

/// 🩹️ Local DSL-only mirror of `ProcessStepPatch` — the real type's `origin: Option<Option<StepOrigin>>`
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
/// problem — and converts at the `store::OpText` boundary only; `Process3dOperation` itself, and every
/// consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Process3dOperationDsl {
    StepsAdd {
        index: usize,
        #[dsl(block)]
        item: ProcessStep,
    },
    StepsRemove { id: String },
    StepsMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    StepsPatch {
        id: String,
        #[dsl(block)]
        patch: ProcessStepPatchDsl,
    },
    MachinesAdd {
        index: usize,
        #[dsl(block)]
        item: WorkshopMachine,
    },
    MachinesRemove { id: String },
    MachinesMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    MachinesPatch {
        id: String,
        #[dsl(block)]
        patch: WorkshopMachinePatch,
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
    }
}

fn process3d_operation_to_dsl(operation: &Process3dOperation) -> Process3dOperationDsl {
    match operation {
        Process3dOperation::Steps { collection: CollectionOperation::Add { id: _id, item, at } } => Process3dOperationDsl::StepsAdd { index: *at, item: item.clone() },
        Process3dOperation::Steps { collection: CollectionOperation::Remove { id } } => Process3dOperationDsl::StepsRemove { id: id.clone() },
        Process3dOperation::Steps { collection: CollectionOperation::Move { id, to } } => Process3dOperationDsl::StepsMove { id: id.clone(), to_index: *to },
        Process3dOperation::Steps { collection: CollectionOperation::Patch { id, patch } } => {
            Process3dOperationDsl::StepsPatch { id: id.clone(), patch: process_step_patch_to_dsl(patch) }
        }
        Process3dOperation::Machines { collection: CollectionOperation::Add { id: _id, item, at } } => Process3dOperationDsl::MachinesAdd { index: *at, item: item.clone() },
        Process3dOperation::Machines { collection: CollectionOperation::Remove { id } } => Process3dOperationDsl::MachinesRemove { id: id.clone() },
        Process3dOperation::Machines { collection: CollectionOperation::Move { id, to } } => Process3dOperationDsl::MachinesMove { id: id.clone(), to_index: *to },
        Process3dOperation::Machines { collection: CollectionOperation::Patch { id, patch } } => Process3dOperationDsl::MachinesPatch { id: id.clone(), patch: patch.clone() },
        Process3dOperation::SetStock { stock } => Process3dOperationDsl::SetStock { stock: stock.clone() },
        Process3dOperation::SetCursor { resolved_up_to } => Process3dOperationDsl::SetCursor { value: *resolved_up_to },
        Process3dOperation::SetDocument { document } => Process3dOperationDsl::SetDocument { document: document.clone() },
    }
}

fn process3d_operation_from_dsl(operation: Process3dOperationDsl) -> Process3dOperation {
    match operation {
        Process3dOperationDsl::StepsAdd { index, item } => Process3dOperation::Steps { collection: CollectionOperation::Add { id: item.id.clone(), item, at: index } },
        Process3dOperationDsl::StepsRemove { id } => Process3dOperation::Steps { collection: CollectionOperation::Remove { id } },
        Process3dOperationDsl::StepsMove { id, to_index } => Process3dOperation::Steps { collection: CollectionOperation::Move { id, to: to_index } },
        Process3dOperationDsl::StepsPatch { id, patch } => Process3dOperation::Steps { collection: CollectionOperation::Patch { id, patch: process_step_patch_from_dsl(patch) } },
        Process3dOperationDsl::MachinesAdd { index, item } => Process3dOperation::Machines { collection: CollectionOperation::Add { id: item.id.clone(), item, at: index } },
        Process3dOperationDsl::MachinesRemove { id } => Process3dOperation::Machines { collection: CollectionOperation::Remove { id } },
        Process3dOperationDsl::MachinesMove { id, to_index } => Process3dOperation::Machines { collection: CollectionOperation::Move { id, to: to_index } },
        Process3dOperationDsl::MachinesPatch { id, patch } => Process3dOperation::Machines { collection: CollectionOperation::Patch { id, patch } },
        Process3dOperationDsl::SetStock { stock } => Process3dOperation::SetStock { stock },
        Process3dOperationDsl::SetCursor { value } => Process3dOperation::SetCursor { resolved_up_to: value },
        Process3dOperationDsl::SetDocument { document } => Process3dOperation::SetDocument { document },
    }
}

impl OpText for Process3dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(process3d_operation_from_dsl(<Process3dOperationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <Process3dOperationDsl as OpText>::print_op(&process3d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Process3dOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for Process3dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        process3d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(process3d_operation_from_dsl(Process3dOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `process_3d_engine::Process3dConfig`'s operation enum — one variant per settled
/// interaction (mirrors the pre-B1 `Process3dRuntime` field writes), plus a generic `Snapshot` every
/// variant's `backwards()` returns — mirrors `shooting_op::ShootingConfigOperation`/
/// `lowpoly_op::LowpolyConfigOperation`'s identical pattern: a config-only dispatch is always a plain
/// `Apply` (never `AmendLast`), so "undo this tick" = "restore the whole-config snapshot from just
/// before it", the simplest correct inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Process3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: process_3d_engine::Process3dConfig,
    },
    #[dsl(key = "selected-id")]
    SetSelectedId { value: Option<String> },
    #[dsl(key = "hovered-id")]
    SetHoveredId { value: Option<String> },
    #[dsl(key = "selected-face-id")]
    SetSelectedFaceId { value: Option<u32> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
    #[dsl(key = "sun")]
    SetSun { enabled: bool, azimuth: f64, elevation: f64, intensity: f64, color: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<process_3d_engine::Process3dConfig> for Process3dConfigOperation {
    type Diff = process_3d_engine::Process3dConfig;

    fn diff(&self, base: &process_3d_engine::Process3dConfig) -> process_3d_engine::Process3dConfig {
        let mut next = base.clone();
        match self {
            Process3dConfigOperation::Snapshot { config } => return config.clone(),
            Process3dConfigOperation::SetSelectedId { value } => next.selected_id = value.clone(),
            Process3dConfigOperation::SetHoveredId { value } => next.hovered_id = value.clone(),
            Process3dConfigOperation::SetSelectedFaceId { value } => next.selected_face_id = *value,
            Process3dConfigOperation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            Process3dConfigOperation::SetCamera { position, target, fov } => {
                next.camera_position = *position;
                next.camera_target = *target;
                next.camera_fov = *fov;
            }
            Process3dConfigOperation::SetSun { enabled, azimuth, elevation, intensity, color } => {
                next.sun_enabled = *enabled;
                next.sun_azimuth = *azimuth;
                next.sun_elevation = *elevation;
                next.sun_intensity = *intensity;
                next.sun_color = color.clone();
            }
            Process3dConfigOperation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            Process3dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &process_3d_engine::Process3dConfig) -> Vec<Self> {
        vec![Process3dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use process_3d::{empty_process3d_projection, Pose, SolidSpec, Workshop};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn imported_mesh_stock() -> Stock {
        Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }, pose: Pose::default() }
    }

    fn drill_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Drill".into(), enabled: true, origin: Some(StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() }), measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() } }
    }

    fn attach_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Attach".into(), enabled: false, origin: None, measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.05 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } } }
    }

    fn circular_saw_machine() -> WorkshopMachine {
        use process_3d::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, StockQuantity};
        WorkshopMachine {
            id: "circularSaw".into(),
            label: "Circular Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: Some("wood".into()),
            capabilities: vec![Capability {
                id: "crosscut".into(),
                label: "Crosscut".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                parameters: vec![CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.184 }, CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 }],
                rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "bladeDiameter".into(), margin: 0.0 }],
            }],
        }
    }

    /// 📜️ A document exercising every `SolidSpec`/`ProcessMeasure` shape, both `origin` states, and a
    /// non-default workshop machine, so the OpText round trip covers the full grammar including the
    /// 3-deep workshop nesting, not just the happy path.
    fn sample_document() -> Process3dDocument {
        Process3dDocument {
            workshop: Workshop { machines: vec![circular_saw_machine()] },
            stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
            resolved_up_to: Some(2),
        }
    }

    #[test]
    fn backwards_of_add_is_remove() {
        let projection = empty_process3d_projection();
        let operation = Process3dOperation::Steps { collection: CollectionOperation::Add { id: "a".into(), item: cut_step("a"), at: 0 } };
        let inverse = operation.backwards(&projection);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dOperation::Steps { collection: CollectionOperation::Remove { id } } => assert_eq!(id, "a"),
            _ => panic!("expected Steps::Remove"),
        }
    }

    //#region 🔖️OpTextTests
    #[test]
    fn process3d_op_text_round_trips_steps_add() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Add { id: "cut-1".into(), item: cut_step("cut-1"), at: 0 } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_remove() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Remove { id: "cut-1".into() } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_move() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Move { id: "cut-1".into(), to: 2 } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_full() {
        let patch = ProcessStepPatch {
            label: Some("Renamed".into()),
            enabled: Some(false),
            measure: Some(ProcessMeasure::Drill { radius: 0.03, depth: 0.4, pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } }),
            origin: Some(Some(StepOrigin { machine_id: "tableSaw".into(), capability_id: "crosscut".into() })),
        };
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_clearing_origin() {
        let patch = ProcessStepPatch { label: None, enabled: None, measure: None, origin: Some(None) };
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_empty() {
        let patch = ProcessStepPatch::default();
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Steps { collection: CollectionOperation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_set_stock() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::SetStock { stock: imported_mesh_stock() });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_some() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::SetCursor { resolved_up_to: Some(3) });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_none() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::SetCursor { resolved_up_to: None });
    }

    #[test]
    fn process3d_op_text_round_trips_set_document() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::SetDocument { document: sample_document() });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_add() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Machines { collection: CollectionOperation::Add { id: "circularSaw".into(), item: circular_saw_machine(), at: 0 } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_remove() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Machines { collection: CollectionOperation::Remove { id: "circularSaw".into() } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_move() {
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Machines { collection: CollectionOperation::Move { id: "circularSaw".into(), to: 2 } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_patch_full() {
        use process_3d::WorkshopMachinePatch;
        let patch = WorkshopMachinePatch { label: Some("Big Saw".into()), icon_id: Some("scissors".into()), capabilities: Some(circular_saw_machine().capabilities) };
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Machines { collection: CollectionOperation::Patch { id: "circularSaw".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_patch_empty() {
        use process_3d::WorkshopMachinePatch;
        store::test_support::assert_op_line_round_trip(&Process3dOperation::Machines { collection: CollectionOperation::Patch { id: "circularSaw".into(), patch: WorkshopMachinePatch::default() } });
    }

    #[test]
    fn backwards_of_machines_add_is_remove() {
        let projection = empty_process3d_projection();
        let operation = Process3dOperation::Machines { collection: CollectionOperation::Add { id: "circularSaw".into(), item: circular_saw_machine(), at: 0 } };
        let inverse = operation.backwards(&projection);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dOperation::Machines { collection: CollectionOperation::Remove { id } } => assert_eq!(id, "circularSaw"),
            _ => panic!("expected Machines::Remove"),
        }
    }
    //#endregion 🔖️OpTextTests

    //#region 🔖️ConfigOperationsTests
    #[test]
    fn process3d_config_operation_backwards_is_always_a_snapshot_of_base() {
        let base = process_3d_engine::Process3dConfig::default();
        let operation = Process3dConfigOperation::SetSelectedId { value: Some("step-0".into()) };
        let inverse = operation.backwards(&base);
        assert_eq!(inverse, vec![Process3dConfigOperation::Snapshot { config: base }]);
    }

    #[test]
    fn process3d_config_operation_diff_applies_expected_fields() {
        let base = process_3d_engine::Process3dConfig::default();
        let next = Process3dConfigOperation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 }.diff(&base);
        assert_eq!(next.camera_position, [1.0, 2.0, 3.0]);
        assert_eq!(next.camera_target, [0.1, 0.2, 0.3]);
        assert_eq!(next.camera_fov, 60.0);

        let next = Process3dConfigOperation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() }.diff(&base);
        assert!(next.sun_enabled);
        assert_eq!(next.sun_azimuth, 10.0);
        assert_eq!(next.sun_elevation, 20.0);
        assert_eq!(next.sun_intensity, 0.5);
        assert_eq!(next.sun_color, "#123456");
    }

    #[test]
    fn process3d_config_op_text_round_trips_every_variant() {
        let config = process_3d_engine::Process3dConfig { selected_id: Some("stock".into()), hovered_id: Some("step-0".into()), selected_face_id: Some(2), active_utility_id: "cut".into(), ..process_3d_engine::Process3dConfig::default() };
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedId { value: Some("stock".into()) });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedId { value: None });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetHoveredId { value: Some("step-0".into()) });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedFaceId { value: Some(3) });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSelectedFaceId { value: None });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetEngagementInput { value: "cut".into() });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetActiveUtility { utility_id: "cut".into() });
        store::test_support::assert_op_line_round_trip(&Process3dConfigOperation::SetLocale { value: "de-DE".into() });
    }
    //#endregion 🔖️ConfigOperationsTests
}
//#endregion 🧪️Tests

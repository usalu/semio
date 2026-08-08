//! 🔧️ Process3d artifact — OpText/OpBinary codecs + grammar for serializing `Process3dMutation`.
//! Mutation apply/inverse live in `🧬️mutations`; this facet only handcrafts the op wire forms.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


pub use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Process3dSnapshot, ProcessMeasure, ProcessStep, ProcessStepPatch, StepOrigin, Stock, WorkshopMachine, WorkshopMachinePatch};
use protocol::{CollectionMutation, OpText};
use serde::{Deserialize, Serialize};

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
    },
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

/// ✂️ Local DSL-only mirror of `Process3dMutation` — `protocol::CollectionMutation<K,V,P>` is
/// declared in the `protocol` crate (foreign type), so it cannot itself gain a
/// `dsl::DslField`/`dsl::DslVariants` binding here (orphan rule: neither the trait nor the type is
/// local to this crate). This twin flattens the `Steps { collection }` wrapper into its own four
/// keyworded variants — mirroring `imperative_core::ImperativeOperationDsl`'s identical fix for the
/// same foreign-`CollectionMutation` problem — and converts at the `store::OpText` boundary only;
/// `Process3dMutation` itself, and every consumer matching on it, is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Process3dMutationDsl {
    StepsAdd {
        index: usize,
        #[dsl(block)]
        item: ProcessStep,
    },
    StepsRemove {
        id: String,
    },
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
    MachinesRemove {
        id: String,
    },
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
    SetCursor {
        value: Option<usize>,
    },
    #[dsl(key = "snapshot")]
    SetSnapshot {
        #[dsl(block)]
        snapshot: Process3dSnapshot,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Process3dMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Process3dMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn process3d_mutation_to_dsl(operation: &Process3dMutation) -> Process3dMutationDsl {
    match operation {
        Process3dMutation::Steps { collection: CollectionMutation::Add { index: at, item } } => Process3dMutationDsl::StepsAdd { index: *at, item: item.clone() },
        Process3dMutation::Steps { collection: CollectionMutation::Remove { id } } => Process3dMutationDsl::StepsRemove { id: id.clone() },
        Process3dMutation::Steps { collection: CollectionMutation::Move { id, to_index: to } } => Process3dMutationDsl::StepsMove { id: id.clone(), to_index: *to },
        Process3dMutation::Steps { collection: CollectionMutation::Patch { id, patch } } => Process3dMutationDsl::StepsPatch { id: id.clone(), patch: process_step_patch_to_dsl(patch) },
        Process3dMutation::Machines { collection: CollectionMutation::Add { index: at, item } } => Process3dMutationDsl::MachinesAdd { index: *at, item: item.clone() },
        Process3dMutation::Machines { collection: CollectionMutation::Remove { id } } => Process3dMutationDsl::MachinesRemove { id: id.clone() },
        Process3dMutation::Machines { collection: CollectionMutation::Move { id, to_index: to } } => Process3dMutationDsl::MachinesMove { id: id.clone(), to_index: *to },
        Process3dMutation::Machines { collection: CollectionMutation::Patch { id, patch } } => Process3dMutationDsl::MachinesPatch { id: id.clone(), patch: patch.clone() },
        Process3dMutation::SetStock { stock } => Process3dMutationDsl::SetStock { stock: stock.clone() },
        Process3dMutation::SetCursor { resolved_up_to } => Process3dMutationDsl::SetCursor { value: *resolved_up_to },
        Process3dMutation::SetSnapshot { snapshot } => Process3dMutationDsl::SetSnapshot { snapshot: snapshot.clone() },
    }
}

fn process3d_mutation_from_dsl(operation: Process3dMutationDsl) -> Process3dMutation {
    match operation {
        Process3dMutationDsl::StepsAdd { index, item } => Process3dMutation::Steps { collection: CollectionMutation::Add { index: index, item } },
        Process3dMutationDsl::StepsRemove { id } => Process3dMutation::Steps { collection: CollectionMutation::Remove { id } },
        Process3dMutationDsl::StepsMove { id, to_index } => Process3dMutation::Steps { collection: CollectionMutation::Move { id, to_index: to_index } },
        Process3dMutationDsl::StepsPatch { id, patch } => Process3dMutation::Steps { collection: CollectionMutation::Patch { id, patch: process_step_patch_from_dsl(patch) } },
        Process3dMutationDsl::MachinesAdd { index, item } => Process3dMutation::Machines { collection: CollectionMutation::Add { index: index, item } },
        Process3dMutationDsl::MachinesRemove { id } => Process3dMutation::Machines { collection: CollectionMutation::Remove { id } },
        Process3dMutationDsl::MachinesMove { id, to_index } => Process3dMutation::Machines { collection: CollectionMutation::Move { id, to_index: to_index } },
        Process3dMutationDsl::MachinesPatch { id, patch } => Process3dMutation::Machines { collection: CollectionMutation::Patch { id, patch } },
        Process3dMutationDsl::SetStock { stock } => Process3dMutation::SetStock { stock },
        Process3dMutationDsl::SetCursor { value } => Process3dMutation::SetCursor { resolved_up_to: value },
        Process3dMutationDsl::SetSnapshot { snapshot } => Process3dMutation::SetSnapshot { snapshot },
    }
}

impl OpText for Process3dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(process3d_mutation_from_dsl(<Process3dMutationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <Process3dMutationDsl as OpText>::print_op(&process3d_mutation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Process3dMutationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for Process3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        process3d_mutation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(process3d_mutation_from_dsl(Process3dMutationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::Mutation;
    use crate::artifacts::process3d::{empty_process3d_snapshot, Pose, SolidSpec, Workshop};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn imported_mesh_stock() -> Stock {
        Stock { id: "stock".into(), label: "Imported GLB".into(), solid: SolidSpec::ImportedMesh { mesh_url: "data:model/gltf-binary;base64,AAAA".into() }, pose: Pose::default() }
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

    fn attach_step(id: &str) -> ProcessStep {
        ProcessStep {
            id: id.into(),
            label: "Attach".into(),
            enabled: false,
            origin: None,
            measure: ProcessMeasure::Attach { component: SolidSpec::Sphere { radius: 0.05 }, pose: Pose { position: [0.1, -0.2, 0.3], axis: [0.0, 1.0, 0.0], angle: 1.2 } },
        }
    }

    fn circular_saw_machine() -> WorkshopMachine {
        use crate::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, StockQuantity};
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
    fn sample_document() -> Process3dSnapshot {
        Process3dSnapshot {
            workshop: Workshop { machines: vec![circular_saw_machine()] },
            stock: Stock { id: "beam".into(), label: "Timber Beam".into(), solid: SolidSpec::Box { width: 2.4, depth: 0.12, height: 0.24 }, pose: Pose { position: [0.0, 0.0, 0.12], axis: [0.0, 0.0, 1.0], angle: 0.0 } },
            steps: vec![cut_step("cut-1"), drill_step("drill-1"), attach_step("attach-1")],
            resolved_up_to: Some(2),
        }
    }

    #[test]
    fn inverse_of_add_is_remove() {
        let snapshot = empty_process3d_snapshot();
        let operation = Process3dMutation::Steps { collection: CollectionMutation::Add { index: 0, item: cut_step("a") } };
        let inverse = operation.inverse(&snapshot);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dMutation::Steps { collection: CollectionMutation::Remove { id } } => assert_eq!(id, "a"),
            _ => panic!("expected Steps::Remove"),
        }
    }

    //#region 🔖️OpTextTests
    #[test]
    fn process3d_op_text_round_trips_steps_add() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Steps { collection: CollectionMutation::Add { index: 0, item: cut_step("cut-1") } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_remove() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Steps { collection: CollectionMutation::Remove { id: "cut-1".into() } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_move() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Steps { collection: CollectionMutation::Move { id: "cut-1".into(), to_index: 2 } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_full() {
        let patch = ProcessStepPatch {
            label: Some("Renamed".into()),
            enabled: Some(false),
            measure: Some(ProcessMeasure::Drill { radius: 0.03, depth: 0.4, pose: Pose { position: [1.0, 2.0, 3.0], axis: [0.0, 1.0, 0.0], angle: 0.7 } }),
            origin: Some(Some(StepOrigin { machine_id: "tableSaw".into(), capability_id: "crosscut".into() })),
        };
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Steps { collection: CollectionMutation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_clearing_origin() {
        let patch = ProcessStepPatch { label: None, enabled: None, measure: None, origin: Some(None) };
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Steps { collection: CollectionMutation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_steps_patch_empty() {
        let patch = ProcessStepPatch::default();
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Steps { collection: CollectionMutation::Patch { id: "cut-1".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_set_stock() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::SetStock { stock: imported_mesh_stock() });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_some() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::SetCursor { resolved_up_to: Some(3) });
    }

    #[test]
    fn process3d_op_text_round_trips_set_cursor_none() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::SetCursor { resolved_up_to: None });
    }

    #[test]
    fn process3d_op_text_round_trips_set_snapshot() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::SetSnapshot { snapshot: sample_document() });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_add() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Machines { collection: CollectionMutation::Add { index: 0, item: circular_saw_machine() } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_remove() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Machines { collection: CollectionMutation::Remove { id: "circularSaw".into() } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_move() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Machines { collection: CollectionMutation::Move { id: "circularSaw".into(), to_index: 2 } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_patch_full() {
        let patch = WorkshopMachinePatch { label: Some("Big Saw".into()), icon_id: Some("scissors".into()), capabilities: Some(circular_saw_machine().capabilities) };
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Machines { collection: CollectionMutation::Patch { id: "circularSaw".into(), patch } });
    }

    #[test]
    fn process3d_op_text_round_trips_machines_patch_empty() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dMutation::Machines { collection: CollectionMutation::Patch { id: "circularSaw".into(), patch: WorkshopMachinePatch::default() } });
    }

    #[test]
    fn inverse_of_machines_add_is_remove() {
        let snapshot = empty_process3d_snapshot();
        let operation = Process3dMutation::Machines { collection: CollectionMutation::Add { index: 0, item: circular_saw_machine() } };
        let inverse = operation.inverse(&snapshot);
        assert_eq!(inverse.len(), 1);
        match &inverse[0] {
            Process3dMutation::Machines { collection: CollectionMutation::Remove { id } } => assert_eq!(id, "circularSaw"),
            _ => panic!("expected Machines::Remove"),
        }
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests

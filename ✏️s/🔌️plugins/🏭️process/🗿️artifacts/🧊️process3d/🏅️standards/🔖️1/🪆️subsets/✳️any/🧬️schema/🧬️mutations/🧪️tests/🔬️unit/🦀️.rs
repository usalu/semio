use super::*;
use crate::{
    brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot, process_working_scene_to_snapshot, Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, StepOrigin,
    Stock, StockQuantity, WorkingSolid, Workshop, WorkshopMachine,
};
use change_machine_icon::ChangeMachineIcon;
use change_step_enabled::ChangeStepEnabled;
use change_step_origin::ChangeStepOrigin;
use change_stock_label::ChangeStockLabel;
use create_machine::CreateMachine;
use create_step::CreateStep;
use delete_machine::DeleteMachine;
use delete_step::DeleteStep;
use move_stock::MoveStock;
use protocol::Mutation;
use protocol::SemanticMutation;
use rename_machine::RenameMachine;
use rename_step::RenameStep;
use reorder_steps::ReorderSteps;
use replace_machine_capabilities::ReplaceMachineCapabilities;
use replace_step_measure::ReplaceStepMeasure;
use replace_stock_solid::ReplaceStockSolid;

fn cut_step(id: &str) -> ProcessStep {
    ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
}

fn saw_machine(id: &str) -> WorkshopMachine {
    WorkshopMachine { id: id.into(), label: "Saw".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: vec![] }
}

fn round_trip(base: &Process3dSnapshot, mutation: &Process3dMutation) -> Process3dSnapshot {
    let (forward, _messages) = protocol::apply_mutation(base, mutation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in mutation.inverse(base).expect("valid retained mutation inverse fixture") {
        let (next, _messages) = protocol::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

/// ⚖️ One value per `Process3dMutation` variant — the closed set the semantics test iterates.
fn every_mutation() -> Vec<Process3dMutation> {
    vec![
        Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-fresh") }),
        Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() }),
        Process3dMutation::RenameStep(RenameStep { id: "step-1".into(), new_label: "Renamed".into() }),
        Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "step-1".into(), new_enabled: false }),
        Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "step-1".into(), new_origin: Some(StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() }) }),
        Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: "step-1".into(), new_measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() } }),
        Process3dMutation::ReorderSteps(ReorderSteps { id: "step-1".into(), to_index: 0 }),
        Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-fresh") }),
        Process3dMutation::DeleteMachine(DeleteMachine { id: "machine-1".into() }),
        Process3dMutation::RenameMachine(RenameMachine { id: "machine-1".into(), new_label: "Renamed".into() }),
        Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: "machine-1".into(), new_icon_id: "drill".into() }),
        Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: "machine-1".into(), new_capabilities: vec![] }),
        Process3dMutation::MoveStock(MoveStock { new_pose: Pose { position: [1.0, 0.0, 0.0], ..Pose::default() } }),
        Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }),
        Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Sphere { radius: 0.5 })) }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn every_variant_registers_an_approved_semantic_descriptor() {
    for mutation in every_mutation() {
        let descriptor = SemanticMutation::semantics(&mutation);
        assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
    }
    assert_eq!(<Process3dMutation as SemanticMutation<Process3dSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
}

//#region 🔖️StepMutations
/// 🌱 Ticket `26/09/01/PROCESS-END-TO-END`: `step_payloads` is the durable, inline timeline
/// record (`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4) — the composed `steps`/
/// `tool_solids` children carry composition identity only, re-minted from it via
/// `process3d_step_timeline_diff`. These seven verbs are real mutations against it, mirroring
/// the id-keyed `machine` tests below one-for-one.
fn base_with_steps(steps: Vec<ProcessStep>) -> Process3dSnapshot {
    process_working_scene_to_snapshot(&ProcessWorkingScene { stock: Stock::default(), steps }, Workshop::default())
}

#[semio_framework_async_macros::async_test]
async fn create_step_round_trips() {
    let base = empty_process3d_snapshot();
    let after = round_trip(&base, &Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-9") }));
    assert!(after.step_payloads.iter().any(|step| step.id == "step-9"));
}

#[semio_framework_async_macros::async_test]
async fn delete_step_round_trips() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let after = round_trip(&base, &Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() }));
    assert!(!after.step_payloads.iter().any(|step| step.id == "step-1"));
}

#[semio_framework_async_macros::async_test]
async fn inverse_delete_step_when_missing_returns_empty() {
    let base = empty_process3d_snapshot();
    assert!(Process3dMutation::DeleteStep(DeleteStep { id: "ghost".into() }).inverse(&base).expect("valid retained mutation inverse fixture").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn rename_step_round_trips() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let after = round_trip(&base, &Process3dMutation::RenameStep(RenameStep { id: "step-1".into(), new_label: "Big Cut".into() }));
    assert_eq!(after.step_payloads.iter().find(|step| step.id == "step-1").expect("step-1 present").label, "Big Cut");
}

#[semio_framework_async_macros::async_test]
async fn change_step_enabled_round_trips() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let after = round_trip(&base, &Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "step-1".into(), new_enabled: false }));
    assert!(!after.step_payloads.iter().find(|step| step.id == "step-1").expect("step-1 present").enabled);
}

#[semio_framework_async_macros::async_test]
async fn change_step_origin_round_trips() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let origin = StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() };
    let after = round_trip(&base, &Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "step-1".into(), new_origin: Some(origin.clone()) }));
    assert_eq!(after.step_payloads.iter().find(|step| step.id == "step-1").expect("step-1 present").origin, Some(origin));
}

#[semio_framework_async_macros::async_test]
async fn replace_step_measure_round_trips() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let new_measure = ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() };
    let after = round_trip(&base, &Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: "step-1".into(), new_measure: new_measure.clone() }));
    assert_eq!(after.step_payloads.iter().find(|step| step.id == "step-1").expect("step-1 present").measure, new_measure);
    assert!(after.tool_solids.is_empty(), "a Drill step mints no tool solid");
}

#[semio_framework_async_macros::async_test]
async fn reorder_steps_round_trips() {
    let base = base_with_steps(vec![cut_step("step-a"), cut_step("step-b")]);
    let after = round_trip(&base, &Process3dMutation::ReorderSteps(ReorderSteps { id: "step-b".into(), to_index: 0 }));
    assert_eq!(after.step_payloads.first().expect("first step present").id, "step-b");
}
//#endregion 🔖️StepMutations

#[semio_framework_async_macros::async_test]
async fn create_machine_round_trips() {
    let base = empty_process3d_snapshot();
    let after = round_trip(&base, &Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-9") }));
    assert!(after.workshop.machines.iter().any(|machine| machine.id == "machine-9"));
}

#[semio_framework_async_macros::async_test]
async fn delete_machine_round_trips() {
    let mut base = empty_process3d_snapshot();
    base.workshop.machines.push(saw_machine("machine-1"));
    let after = round_trip(&base, &Process3dMutation::DeleteMachine(DeleteMachine { id: "machine-1".into() }));
    assert!(!after.workshop.machines.iter().any(|machine| machine.id == "machine-1"));
}

#[semio_framework_async_macros::async_test]
async fn inverse_delete_machine_when_missing_returns_empty() {
    let base = empty_process3d_snapshot();
    assert!(Process3dMutation::DeleteMachine(DeleteMachine { id: "ghost".into() }).inverse(&base).expect("valid retained mutation inverse fixture").is_empty());
}

#[semio_framework_async_macros::async_test]
async fn rename_machine_round_trips() {
    let mut base = empty_process3d_snapshot();
    base.workshop.machines.push(saw_machine("machine-1"));
    let after = round_trip(&base, &Process3dMutation::RenameMachine(RenameMachine { id: "machine-1".into(), new_label: "Big Saw".into() }));
    assert_eq!(after.workshop.machines.iter().find(|machine| machine.id == "machine-1").expect("machine-1 present").label, "Big Saw");
}

#[semio_framework_async_macros::async_test]
async fn change_machine_icon_round_trips() {
    let mut base = empty_process3d_snapshot();
    base.workshop.machines.push(saw_machine("machine-1"));
    let after = round_trip(&base, &Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: "machine-1".into(), new_icon_id: "drill".into() }));
    assert_eq!(after.workshop.machines.iter().find(|machine| machine.id == "machine-1").expect("machine-1 present").icon_id, "drill");
}

#[semio_framework_async_macros::async_test]
async fn replace_machine_capabilities_round_trips() {
    let mut base = empty_process3d_snapshot();
    base.workshop.machines.push(saw_machine("machine-1"));
    let after = round_trip(&base, &Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: "machine-1".into(), new_capabilities: vec![] }));
    assert!(after.workshop.machines.iter().find(|machine| machine.id == "machine-1").expect("machine-1 present").capabilities.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn move_stock_round_trips() {
    let base = empty_process3d_snapshot();
    let new_pose = Pose { position: [1.0, 2.0, 3.0], ..Pose::default() };
    let after = round_trip(&base, &Process3dMutation::MoveStock(MoveStock { new_pose: new_pose.clone() }));
    assert_eq!(after.stock_pose, new_pose);
}

#[semio_framework_async_macros::async_test]
async fn change_stock_label_round_trips() {
    let base = empty_process3d_snapshot();
    let after = round_trip(&base, &Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }));
    assert_eq!(after.stock_label, "Beam");
}

#[semio_framework_async_macros::async_test]
async fn replace_stock_solid_round_trips() {
    let base = empty_process3d_snapshot();
    let new_handle = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Sphere { radius: 0.5 }));
    let after = round_trip(&base, &Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: new_handle.clone() }));
    assert_eq!(after.stock_solid, new_handle);
}

//#region 🧪️MutationLaws
/// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs`
/// (reachable here as `protocol::os_spr::protocol_laws` — the bare `protocol::os_spr::protocol_laws` path is
/// ambiguous: the kernel root glob-reexports both `os_pack::*` and `os_spr::*`, and both mount
/// a `test context` module), exercised against the three most structurally
/// distinct new variants: an id-keyed create/delete pair on an ordered collection
/// (`create-step`), an id-keyed create/delete pair on an unordered collection
/// (`create-machine`), and a document-level facet setter (`change-stock-label`).
#[semio_framework_async_macros::async_test]
async fn create_step_satisfies_the_inverse_and_absorb_laws() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-fresh") });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }).diff(&base).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn create_machine_satisfies_the_inverse_and_absorb_laws() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-fresh") });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }).diff(&base).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}

#[semio_framework_async_macros::async_test]
async fn change_stock_label_satisfies_the_inverse_and_absorb_laws() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    let d1 = mutation.diff(&base).into_parts().0;
    let d2 = Process3dMutation::MoveStock(MoveStock { new_pose: Pose { position: [0.0, 0.0, 1.0], ..Pose::default() } }).diff(&base).into_parts().0;
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, d1, d2).await;
}
//#endregion 🧪️MutationLaws

//#region 🔖️OutcomeLaws
/// ✅️ 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C2 laws — one per
/// representative verb family across `machine`s and `step`s, both id-keyed:
/// `assert_missing_target_is_error`/`assert_fatal_never_applies` below,
/// `assert_outcome_policy_matrix` cases further down (delete, rename, create).
#[semio_framework_async_macros::async_test]
async fn delete_machine_missing_target_is_an_error() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::DeleteMachine(DeleteMachine { id: "does-not-exist".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_machine_missing_target_is_an_error() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::RenameMachine(RenameMachine { id: "does-not-exist".into(), new_label: "X".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_machine_duplicate_id_is_fatal_and_never_applies() {
    let mut base = empty_process3d_snapshot();
    base.workshop.machines.push(saw_machine("machine-1"));
    let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-1") });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&outcome).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_machine_outcome_obeys_the_policy_matrix() {
    let mut base = empty_process3d_snapshot();
    base.workshop.machines.push(saw_machine("machine-1"));
    let mutation = Process3dMutation::DeleteMachine(DeleteMachine { id: "machine-1".into() });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_machine_outcome_obeys_the_policy_matrix() {
    let mut base = empty_process3d_snapshot();
    base.workshop.machines.push(saw_machine("machine-1"));
    let mutation = Process3dMutation::RenameMachine(RenameMachine { id: "machine-1".into(), new_label: "X".into() });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_machine_outcome_obeys_the_policy_matrix() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-fresh") });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_step_missing_target_is_an_error() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::DeleteStep(DeleteStep { id: "does-not-exist".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_step_missing_target_is_an_error() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::RenameStep(RenameStep { id: "does-not-exist".into(), new_label: "X".into() });
    protocol::os_spr::protocol_laws::assert_missing_target_is_error(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_step_duplicate_id_is_fatal_and_never_applies() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-1") });
    let outcome = mutation.diff(&base);
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&outcome).await;
}

#[semio_framework_async_macros::async_test]
async fn delete_step_outcome_obeys_the_policy_matrix() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let mutation = Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn rename_step_outcome_obeys_the_policy_matrix() {
    let base = base_with_steps(vec![cut_step("step-1")]);
    let mutation = Process3dMutation::RenameStep(RenameStep { id: "step-1".into(), new_label: "X".into() });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}

#[semio_framework_async_macros::async_test]
async fn create_step_outcome_obeys_the_policy_matrix() {
    let base = empty_process3d_snapshot();
    let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-fresh") });
    protocol::os_spr::protocol_laws::assert_outcome_policy_matrix(&base, &mutation).await;
}
//#endregion 🔖️OutcomeLaws



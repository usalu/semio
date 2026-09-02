//! 🧬️ Process3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Process3dSnapshot`'s shape (`workshop.machines: Vec<WorkshopMachine>`,
//! `stock: Stock`, `steps: Vec<ProcessStep>`, `resolved_up_to: Option<usize>`) per
//! `📓️derivation-rules.md`: an id-keyed, order-meaningful `steps` timeline
//! (`create`/`delete`/`rename`/`change-*-enabled`/`change-*-origin`/`replace-*-measure`/
//! `reorder-steps`), an id-keyed, unordered `machines` set
//! (`create`/`delete`/`rename`/`change-*-icon`/`replace-*-capabilities`), the document's single
//! `stock` facet split into its spatial (`move-stock`), identity (`change-stock-label`), and large
//! structured (`replace-stock-solid`) fields, and one document-level scalar (`change-cursor`).
//! Every variant wraps exactly one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Process3dSnapshot, Process3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to
//! each payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! The whole-collection `Steps { collection: ... }` / `Machines { collection: ... }` / `SetStock` /
//! `SetCursor` / whole-document-replacement variants — the pre-migration generic vocabulary — are
//! gone. Whole-document replacement has NO replacement mutation (it is banned; file-open/import/
//! load-example goes through `store::ArtifactStore::reset`, outside this enum).
//!
//! Every triad-leaf directory now carries its target slug (`kind` name, emoji stripped) exactly —
//! the five directories that used to repurpose pre-migration names (`⏱️set-cursor` → `⏱️change-cursor`,
//! `📄set-snapshot` → `📐replace-step-measure`, `📋steps` → `🌱create-step`, `🛠️machines` →
//! `🏭create-machine`, `🧱set-stock` → `📍move-stock`) were renamed, and every duplicate emoji among
//! the fresh leaves was reassigned a unique one within this facet, as part of this ticket's
//! directory + glue trueing pass. See this facet's migration report for the emoji table.

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::Process3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MutationLeaves
// 🌱️ Every `🧬️mutations/<kind>/` triad leaf is `#[path]`-mounted as a sibling of this dispatch file
// directly in the plugin's `🦀️.rs` (this facet's fan-out ticket, SEMANTIC-MUTATIONS-OVERHAUL
// wave-C, owns `🦀️.rs` for this plugin); `use super::<kind>;` below brings each sibling into
// this file's scope so the enum body can reference `<kind>::<Type>`.
use super::change_cursor;
use super::change_machine_icon;
use super::change_step_enabled;
use super::change_step_origin;
use super::change_stock_label;
use super::create_machine;
use super::create_step;
use super::delete_machine;
use super::delete_step;
use super::move_stock;
use super::rename_machine;
use super::rename_step;
use super::reorder_steps;
use super::replace_machine_capabilities;
use super::replace_step_measure;
use super::replace_stock_solid;
//#endregion 🔖️MutationLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the process3d document, derived per
/// `📓️derivation-rules.md` from `Process3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Process3dSnapshot, diff = Process3dDiff, schema = "process.process3d")]
pub enum Process3dMutation {
    CreateStep(create_step::CreateStep),
    DeleteStep(delete_step::DeleteStep),
    RenameStep(rename_step::RenameStep),
    ChangeStepEnabled(change_step_enabled::ChangeStepEnabled),
    ChangeStepOrigin(change_step_origin::ChangeStepOrigin),
    ReplaceStepMeasure(replace_step_measure::ReplaceStepMeasure),
    ReorderSteps(reorder_steps::ReorderSteps),
    CreateMachine(create_machine::CreateMachine),
    DeleteMachine(delete_machine::DeleteMachine),
    RenameMachine(rename_machine::RenameMachine),
    ChangeMachineIcon(change_machine_icon::ChangeMachineIcon),
    ReplaceMachineCapabilities(replace_machine_capabilities::ReplaceMachineCapabilities),
    MoveStock(move_stock::MoveStock),
    ChangeStockLabel(change_stock_label::ChangeStockLabel),
    ReplaceStockSolid(replace_stock_solid::ReplaceStockSolid),
    ChangeCursor(change_cursor::ChangeCursor),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::process3d::{brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot, process_working_scene_to_snapshot, Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, StepOrigin, Stock, StockQuantity, WorkingSolid, Workshop, WorkshopMachine};
    use change_cursor::ChangeCursor;
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
        for back in mutation.inverse(base) {
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
            Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(1) }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<Process3dMutation as protocol::SemanticMutation<Process3dSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    //#region 🔖️StepMutations
    /// 🌱 Ticket `26/09/01/PROCESS-END-TO-END`: `step_payloads` is the durable, inline timeline
    /// record (`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4) — the composed `steps`/
    /// `tool_solids` children carry composition identity only, re-minted from it via
    /// `process3d_step_timeline_diff`. These seven verbs are real mutations against it, mirroring
    /// the id-keyed `machine` tests below one-for-one.
    fn base_with_steps(steps: Vec<ProcessStep>) -> Process3dSnapshot {
        process_working_scene_to_snapshot(&ProcessWorkingScene { stock: Stock::default(), steps }, Workshop::default(), None)
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
        assert!(Process3dMutation::DeleteStep(DeleteStep { id: "ghost".into() }).inverse(&base).is_empty());
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
        assert!(Process3dMutation::DeleteMachine(DeleteMachine { id: "ghost".into() }).inverse(&base).is_empty());
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

    #[semio_framework_async_macros::async_test]
    async fn change_cursor_round_trips() {
        let base = empty_process3d_snapshot();
        let after = round_trip(&base, &Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(0) }));
        assert_eq!(after.resolved_up_to, Some(0));
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️.rs`
    /// (reachable here as `protocol::os_spr::testkit` — the bare `protocol::testkit` path is
    /// ambiguous: the kernel root glob-reexports both `os_pack::*` and `os_spr::*`, and both mount
    /// a `testkit` module), exercised against the three most structurally
    /// distinct new variants: an id-keyed create/delete pair on an ordered collection
    /// (`create-step`), an id-keyed create/delete pair on an unordered collection
    /// (`create-machine`), and a document-level facet setter (`change-stock-label`).
    #[semio_framework_async_macros::async_test]
    async fn create_step_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-fresh") });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }).diff(&base).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn create_machine_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-fresh") });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(1) }).diff(&base).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn change_stock_label_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() });
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &mutation).await;
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(2) }).diff(&base).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, d1, d2).await;
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
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_machine_missing_target_is_an_error() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::RenameMachine(RenameMachine { id: "does-not-exist".into(), new_label: "X".into() });
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn create_machine_duplicate_id_is_fatal_and_never_applies() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-1") });
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::os_dsl::Severity::Fatal));
        protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_machine_outcome_obeys_the_policy_matrix() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let mutation = Process3dMutation::DeleteMachine(DeleteMachine { id: "machine-1".into() });
        protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_machine_outcome_obeys_the_policy_matrix() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let mutation = Process3dMutation::RenameMachine(RenameMachine { id: "machine-1".into(), new_label: "X".into() });
        protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn create_machine_outcome_obeys_the_policy_matrix() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-fresh") });
        protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_missing_target_is_an_error() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::DeleteStep(DeleteStep { id: "does-not-exist".into() });
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_step_missing_target_is_an_error() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::RenameStep(RenameStep { id: "does-not-exist".into(), new_label: "X".into() });
        protocol::os_spr::testkit::assert_missing_target_is_error(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn create_step_duplicate_id_is_fatal_and_never_applies() {
        let base = base_with_steps(vec![cut_step("step-1")]);
        let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-1") });
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::os_dsl::Severity::Fatal));
        protocol::os_spr::testkit::assert_fatal_never_applies(&outcome).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_outcome_obeys_the_policy_matrix() {
        let base = base_with_steps(vec![cut_step("step-1")]);
        let mutation = Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() });
        protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_step_outcome_obeys_the_policy_matrix() {
        let base = base_with_steps(vec![cut_step("step-1")]);
        let mutation = Process3dMutation::RenameStep(RenameStep { id: "step-1".into(), new_label: "X".into() });
        protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn create_step_outcome_obeys_the_policy_matrix() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-fresh") });
        protocol::os_spr::testkit::assert_outcome_policy_matrix(&base, &mutation).await;
    }
    //#endregion 🔖️OutcomeLaws

    //#region 🔖️FixtureRegeneration
    /// 🌉️ Regenerates the seven step-scoped mutation vectors via the REAL mutation pipeline (never
    /// hand-transcribed JSON), writing each `(before, mutation, after, diff, outcome)` quintet to
    /// the ticket's `🗑️generated` folder for manual copy into the matching `🧬️mutations/<verb>/
    /// 🧪️tests/…` fixture directory. `#[ignore]`d: a one-shot authoring tool, not part of the
    /// regular test run — mirrors `📸️snapshot/📝️text/🦀️.rs`'s own
    /// `regenerate_example_fixtures`.
    fn write_vector(dir: &std::path::Path, kind: &str, before: &Process3dSnapshot, mutation: &Process3dMutation) {
        let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(mutation, before);
        let mut after = before.clone();
        let forward = outcome.apply_to(&mut after);
        let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(mutation, before);
        let mut undone = after.clone();
        for step in &inverse {
            <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        }
        assert_eq!(&undone, before, "regenerate-{kind}: inverse must restore before");
        let messages: Vec<semio_framework_os_kernel::json::Value> = forward
            .messages()
            .iter()
            .map(|message| {
                let level = semio_framework_os_kernel::ToValue::to_value(&message.level).as_str().expect("severity is a string").to_string();
                semio_framework_os_kernel::json::object([("level".to_string(), semio_framework_os_kernel::json::Value::String(level)), ("code".to_string(), semio_framework_os_kernel::json::Value::String(message.code.0.clone()))])
            })
            .collect();
        let outcome_json = semio_framework_os_kernel::json::object([("status".to_string(), semio_framework_os_kernel::json::Value::String("applied".to_string())), ("messages".to_string(), semio_framework_os_kernel::json::array(messages))]);
        std::fs::write(dir.join(format!("{kind}.before.json")), semio_framework_os_kernel::json::to_json_string(before)).expect("write before");
        std::fs::write(dir.join(format!("{kind}.mutation.json")), semio_framework_os_kernel::json::to_json_string(mutation)).expect("write mutation");
        std::fs::write(dir.join(format!("{kind}.after.json")), semio_framework_os_kernel::json::to_json_string(&after)).expect("write after");
        std::fs::write(dir.join(format!("{kind}.diff.json")), semio_framework_os_kernel::json::to_json_string(forward.diff())).expect("write diff");
        std::fs::write(dir.join(format!("{kind}.outcome.json")), semio_framework_os_kernel::json::to_string(&outcome_json)).expect("write outcome");
    }

    #[semio_framework_async_macros::async_test]
    #[ignore]
    async fn regenerate_step_mutation_vectors() {
        let dir = std::path::Path::new("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/PROCESS-END-TO-END/🗑️generated");
        let workshop = Workshop { machines: vec![saw_machine("saw")] };
        let rip_cut = ProcessStep { id: "step-1".into(), label: "Rip Cut".into(), enabled: true, origin: Some(StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() }), measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.5, depth: 0.006, height: 0.1 }, pose: Pose::default() } };
        let rip_cut_no_origin = ProcessStep { origin: None, ..rip_cut.clone() };
        let bore_hole = ProcessStep { id: "step-2".into(), label: "Bore Hole".into(), enabled: true, origin: Some(StepOrigin { machine_id: "saw".into(), capability_id: "drill".into() }), measure: ProcessMeasure::Drill { radius: 0.008, depth: 0.02, pose: Pose::default() } };
        let scene = |steps: Vec<ProcessStep>| ProcessWorkingScene { stock: Stock::default(), steps };

        let base = process_working_scene_to_snapshot(&scene(vec![]), workshop.clone(), None);
        write_vector(dir, "create-step", &base, &Process3dMutation::CreateStep(CreateStep { index: 0, step: rip_cut.clone() }));

        let base = process_working_scene_to_snapshot(&scene(vec![rip_cut.clone()]), workshop.clone(), None);
        write_vector(dir, "delete-step", &base, &Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() }));

        let base = process_working_scene_to_snapshot(&scene(vec![rip_cut.clone()]), workshop.clone(), None);
        write_vector(dir, "rename-step", &base, &Process3dMutation::RenameStep(RenameStep { id: "step-1".into(), new_label: "Final Rip Cut".into() }));

        let base = process_working_scene_to_snapshot(&scene(vec![rip_cut.clone()]), workshop.clone(), None);
        write_vector(dir, "change-step-enabled", &base, &Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "step-1".into(), new_enabled: false }));

        let base = process_working_scene_to_snapshot(&scene(vec![rip_cut_no_origin]), workshop.clone(), None);
        let new_origin = StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() };
        write_vector(dir, "change-step-origin", &base, &Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "step-1".into(), new_origin: Some(new_origin) }));

        let base = process_working_scene_to_snapshot(&scene(vec![rip_cut.clone()]), workshop.clone(), None);
        let bore_measure = ProcessMeasure::Drill { radius: 0.008, depth: 0.02, pose: Pose::default() };
        write_vector(dir, "replace-step-measure", &base, &Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: "step-1".into(), new_measure: bore_measure }));

        let base = process_working_scene_to_snapshot(&scene(vec![rip_cut, bore_hole]), workshop, None);
        write_vector(dir, "reorder-steps", &base, &Process3dMutation::ReorderSteps(ReorderSteps { id: "step-2".into(), to_index: 0 }));
    }

    /// 🌱 Ticket `26/09/01/PROCESS-END-TO-END`: the nine machine/stock/cursor mutation vectors predate
    /// wave 4's `stock_payload`/`step_payloads` fields and no longer round-trip. Same technique as
    /// `regenerate_step_mutation_vectors` above — a real `ProcessWorkingScene` through
    /// `process_working_scene_to_snapshot`, the mutation's own `diff`/`apply`/`inverse`, one written
    /// quintet per verb. Each `before` scene is built to make its fixture directory's NAME literally
    /// true; mutation payloads are copied verbatim from the committed (still schema-valid)
    /// `🦠️mutation/🔣️.json` for that fixture.
    #[semio_framework_async_macros::async_test]
    #[ignore]
    async fn regenerate_machine_stock_cursor_mutation_vectors() {
        let dir = std::path::Path::new("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/PROCESS-END-TO-END/🗑️generated");
        let cut_capability = Capability {
            id: "cut".into(),
            label: "Cut".into(),
            icon_id: "scissors".into(),
            recipe: MeasureRecipe::BladeCut { kerf: "kerf".into(), length: "length".into(), depth: "depth".into() },
            parameters: vec![
                CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.0625 },
                CapabilityParameter { id: "length".into(), label: "Length".into(), value: 0.5 },
                CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 0.5 },
            ],
            rules: Vec::new(),
        };
        let workshop = Workshop { machines: vec![WorkshopMachine { id: "saw".into(), label: "Bench Saw".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: vec![cut_capability] }] };
        let stock = Stock { id: "stock-1".into(), label: "Oak Beam".into(), solid: WorkingSolid::Box { width: 1.0, depth: 1.0, height: 1.0 }, pose: Pose::default() };
        let empty_scene = |stock: Stock| ProcessWorkingScene { stock, steps: vec![] };

        //#region 🔖️CreateMachine — adds-a-drill-press-to-the-workshop
        let drill_press = WorkshopMachine {
            id: "drill-press".into(),
            label: "Drill Press".into(),
            icon_id: "circle-dot".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "bore".into(),
                label: "Bore".into(),
                icon_id: "circle-dot".into(),
                recipe: MeasureRecipe::BoreDrill { radius: "radius".into(), depth: "depth".into() },
                parameters: vec![CapabilityParameter { id: "radius".into(), label: "Radius".into(), value: 0.0625 }, CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 0.25 }],
                rules: Vec::new(),
            }],
        };
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        write_vector(dir, "create-machine", &base, &Process3dMutation::CreateMachine(CreateMachine { index: 1, machine: drill_press }));
        //#endregion 🔖️CreateMachine

        //#region 🔖️DeleteMachine — empties-the-workshop-of-the-saw
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        write_vector(dir, "delete-machine", &base, &Process3dMutation::DeleteMachine(DeleteMachine { id: "saw".into() }));
        //#endregion 🔖️DeleteMachine

        //#region 🔖️RenameMachine — retitles-the-saw
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        write_vector(dir, "rename-machine", &base, &Process3dMutation::RenameMachine(RenameMachine { id: "saw".into(), new_label: "Panel Saw".into() }));
        //#endregion 🔖️RenameMachine

        //#region 🔖️ChangeMachineIcon — swaps-the-saw-icon
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        write_vector(dir, "change-machine-icon", &base, &Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: "saw".into(), new_icon_id: "saw-blade".into() }));
        //#endregion 🔖️ChangeMachineIcon

        //#region 🔖️ReplaceMachineCapabilities — trades-the-blade-cut-for-a-gated-pocket-cut
        let pocket_capability = Capability {
            id: "pocket".into(),
            label: "Pocket".into(),
            icon_id: "square".into(),
            recipe: MeasureRecipe::PocketCut { diameter: "diameter".into(), depth: "depth".into() },
            parameters: vec![CapabilityParameter { id: "diameter".into(), label: "Diameter".into(), value: 0.125 }, CapabilityParameter { id: "depth".into(), label: "Depth".into(), value: 0.25 }],
            rules: vec![CapabilityRule::Min { quantity: StockQuantity::Width, parameter: "diameter".into(), margin: 0.0625 }],
        };
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        write_vector(dir, "replace-machine-capabilities", &base, &Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: "saw".into(), new_capabilities: vec![pocket_capability] }));
        //#endregion 🔖️ReplaceMachineCapabilities

        //#region 🔖️MoveStock — lifts-and-tilts-the-stock
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        let lifted_pose = Pose { position: [0.0, 0.0, 1.5], axis: [1.0, 0.0, 0.0], angle: 0.5 };
        write_vector(dir, "move-stock", &base, &Process3dMutation::MoveStock(MoveStock { new_pose: lifted_pose }));
        //#endregion 🔖️MoveStock

        //#region 🔖️ChangeStockLabel — relabels-the-oak-beam-as-planed
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        write_vector(dir, "change-stock-label", &base, &Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Oak Beam, planed".into() }));
        //#endregion 🔖️ChangeStockLabel

        //#region 🔖️ReplaceStockSolid — reissues-the-stock-brep-child-handle
        let base = process_working_scene_to_snapshot(&empty_scene(stock.clone()), workshop.clone(), None);
        let planed_solid_handle = store::ArtifactChild::new(
            "brep-stock-02".to_string(),
            store::os_io::ArtifactRef { artifact_id: "stock-1-solid-planed".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() } },
        );
        write_vector(dir, "replace-stock-solid", &base, &Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: planed_solid_handle }));
        //#endregion 🔖️ReplaceStockSolid

        //#region 🔖️ChangeCursor — pins-the-replay-cursor-to-two-steps
        let rip_cut = ProcessStep { id: "step-1".into(), label: "Rip Cut".into(), enabled: true, origin: Some(StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() }), measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.5, depth: 0.006, height: 0.1 }, pose: Pose::default() } };
        let bore_hole = ProcessStep { id: "step-2".into(), label: "Bore Hole".into(), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.05, depth: 0.2, pose: Pose::default() } };
        let attach_dowel = ProcessStep { id: "step-3".into(), label: "Attach Dowel".into(), enabled: true, origin: None, measure: ProcessMeasure::Attach { component: WorkingSolid::Cylinder { radius: 0.03, height: 0.2 }, pose: Pose::default() } };
        let base = process_working_scene_to_snapshot(&ProcessWorkingScene { stock, steps: vec![rip_cut, bore_hole, attach_dowel] }, workshop, None);
        write_vector(dir, "change-cursor", &base, &Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(2) }));
        //#endregion 🔖️ChangeCursor
    }
    //#endregion 🔖️FixtureRegeneration
}
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Process3dMutation` variant, in declaration order — the vocabulary the `process3d-1-any` mutation catalog
/// (`../../🧪️oracle/🔣️.json`) declares and the `mutate-process3d-1` exhaustive test case measures
/// itself against. The framework never parses Rust, so `kinds_match_the_enum_and_the_catalog` below is
/// what keeps this list honest in both directions.
pub const KINDS: &[&str] = &[
    "create-step",
    "delete-step",
    "rename-step",
    "change-step-enabled",
    "change-step-origin",
    "replace-step-measure",
    "reorder-steps",
    "create-machine",
    "delete-machine",
    "rename-machine",
    "change-machine-icon",
    "replace-machine-capabilities",
    "move-stock",
    "change-stock-label",
    "replace-stock-solid",
    "change-cursor",
];
//#endregion 🔖️Kinds

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying `mutation_json` to `base_json`, for a language-neutral test adapter.
///
/// A generated test host links only `semio-repo-test-host` and, behind its `sut` feature, this crate —
/// no `serde`, no `serde_json` and no `protocol` is reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Process3dMutation` nor
/// `Process3dSnapshot` can be named there, and hand-transcribing either into a Rust literal
/// would be a second copy of the committed specification vector, free to drift away from it. This
/// bridge is the whole surface an adapter needs, and every type in its signature is a `str`.
///
/// `after_json` is decoded through the SAME path as `base_json` and returned as `expectedSnapshot`,
/// so the caller compares like with like. The report carries the forward half (`base`, `snapshot`,
/// `diff`, `messages`) and the inverse half (`inverseSteps`, `inverseSnapshot`, `inverseMessages`),
/// so the inverse law is checked against the mutation's OWN computed inverse rather than against a
/// hand-written undo.
///
/// @see ../../🧪️oracle/🔣️.json — the catalog and the recorded no-oracle decision.
pub fn process3d_mutation_report_json(base_json: &str, mutation_json: &str, after_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Process3dSnapshot, String> {
        let decoded: Process3dSnapshot = semio_framework_os_kernel::json::from_json_str(text).map_err(|error| error.to_string())?;
        Ok(decoded)
    };
    let base = decode_snapshot(base_json)?;
    let expected = decode_snapshot(after_json)?;
    let mutation: Process3dMutation = semio_framework_os_kernel::json::from_json_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = semio_framework_os_kernel::json::object([
        ("base".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&base))),
        ("expectedSnapshot".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&expected))),
        ("snapshot".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&applied))),
        ("diff".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(forward.diff()))),
        ("messages".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&forward.messages().to_vec()))),
        ("inverseSteps".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&inverse))),
        ("inverseSnapshot".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&undone))),
        ("inverseMessages".to_string(), semio_framework_os_kernel::json::from_dsl_value(&semio_framework_os_kernel::ToValue::to_value(&inverse_messages))),
    ]);
    Ok(semio_framework_os_kernel::json::to_string(&report))
}
//#endregion 🌉️TestBridge

//#region 🧪️KindsConformance
#[cfg(test)]
mod kinds_conformance {
    use super::*;

    /// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
    /// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
    /// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
    /// honest in both directions at once.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let descriptors = <Process3dMutation as protocol::SemanticMutation<Process3dSnapshot>>::kinds();
        assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance

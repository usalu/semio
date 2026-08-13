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
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs` for the reference shape.
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
use serde::{Deserialize, Serialize};

//#region 🔖️MutationLeaves
// 🌱️ Every `🧬️mutations/<kind>/` triad leaf is `#[path]`-mounted as a sibling of this dispatch file
// directly in the plugin's `📦️glue.rs` (this facet's fan-out ticket, SEMANTIC-MUTATIONS-OVERHAUL
// wave-C, owns `📦️glue.rs` for this plugin); `use super::<kind>;` below brings each sibling into
// this file's scope so the enum body can reference `<kind>::mutation::<Type>`.
use super::create_step;
use super::delete_step;
use super::rename_step;
use super::change_step_enabled;
use super::change_step_origin;
use super::replace_step_measure;
use super::reorder_steps;
use super::create_machine;
use super::delete_machine;
use super::rename_machine;
use super::change_machine_icon;
use super::replace_machine_capabilities;
use super::move_stock;
use super::change_stock_label;
use super::replace_stock_solid;
use super::change_cursor;
//#endregion 🔖️MutationLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the process3d document, derived per
/// `📓️derivation-rules.md` from `Process3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = Process3dSnapshot, diff = Process3dDiff, schema = "process.process3d")]
pub enum Process3dMutation {
    CreateStep(create_step::mutation::CreateStep),
    DeleteStep(delete_step::mutation::DeleteStep),
    RenameStep(rename_step::mutation::RenameStep),
    ChangeStepEnabled(change_step_enabled::mutation::ChangeStepEnabled),
    ChangeStepOrigin(change_step_origin::mutation::ChangeStepOrigin),
    ReplaceStepMeasure(replace_step_measure::mutation::ReplaceStepMeasure),
    ReorderSteps(reorder_steps::mutation::ReorderSteps),
    CreateMachine(create_machine::mutation::CreateMachine),
    DeleteMachine(delete_machine::mutation::DeleteMachine),
    RenameMachine(rename_machine::mutation::RenameMachine),
    ChangeMachineIcon(change_machine_icon::mutation::ChangeMachineIcon),
    ReplaceMachineCapabilities(replace_machine_capabilities::mutation::ReplaceMachineCapabilities),
    MoveStock(move_stock::mutation::MoveStock),
    ChangeStockLabel(change_stock_label::mutation::ChangeStockLabel),
    ReplaceStockSolid(replace_stock_solid::mutation::ReplaceStockSolid),
    ChangeCursor(change_cursor::mutation::ChangeCursor),
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    use crate::artifacts::process3d::{brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot, Pose, ProcessMeasure, ProcessStep, StepOrigin, WorkingSolid, WorkshopMachine};
    use change_machine_icon::mutation::ChangeMachineIcon;
    use change_step_enabled::mutation::ChangeStepEnabled;
    use change_step_origin::mutation::ChangeStepOrigin;
    use change_stock_label::mutation::ChangeStockLabel;
    use delete_machine::mutation::DeleteMachine;
    use delete_step::mutation::DeleteStep;
    use create_machine::mutation::CreateMachine;
    use protocol::Mutation;
    use rename_machine::mutation::RenameMachine;
    use rename_step::mutation::RenameStep;
    use replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
    use replace_stock_solid::mutation::ReplaceStockSolid;
    use reorder_steps::mutation::ReorderSteps;
    use change_cursor::mutation::ChangeCursor;
    use replace_step_measure::mutation::ReplaceStepMeasure;
    use move_stock::mutation::MoveStock;
    use create_step::mutation::CreateStep;

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn saw_machine(id: &str) -> WorkshopMachine {
        WorkshopMachine { id: id.into(), label: "Saw".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: vec![] }
    }

    fn round_trip(base: &Process3dSnapshot, mutation: &Process3dMutation) -> Process3dSnapshot {
        let forward = vcs::apply_mutation(base, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            restored = vcs::apply_mutation(&restored, &back);
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

    #[test]
    fn every_variant_registers_an_approved_semantic_descriptor() {
        for mutation in every_mutation() {
            let descriptor = protocol::SemanticMutation::semantics(&mutation);
            assert!(protocol::is_approved_verb(descriptor.verb), "unapproved verb {:?} on {mutation:?}", descriptor.verb);
        }
        assert_eq!(<Process3dMutation as protocol::SemanticMutation<Process3dSnapshot>>::kinds().len(), every_mutation().len(), "kinds() must register exactly one descriptor per dispatch variant");
    }

    //#region 🔖️StepMutationsAreDocumentedNoOps
    /// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `steps` composes an
    /// `s.stdio.semio.flow` CHILD HANDLE now — no inline `Vec<ProcessStep>` for these 7 mutations
    /// to edit, and no `LinkResolver` to read the child's content back through (see
    /// `🌱create-step/🔺️diff/🦀️component.rs`'s doc comment). Each is now a REAL, honest no-op:
    /// `diff()` returns `Process3dDiff::default()`, `inverse()` returns `Vec::new()` — the
    /// sanctioned `MutationKind::inverse` contract for "nothing changed, nothing to undo". These
    /// tests assert exactly that, matching `📐️cad`'s own precedent
    /// (`add_object_action_is_a_documented_no_op_pending_the_child_dispatch_seam`).
    #[test]
    fn create_step_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-9") });
        assert_eq!(mutation.diff(&base), Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn delete_step_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() });
        assert_eq!(mutation.diff(&base), Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn rename_step_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::RenameStep(RenameStep { id: "step-1".into(), new_label: "Renamed".into() });
        assert_eq!(mutation.diff(&base), Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn change_step_enabled_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "step-1".into(), new_enabled: false });
        assert_eq!(mutation.diff(&base), Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn change_step_origin_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let origin = StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() };
        let mutation = Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "step-1".into(), new_origin: Some(origin) });
        assert_eq!(mutation.diff(&base), Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn replace_step_measure_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let new_measure = ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() };
        let mutation = Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: "step-1".into(), new_measure });
        assert_eq!(mutation.diff(&base), Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[test]
    fn reorder_steps_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::ReorderSteps(ReorderSteps { id: "b".into(), to_index: 0 });
        assert_eq!(mutation.diff(&base), Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }
    //#endregion 🔖️StepMutationsAreDocumentedNoOps

    #[test]
    fn create_machine_round_trips() {
        let base = empty_process3d_snapshot();
        let after = round_trip(&base, &Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-9") }));
        assert!(after.workshop.machines.iter().any(|machine| machine.id == "machine-9"));
    }

    #[test]
    fn delete_machine_round_trips() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let after = round_trip(&base, &Process3dMutation::DeleteMachine(DeleteMachine { id: "machine-1".into() }));
        assert!(!after.workshop.machines.iter().any(|machine| machine.id == "machine-1"));
    }

    #[test]
    fn inverse_delete_machine_when_missing_returns_empty() {
        let base = empty_process3d_snapshot();
        assert!(Process3dMutation::DeleteMachine(DeleteMachine { id: "ghost".into() }).inverse(&base).is_empty());
    }

    #[test]
    fn rename_machine_round_trips() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let after = round_trip(&base, &Process3dMutation::RenameMachine(RenameMachine { id: "machine-1".into(), new_label: "Big Saw".into() }));
        assert_eq!(after.workshop.machines[0].label, "Big Saw");
    }

    #[test]
    fn change_machine_icon_round_trips() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let after = round_trip(&base, &Process3dMutation::ChangeMachineIcon(ChangeMachineIcon { id: "machine-1".into(), new_icon_id: "drill".into() }));
        assert_eq!(after.workshop.machines[0].icon_id, "drill");
    }

    #[test]
    fn replace_machine_capabilities_round_trips() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let after = round_trip(&base, &Process3dMutation::ReplaceMachineCapabilities(ReplaceMachineCapabilities { id: "machine-1".into(), new_capabilities: vec![] }));
        assert!(after.workshop.machines[0].capabilities.is_empty());
    }

    #[test]
    fn move_stock_round_trips() {
        let base = empty_process3d_snapshot();
        let new_pose = Pose { position: [1.0, 2.0, 3.0], ..Pose::default() };
        let after = round_trip(&base, &Process3dMutation::MoveStock(MoveStock { new_pose: new_pose.clone() }));
        assert_eq!(after.stock_pose, new_pose);
    }

    #[test]
    fn change_stock_label_round_trips() {
        let base = empty_process3d_snapshot();
        let after = round_trip(&base, &Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }));
        assert_eq!(after.stock_label, "Beam");
    }

    #[test]
    fn replace_stock_solid_round_trips() {
        let base = empty_process3d_snapshot();
        let new_handle = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Sphere { radius: 0.5 }));
        let after = round_trip(&base, &Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: new_handle.clone() }));
        assert_eq!(after.stock_solid, new_handle);
    }

    #[test]
    fn change_cursor_round_trips() {
        let base = empty_process3d_snapshot();
        let after = round_trip(&base, &Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(0) }));
        assert_eq!(after.resolved_up_to, Some(0));
    }

    //#region 🧪️MutationLaws
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::testkit`), exercised against the three most structurally
    /// distinct new variants: an id-keyed create/delete pair on an ordered collection
    /// (`create-step`), an id-keyed create/delete pair on an unordered collection
    /// (`create-machine`), and a document-level facet setter (`change-stock-label`).
    #[test]
    fn create_step_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-fresh") });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn create_machine_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-fresh") });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(1) }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn change_stock_label_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base);
        let d2 = Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(2) }).diff(&base);
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws
}
//#endregion 🧪️Tests

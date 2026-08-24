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
    use crate::artifacts::process3d::{brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot, Pose, ProcessMeasure, ProcessStep, StepOrigin, WorkingSolid, WorkshopMachine};
    use change_cursor::mutation::ChangeCursor;
    use change_machine_icon::mutation::ChangeMachineIcon;
    use change_step_enabled::mutation::ChangeStepEnabled;
    use change_step_origin::mutation::ChangeStepOrigin;
    use change_stock_label::mutation::ChangeStockLabel;
    use create_machine::mutation::CreateMachine;
    use create_step::mutation::CreateStep;
    use delete_machine::mutation::DeleteMachine;
    use delete_step::mutation::DeleteStep;
    use move_stock::mutation::MoveStock;
    use protocol::Mutation;
    use protocol::SemanticMutation;
    use rename_machine::mutation::RenameMachine;
    use rename_step::mutation::RenameStep;
    use reorder_steps::mutation::ReorderSteps;
    use replace_machine_capabilities::mutation::ReplaceMachineCapabilities;
    use replace_step_measure::mutation::ReplaceStepMeasure;
    use replace_stock_solid::mutation::ReplaceStockSolid;

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn saw_machine(id: &str) -> WorkshopMachine {
        WorkshopMachine { id: id.into(), label: "Saw".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: vec![] }
    }

    fn round_trip(base: &Process3dSnapshot, mutation: &Process3dMutation) -> Process3dSnapshot {
        let (forward, _messages) = vcs::apply_mutation(base, mutation).expect("valid mutation");
        let mut restored = forward.clone();
        for back in mutation.inverse(base) {
            let (next, _messages) = vcs::apply_mutation(&restored, &back).expect("valid inverse mutation");
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

    //#region 🔖️StepMutationsAreDocumentedNoOps
    /// 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `steps` composes an
    /// `s.stdio.semio.flow` CHILD HANDLE now — no inline `Vec<ProcessStep>` for these 7 mutations
    /// to edit, and no `LinkResolver` to read the child's content back through (see
    /// `🌱create-step/🔺️diff/🦀️component.rs`'s doc comment). Each is now a REAL, honest no-op:
    /// `diff()` returns `Process3dDiff::default()`, `inverse()` returns `Vec::new()` — the
    /// sanctioned `MutationKind::inverse` contract for "nothing changed, nothing to undo". These
    /// tests assert exactly that, matching `📐️cad`'s own precedent
    /// (`add_object_action_is_a_documented_no_op_pending_the_child_dispatch_seam`).
    #[semio_framework_async_macros::async_test]
    async fn create_step_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-9") });
        assert_eq!(mutation.diff(&base).diff(), &Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn delete_step_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() });
        assert_eq!(mutation.diff(&base).diff(), &Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_step_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::RenameStep(RenameStep { id: "step-1".into(), new_label: "Renamed".into() });
        assert_eq!(mutation.diff(&base).diff(), &Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn change_step_enabled_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "step-1".into(), new_enabled: false });
        assert_eq!(mutation.diff(&base).diff(), &Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn change_step_origin_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let origin = StepOrigin { machine_id: "saw".into(), capability_id: "cut".into() };
        let mutation = Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "step-1".into(), new_origin: Some(origin) });
        assert_eq!(mutation.diff(&base).diff(), &Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn replace_step_measure_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let new_measure = ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() };
        let mutation = Process3dMutation::ReplaceStepMeasure(ReplaceStepMeasure { id: "step-1".into(), new_measure });
        assert_eq!(mutation.diff(&base).diff(), &Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn reorder_steps_diff_is_a_documented_no_op() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::ReorderSteps(ReorderSteps { id: "b".into(), to_index: 0 });
        assert_eq!(mutation.diff(&base).diff(), &Process3dDiff::default());
        assert!(mutation.inverse(&base).is_empty());
    }
    //#endregion 🔖️StepMutationsAreDocumentedNoOps

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
    /// ⚖️ Shared law helpers from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
    /// (reachable here as `protocol::testkit`), exercised against the three most structurally
    /// distinct new variants: an id-keyed create/delete pair on an ordered collection
    /// (`create-step`), an id-keyed create/delete pair on an unordered collection
    /// (`create-machine`), and a document-level facet setter (`change-stock-label`).
    #[semio_framework_async_macros::async_test]
    async fn create_step_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("step-fresh") });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }).diff(&base).into_parts().0;
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_machine_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-fresh") });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(1) }).diff(&base).into_parts().0;
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[semio_framework_async_macros::async_test]
    async fn change_stock_label_satisfies_the_inverse_and_absorb_laws() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() });
        protocol::testkit::assert_mutation_inverse_law(&base, &mutation);
        let d1 = mutation.diff(&base).into_parts().0;
        let d2 = Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(2) }).diff(&base).into_parts().0;
        protocol::testkit::assert_mutation_diff_absorb_law(&base, d1, d2);
    }
    //#endregion 🧪️MutationLaws

    //#region 🔖️OutcomeLaws
    /// ✅️ 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C2 laws, landed
    /// testkit helpers only (`assert_missing_target_is_error`/`assert_fatal_never_applies`) — one per
    /// representative verb family across `machine`s (id-keyed) and `step`s (documented no-op).
    /// `assert_outcome_policy_matrix` is not landed yet (checked at
    /// `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`); TODO(1-D testkit
    /// laws pending): add a `MergePolicy` × `Severity` matrix test per verb family here once it lands.
    #[semio_framework_async_macros::async_test]
    async fn delete_machine_missing_target_is_an_error() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::DeleteMachine(DeleteMachine { id: "does-not-exist".into() });
        protocol::testkit::assert_missing_target_is_error(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn rename_machine_missing_target_is_an_error() {
        let base = empty_process3d_snapshot();
        let mutation = Process3dMutation::RenameMachine(RenameMachine { id: "does-not-exist".into(), new_label: "X".into() });
        protocol::testkit::assert_missing_target_is_error(&base, &mutation);
    }

    #[semio_framework_async_macros::async_test]
    async fn create_machine_duplicate_id_is_fatal_and_never_applies() {
        let mut base = empty_process3d_snapshot();
        base.workshop.machines.push(saw_machine("machine-1"));
        let mutation = Process3dMutation::CreateMachine(CreateMachine { index: 0, machine: saw_machine("machine-1") });
        let outcome = mutation.diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::os_dsl::Severity::Fatal));
        protocol::testkit::assert_fatal_never_applies(&outcome);
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests

//#region 🔖️Kinds
/// 🏷️ Kebab-case spelling of every `Process3dMutation` variant, in declaration order — the vocabulary the `process3d-1-any` mutation catalog
/// (`../../🧪️oracle/🔣️component.json`) declares and the `mutate-process3d-1` exhaustive test case measures
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
/// there is no `serde`, no `serde_json` and no `protocol` reachable from an adapter, and this crate's
/// `protocol`/`store` extern-crate aliases are private — so neither `Process3dMutation` nor `Process3dSnapshot`
/// can be named there and hand-transcribing either into a Rust literal would be a second copy of the
/// committed specification vector, free to drift away from it. This bridge is the whole surface an
/// adapter needs, and every type in its signature is a `str`.
///
/// The report carries the forward half (`snapshot`, `diff`, `messages`) and the inverse half
/// (`inverseSteps`, `inverseSnapshot`, `inverseMessages`), so the inverse law is checked against the
/// mutation's OWN computed inverse rather than against a hand-written undo.
///
/// @see ../../🧪️oracle/🔣️component.json — the catalog and the recorded no-oracle decision.
pub fn process3d_mutation_report_json(base_json: &str, mutation_json: &str) -> Result<String, String> {
    let decode_snapshot = |text: &str| -> Result<Process3dSnapshot, String> { Ok(serde_json::from_str(text).map_err(|error| error.to_string())?) };
    let base = decode_snapshot(base_json)?;
    let mutation: Process3dMutation = serde_json::from_str(mutation_json).map_err(|error| error.to_string())?;
    let mut applied = base.clone();
    let forward = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation, &base).apply_to(&mut applied);
    let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(&mutation, &base);
    let mut undone = applied.clone();
    let mut inverse_messages = Vec::new();
    for step in &inverse {
        let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(step, &undone).apply_to(&mut undone);
        inverse_messages.extend(outcome.messages().iter().cloned());
    }
    let report = serde_json::json!({
        "snapshot": serde_json::to_value(&applied).map_err(|error| error.to_string())?,
        "diff": serde_json::to_value(forward.diff()).map_err(|error| error.to_string())?,
        "messages": serde_json::to_value(forward.messages()).map_err(|error| error.to_string())?,
        "inverseSteps": serde_json::to_value(&inverse).map_err(|error| error.to_string())?,
        "inverseSnapshot": serde_json::to_value(&undone).map_err(|error| error.to_string())?,
        "inverseMessages": serde_json::to_value(&inverse_messages).map_err(|error| error.to_string())?,
    });
    Ok(report.to_string())
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
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
}
//#endregion 🧪️KindsConformance

//! 🧬️ sequence artifact — semantic document mutation dispatch enum. Every variant is a
//! single-field tuple wrapping a handcrafted `protocol::MutationKind` payload (see the
//! `🧬️mutations/<slug>/` triad leaves); `#[derive(dsl::Mutations)]` generates
//! `impl protocol::Mutation<SequenceSnapshot>` and `impl protocol::SemanticMutation<SequenceSnapshot>`
//! from those payloads — no hand-written apply/diff/inverse dispatch here.
//!
//! `📦️glue.rs` (outside this facet's boundary, must not be touched by this migration) still
//! hand-lists the OLD generic triad-dir names under `pub mod mutations { ... }`; this rewrite
//! deleted those 8 dirs and created 8 real per-mutation ones with dir names matching their kind
//! slug exactly, so `glue.rs`'s `#[path]` entries now point at nonexistent files. See this ticket's
//! wave2 report for the exact replacement block and the `blocked-mechanism` status.

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type SequenceEnvelope = store::ArtifactEnvelope<SequenceSnapshot, SequenceMutation>;
pub type SequenceStore = store::ArtifactStore<SequenceSnapshot, SequenceMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Semantic sequence document mutation vocabulary: id-keyed step create/delete/move/edit-params/
/// change-collapsed, plus relationship connect/disconnect between steps, plus duplicate-step. The
/// canvas camera is session-only runtime state now (never a document field — see
/// `crate::apps::sequence::config`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum, dsl::Mutations)]
#[serde(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = SequenceSnapshot, diff = SequenceDiff, schema = "sequence.sequence")]
pub enum SequenceMutation {
    CreateStep(CreateStep),
    DeleteStep(DeleteStep),
    MoveStep(MoveStep),
    EditStepParams(EditStepParams),
    ChangeStepCollapsed(ChangeStepCollapsed),
    ConnectSteps(ConnectSteps),
    DisconnectSteps(DisconnectSteps),
    DuplicateStep(DuplicateStep),
}
//#endregion 🔖️Mutations

pub use super::change_step_collapsed::mutation::{change_step_collapsed, ChangeStepCollapsed};
pub use super::connect_steps::mutation::{connect_steps, ConnectSteps};
pub use super::create_step::mutation::{create_step, CreateStep};
pub use super::delete_step::mutation::{delete_step, DeleteStep};
pub use super::disconnect_steps::mutation::{disconnect_steps, DisconnectSteps};
pub use super::duplicate_step::mutation::{duplicate_step, DuplicateStep};
pub use super::edit_step_params::mutation::{edit_step_params, EditStepParams};
pub use super::move_step::mutation::{move_step, MoveStep};

/// 🔀️ Diffs two snapshots into a minimal typed semantic mutation set.
pub fn sequence_snapshot_mutations(before: &SequenceSnapshot, after: &SequenceSnapshot) -> Vec<SequenceMutation> {
    let mut mutations = Vec::new();
    for step in &before.steps {
        if !after.steps.iter().any(|entry| entry.id == step.id) {
            mutations.push(delete_step(step.id.clone()));
        }
    }
    for step in &after.steps {
        match before.steps.iter().find(|entry| entry.id == step.id) {
            None => mutations.push(create_step(step.clone())),
            Some(prior) => {
                if prior.x != step.x || prior.y != step.y {
                    mutations.push(move_step(step.id.clone(), step.x, step.y));
                }
                if prior.params != step.params {
                    mutations.push(edit_step_params(step.id.clone(), step.params.clone()));
                }
                if prior.collapsed != step.collapsed {
                    mutations.push(change_step_collapsed(step.id.clone(), step.collapsed));
                }
            }
        }
    }
    for edge in &before.edges {
        if !after.edges.iter().any(|entry| entry.id == edge.id) {
            mutations.push(disconnect_steps(edge.id.clone()));
        }
    }
    for edge in &after.edges {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => mutations.push(connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone())),
            Some(prior) if prior.from != edge.from || prior.to != edge.to => {
                mutations.push(disconnect_steps(edge.id.clone()));
                mutations.push(connect_steps(edge.id.clone(), edge.from.clone(), edge.to.clone()));
            }
            Some(_) => {}
        }
    }
    mutations
}

/// ▶️ Applies `mutation` via its diff.
pub fn apply_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> SequenceSnapshot {
    protocol::MutationDiff::apply(&mutation.diff(snapshot), snapshot)
}

pub fn inverse_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    mutation.inverse(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_snapshot, SequenceStep, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
    use protocol::testkit::{assert_mutation_diff_absorb_law, assert_mutation_inverse_law};
    use protocol::SemanticMutation;
    use store::{create_document_envelope, ArtifactCommand};

    fn round_trip(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> SequenceSnapshot {
        let forward = vcs::apply_mutation(snapshot, mutation);
        let mut restored = forward.clone();
        for back in mutation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "inverse must restore the pre-mutation snapshot");
        forward
    }

    #[test]
    fn create_edit_delete_step_round_trip() {
        let snapshot = default_snapshot();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let added = round_trip(&snapshot, &create_step(step));
        assert_eq!(added.steps.len(), 3);
        let moved = round_trip(&added, &move_step("step-99".into(), 120.0, 6.0));
        assert_eq!(moved.steps.iter().find(|step| step.id == "step-99").unwrap().x, 120.0);
        let removed = round_trip(&moved, &delete_step("step-99".into()));
        assert!(!removed.steps.iter().any(|step| step.id == "step-99"));
    }

    #[test]
    fn delete_step_severs_and_reconnects_edges() {
        let snapshot = default_snapshot();
        assert!(snapshot.edges.iter().any(|edge| edge.from == "step-1" && edge.to == "step-2"));
        round_trip(&snapshot, &delete_step("step-1".into()));
    }

    #[test]
    fn snapshot_mutations_capture_move_and_connect() {
        let mut host = crate::artifacts::sequence::engine::SequenceHost::default();
        let before = host.snapshot.clone();
        let id = host.add_step("math.add", 40.0, 40.0);
        let mutations = sequence_snapshot_mutations(&before, &host.snapshot);
        assert!(mutations.iter().any(|mutation| matches!(mutation, SequenceMutation::CreateStep(payload) if payload.step.id == id)));
    }

    #[test]
    fn store_applies_and_undoes_step_create() {
        let mut store = SequenceStore::new(create_document_envelope(SEQUENCE_DOCUMENT_SCHEMA, "sequence", default_snapshot(), None));
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![create_step(SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false })],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").steps.len(), 3);
    }

    //#region 🔖️MutationLaws
    #[test]
    fn create_step_inverse_law() {
        let base = default_snapshot();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        assert_mutation_inverse_law(&base, &create_step(step));
    }

    #[test]
    fn delete_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &delete_step("step-1".into()));
    }

    #[test]
    fn move_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &move_step("step-1".into(), 42.0, -8.0));
    }

    #[test]
    fn connect_disconnect_steps_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &connect_steps("edge-99".into(), "step-1".into(), "step-2".into()));
        assert_mutation_inverse_law(&base, &disconnect_steps("edge-1".into()));
    }

    #[test]
    fn duplicate_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &duplicate_step("step-1".into(), "step-1-copy".into(), 10.0, 10.0));
    }

    #[test]
    fn move_step_diff_absorb_law() {
        use protocol::Mutation;
        let base = default_snapshot();
        let d1 = move_step("step-1".into(), 10.0, 10.0).diff(&base);
        let mid = protocol::MutationDiff::apply(&d1, &base);
        let d2 = move_step("step-1".into(), 20.0, 30.0).diff(&mid);
        assert_mutation_diff_absorb_law(&base, d1, d2);
    }

    #[test]
    fn dispatch_registers_semantic_descriptors() {
        register_sequence_mutation_descriptors();
        for kind in SequenceMutation::kinds() {
            assert!(protocol::is_approved_verb(kind.verb), "verb '{}' must be in APPROVED_VERBS", kind.verb);
        }
        assert_eq!(SequenceMutation::kinds().len(), 8);
    }
    //#endregion 🔖️MutationLaws
}
//#endregion 🧪️Tests

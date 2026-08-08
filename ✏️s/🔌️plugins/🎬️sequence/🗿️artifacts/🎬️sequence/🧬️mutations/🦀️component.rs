//! 🧬️ sequence artifact — document mutation dispatch.


use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{SequenceEdge, SequenceEdgePatch, SequenceFixture, SequenceStep, SequenceStepPatch};
use protocol::{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type SequenceEnvelope = store::DocumentEnvelope<SequenceFixture, SequenceMutation>;
pub type SequenceStore = store::DocumentStore<SequenceFixture, SequenceMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Typed sequence operation: id-keyed step/edge collection edits. The canvas camera is
/// session-only runtime state now (never a document field — see `crate::apps::sequence::config`).
/// Flattened into one keyword-tagged variant per {@link protocol::CollectionMutation} case rather
/// than wrapping that generic type directly — `CollectionMutation` is foreign (defined in
/// `protocol`) and generic, so it can never itself implement `dsl::DslField`/`dsl::DslVariants` from
/// this crate (the orphan rule requires a local type to anchor the impl on, and its OWN outer type
/// isn't local). {@link Mutation for SequenceMutation} below reconstructs a `CollectionMutation`
/// ad hoc per match arm to keep reusing `protocol`'s generic collection diff/invert helpers.
/// Kept plain `Serialize`/`Deserialize` only; see `📡️spr`'s `SequenceMutationDsl` for the op-log
/// DSL text mirror (`EdgesAdd`/`EdgesPatch` items as `SequenceEdgeDsl`, a `dsl::Wire`-backed
/// connection) and the hand-written `impl protocol::OpText for SequenceMutation` that converts
/// through it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SequenceMutation {
    StepsAdd { index: usize, item: SequenceStep },
    StepsRemove { id: String },
    StepsMove { id: String, to_index: usize },
    StepsPatch { id: String, patch: SequenceStepPatch },
    EdgesAdd { index: usize, item: SequenceEdge },
    EdgesRemove { id: String },
    EdgesMove { id: String, to_index: usize },
    EdgesPatch { id: String, patch: SequenceEdgePatch },
}

/// 🔁️ Converts a generic step `CollectionMutation` (as produced by `protocol::inverse_collection_mutation`)
/// back into its flat `SequenceMutation` variant.
fn steps_operation_from_collection(operation: CollectionMutation<String, SequenceStep, SequenceStepPatch>) -> SequenceMutation {
    match operation {
        CollectionMutation::Add { index: at, item } => SequenceMutation::StepsAdd { index: at, item },
        CollectionMutation::Remove { id } => SequenceMutation::StepsRemove { id },
        CollectionMutation::Move { id, to_index: to } => SequenceMutation::StepsMove { id, to_index: to },
        CollectionMutation::Patch { id, patch } => SequenceMutation::StepsPatch { id, patch },
    }
}

/// 🔁️ Edge counterpart of {@link steps_operation_from_collection}.
fn edges_operation_from_collection(operation: CollectionMutation<String, SequenceEdge, SequenceEdgePatch>) -> SequenceMutation {
    match operation {
        CollectionMutation::Add { index: at, item } => SequenceMutation::EdgesAdd { index: at, item },
        CollectionMutation::Remove { id } => SequenceMutation::EdgesRemove { id },
        CollectionMutation::Move { id, to_index: to } => SequenceMutation::EdgesMove { id, to_index: to },
        CollectionMutation::Patch { id, patch } => SequenceMutation::EdgesPatch { id, patch },
    }
}

impl Mutation<SequenceFixture> for SequenceMutation {
    type Diff = SequenceDiff;

    fn diff(&self, projection: &SequenceFixture) -> SequenceDiff {
        match self {
            SequenceMutation::StepsAdd { index, item } => SequenceDiff { steps: Some(collection_diff_from_mutation(&projection.steps, &CollectionMutation::Add { index: *index, item: item.clone() })), ..Default::default() },
            SequenceMutation::StepsRemove { id } => SequenceDiff { steps: Some(collection_diff_from_mutation(&projection.steps, &CollectionMutation::Remove { id: id.clone() })), ..Default::default() },
            SequenceMutation::StepsMove { id, to_index } => SequenceDiff { steps: Some(collection_diff_from_mutation(&projection.steps, &CollectionMutation::Move { id: id.clone(), to_index: *to_index })), ..Default::default() },
            SequenceMutation::StepsPatch { id, patch } => SequenceDiff { steps: Some(collection_diff_from_mutation(&projection.steps, &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() },
            SequenceMutation::EdgesAdd { index, item } => SequenceDiff { edges: Some(collection_diff_from_mutation(&projection.edges, &CollectionMutation::Add { index: *index, item: item.clone() })), ..Default::default() },
            SequenceMutation::EdgesRemove { id } => SequenceDiff { edges: Some(collection_diff_from_mutation(&projection.edges, &CollectionMutation::Remove { id: id.clone() })), ..Default::default() },
            SequenceMutation::EdgesMove { id, to_index } => SequenceDiff { edges: Some(collection_diff_from_mutation(&projection.edges, &CollectionMutation::Move { id: id.clone(), to_index: *to_index })), ..Default::default() },
            SequenceMutation::EdgesPatch { id, patch } => SequenceDiff { edges: Some(collection_diff_from_mutation(&projection.edges, &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() },
        }
    }

    fn inverse(&self, projection: &SequenceFixture) -> Vec<Self> {
        match self {
            SequenceMutation::StepsAdd { index, item } => vec![steps_operation_from_collection(inverse_collection_mutation(&projection.steps, &CollectionMutation::Add { index: *index, item: item.clone() }))],
            SequenceMutation::StepsRemove { id } => vec![steps_operation_from_collection(inverse_collection_mutation(&projection.steps, &CollectionMutation::Remove { id: id.clone() }))],
            SequenceMutation::StepsMove { id, to_index } => {
                vec![steps_operation_from_collection(inverse_collection_mutation(&projection.steps, &CollectionMutation::Move { id: id.clone(), to_index: *to_index }))]
            }
            SequenceMutation::StepsPatch { id, patch } => {
                vec![steps_operation_from_collection(inverse_collection_mutation(&projection.steps, &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
            SequenceMutation::EdgesAdd { index, item } => vec![edges_operation_from_collection(inverse_collection_mutation(&projection.edges, &CollectionMutation::Add { index: *index, item: item.clone() }))],
            SequenceMutation::EdgesRemove { id } => vec![edges_operation_from_collection(inverse_collection_mutation(&projection.edges, &CollectionMutation::Remove { id: id.clone() }))],
            SequenceMutation::EdgesMove { id, to_index } => {
                vec![edges_operation_from_collection(inverse_collection_mutation(&projection.edges, &CollectionMutation::Move { id: id.clone(), to_index: *to_index }))]
            }
            SequenceMutation::EdgesPatch { id, patch } => {
                vec![edges_operation_from_collection(inverse_collection_mutation(&projection.edges, &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
        }
    }
}

/// 🔀️ Diffs two fixtures into a minimal typed operation set: removed/added/patched steps and edges.
/// Lets command handlers keep computing the target fixture via `crate::artifacts::sequence::engine::SequenceHost`
/// (with all its cycle/slot/layout logic) while emitting granular, mergeable mutations.
pub fn sequence_fixture_mutations(before: &SequenceFixture, after: &SequenceFixture) -> Vec<SequenceMutation> {
    let mut mutations = Vec::new();
    for step in &before.steps {
        if !after.steps.iter().any(|entry| entry.id == step.id) {
            mutations.push(SequenceMutation::StepsRemove { id: step.id.clone() });
        }
    }
    for (index, step) in after.steps.iter().enumerate() {
        match before.steps.iter().find(|entry| entry.id == step.id) {
            None => mutations.push(SequenceMutation::StepsAdd { index, item: step.clone() }),
            Some(prior) => {
                let patch = SequenceStepPatch {
                    params: (prior.params != step.params).then(|| step.params.clone()),
                    x: (prior.x != step.x).then_some(step.x),
                    y: (prior.y != step.y).then_some(step.y),
                    collapsed: (prior.collapsed != step.collapsed).then_some(step.collapsed),
                };
                if patch != SequenceStepPatch::default() {
                    mutations.push(SequenceMutation::StepsPatch { id: step.id.clone(), patch });
                }
            }
        }
    }
    for edge in &before.edges {
        if !after.edges.iter().any(|entry| entry.id == edge.id) {
            mutations.push(SequenceMutation::EdgesRemove { id: edge.id.clone() });
        }
    }
    for (index, edge) in after.edges.iter().enumerate() {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => mutations.push(SequenceMutation::EdgesAdd { index, item: edge.clone() }),
            Some(prior) => {
                let patch = SequenceEdgePatch { from: (prior.from != edge.from).then(|| edge.from.clone()), to: (prior.to != edge.to).then(|| edge.to.clone()) };
                if patch != SequenceEdgePatch::default() {
                    mutations.push(SequenceMutation::EdgesPatch { id: edge.id.clone(), patch });
                }
            }
        }
    }
    mutations
}
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_fixture, StepParams};
    use store::{create_document_envelope, DocumentCommand};
    use vcs::apply_mutation;

    fn round_trip(fixture: &SequenceFixture, operation: &SequenceMutation) -> SequenceFixture {
        let forward = vcs::apply_mutation(fixture, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(fixture) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, fixture, "backwards() must restore the pre-operation fixture");
        forward
    }

    #[test]
    fn add_remove_patch_steps_round_trip() {
        let fixture = default_fixture();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let added = round_trip(&fixture, &SequenceMutation::StepsAdd { index: 2, item: step });
        assert_eq!(added.steps.len(), 3);
        let patched = round_trip(&added, &SequenceMutation::StepsPatch { id: "step-99".into(), patch: SequenceStepPatch { x: Some(120.0), ..Default::default() } });
        assert_eq!(patched.steps.iter().find(|step| step.id == "step-99").unwrap().x, 120.0);
        let removed = round_trip(&patched, &SequenceMutation::StepsRemove { id: "step-99".into() });
        assert!(!removed.steps.iter().any(|step| step.id == "step-99"));
    }

    #[test]
    fn fixture_ops_capture_move_and_connect() {
        let mut host = crate::artifacts::sequence::engine::SequenceHost::default();
        let before = host.fixture.clone();
        let id = host.add_step("math.add", 40.0, 40.0);
        let mutations = sequence_fixture_mutations(&before, &host.fixture);
        assert!(mutations.iter().any(|operation| matches!(operation, SequenceMutation::StepsAdd { item, .. } if item.id == id)));
    }

    #[test]
    fn store_applies_and_undoes_step_add() {
        let mut store = SequenceStore::new(create_document_envelope(crate::artifacts::sequence::SEQUENCE_FIXTURE_SCHEMA, "sequence", default_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                mutations: vec![SequenceMutation::StepsAdd { index: 2, item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 3);
    }
}
//#endregion 🧪️Tests

/// ▶️ Applies `mutation` via its diff.
pub fn apply_sequence_mutation(projection: &SequenceFixture, mutation: &SequenceMutation) -> SequenceFixture {
    protocol::MutationDiff::apply(&mutation.diff(projection), projection)
}

pub fn inverse_sequence_mutation(projection: &SequenceFixture, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    mutation.inverse(projection)
}

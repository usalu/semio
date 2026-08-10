//! 🧬️ sequence artifact — document mutation dispatch.


use crate::artifacts::sequence::diff::{edges_delta_from_collection_mutation, steps_delta_from_collection_mutation, SequenceDiff};
use crate::artifacts::sequence::{SequenceEdge, SequenceEdgePatch, SequenceSnapshot, SequenceStep, SequenceStepPatch};
use protocol::{inverse_collection_mutation, CollectionMutation, Mutation};
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type SequenceEnvelope = store::ArtifactEnvelope<SequenceSnapshot, SequenceMutation>;
pub type SequenceStore = store::ArtifactStore<SequenceSnapshot, SequenceMutation>;
//#endregion 🔖️Store

//#region 🔖️Mutations
/// 🧮️ Typed sequence operation: id-keyed step/edge collection edits. The canvas camera is
/// session-only runtime state now (never a document field — see `crate::apps::sequence::config`).
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

fn steps_operation_from_collection(operation: CollectionMutation<String, SequenceStep, SequenceStepPatch>) -> SequenceMutation {
    match operation {
        CollectionMutation::Add { index: at, item } => SequenceMutation::StepsAdd { index: at, item },
        CollectionMutation::Remove { id } => SequenceMutation::StepsRemove { id },
        CollectionMutation::Move { id, to_index: to } => SequenceMutation::StepsMove { id, to_index: to },
        CollectionMutation::Patch { id, patch } => SequenceMutation::StepsPatch { id, patch },
    }
}

fn edges_operation_from_collection(operation: CollectionMutation<String, SequenceEdge, SequenceEdgePatch>) -> SequenceMutation {
    match operation {
        CollectionMutation::Add { index: at, item } => SequenceMutation::EdgesAdd { index: at, item },
        CollectionMutation::Remove { id } => SequenceMutation::EdgesRemove { id },
        CollectionMutation::Move { id, to_index: to } => SequenceMutation::EdgesMove { id, to_index: to },
        CollectionMutation::Patch { id, patch } => SequenceMutation::EdgesPatch { id, patch },
    }
}

impl Mutation<SequenceSnapshot> for SequenceMutation {
    type Diff = SequenceDiff;

    fn diff(&self, snapshot: &SequenceSnapshot) -> SequenceDiff {
        match self {
            SequenceMutation::StepsAdd { index, item } => SequenceDiff {
                steps: Some(steps_delta_from_collection_mutation(&snapshot.steps, &CollectionMutation::Add { index: *index, item: item.clone() })),
                ..Default::default()
            },
            SequenceMutation::StepsRemove { id } => SequenceDiff {
                steps: Some(steps_delta_from_collection_mutation(&snapshot.steps, &CollectionMutation::Remove { id: id.clone() })),
                ..Default::default()
            },
            SequenceMutation::StepsMove { id, to_index } => SequenceDiff {
                steps: Some(steps_delta_from_collection_mutation(
                    &snapshot.steps,
                    &CollectionMutation::Move { id: id.clone(), to_index: *to_index },
                )),
                ..Default::default()
            },
            SequenceMutation::StepsPatch { id, patch } => SequenceDiff {
                steps: Some(steps_delta_from_collection_mutation(
                    &snapshot.steps,
                    &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() },
                )),
                ..Default::default()
            },
            SequenceMutation::EdgesAdd { index, item } => SequenceDiff {
                edges: Some(edges_delta_from_collection_mutation(&snapshot.edges, &CollectionMutation::Add { index: *index, item: item.clone() })),
                ..Default::default()
            },
            SequenceMutation::EdgesRemove { id } => SequenceDiff {
                edges: Some(edges_delta_from_collection_mutation(&snapshot.edges, &CollectionMutation::Remove { id: id.clone() })),
                ..Default::default()
            },
            SequenceMutation::EdgesMove { id, to_index } => SequenceDiff {
                edges: Some(edges_delta_from_collection_mutation(
                    &snapshot.edges,
                    &CollectionMutation::Move { id: id.clone(), to_index: *to_index },
                )),
                ..Default::default()
            },
            SequenceMutation::EdgesPatch { id, patch } => SequenceDiff {
                edges: Some(edges_delta_from_collection_mutation(
                    &snapshot.edges,
                    &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() },
                )),
                ..Default::default()
            },
        }
    }

    fn inverse(&self, snapshot: &SequenceSnapshot) -> Vec<Self> {
        match self {
            SequenceMutation::StepsAdd { index, item } => vec![steps_operation_from_collection(inverse_collection_mutation(
                &snapshot.steps,
                &CollectionMutation::Add { index: *index, item: item.clone() },
            ))],
            SequenceMutation::StepsRemove { id } => vec![steps_operation_from_collection(inverse_collection_mutation(
                &snapshot.steps,
                &CollectionMutation::Remove { id: id.clone() },
            ))],
            SequenceMutation::StepsMove { id, to_index } => vec![steps_operation_from_collection(inverse_collection_mutation(
                &snapshot.steps,
                &CollectionMutation::Move { id: id.clone(), to_index: *to_index },
            ))],
            SequenceMutation::StepsPatch { id, patch } => vec![steps_operation_from_collection(inverse_collection_mutation(
                &snapshot.steps,
                &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() },
            ))],
            SequenceMutation::EdgesAdd { index, item } => vec![edges_operation_from_collection(inverse_collection_mutation(
                &snapshot.edges,
                &CollectionMutation::Add { index: *index, item: item.clone() },
            ))],
            SequenceMutation::EdgesRemove { id } => vec![edges_operation_from_collection(inverse_collection_mutation(
                &snapshot.edges,
                &CollectionMutation::Remove { id: id.clone() },
            ))],
            SequenceMutation::EdgesMove { id, to_index } => vec![edges_operation_from_collection(inverse_collection_mutation(
                &snapshot.edges,
                &CollectionMutation::Move { id: id.clone(), to_index: *to_index },
            ))],
            SequenceMutation::EdgesPatch { id, patch } => vec![edges_operation_from_collection(inverse_collection_mutation(
                &snapshot.edges,
                &CollectionMutation::Patch { id: id.clone(), patch: patch.clone() },
            ))],
        }
    }
}

/// 🔀️ Diffs two snapshots into a minimal typed operation set.
pub fn sequence_snapshot_mutations(before: &SequenceSnapshot, after: &SequenceSnapshot) -> Vec<SequenceMutation> {
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
                let patch = SequenceEdgePatch {
                    from: (prior.from != edge.from).then(|| edge.from.clone()),
                    to: (prior.to != edge.to).then(|| edge.to.clone()),
                };
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
    use crate::artifacts::sequence::{default_snapshot, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
    use store::{create_document_envelope, ArtifactCommand};
    use vcs::apply_mutation;

    fn round_trip(snapshot: &SequenceSnapshot, operation: &SequenceMutation) -> SequenceSnapshot {
        let forward = vcs::apply_mutation(snapshot, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = vcs::apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "backwards() must restore the pre-operation snapshot");
        forward
    }

    #[test]
    fn add_remove_patch_steps_round_trip() {
        let snapshot = default_snapshot();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let added = round_trip(&snapshot, &SequenceMutation::StepsAdd { index: 2, item: step });
        assert_eq!(added.steps.len(), 3);
        let patched = round_trip(&added, &SequenceMutation::StepsPatch { id: "step-99".into(), patch: SequenceStepPatch { x: Some(120.0), ..Default::default() } });
        assert_eq!(patched.steps.iter().find(|step| step.id == "step-99").unwrap().x, 120.0);
        let removed = round_trip(&patched, &SequenceMutation::StepsRemove { id: "step-99".into() });
        assert!(!removed.steps.iter().any(|step| step.id == "step-99"));
    }

    #[test]
    fn snapshot_ops_capture_move_and_connect() {
        let mut host = crate::artifacts::sequence::engine::SequenceHost::default();
        let before = host.snapshot.clone();
        let id = host.add_step("math.add", 40.0, 40.0);
        let mutations = sequence_snapshot_mutations(&before, &host.snapshot);
        assert!(mutations.iter().any(|operation| matches!(operation, SequenceMutation::StepsAdd { item, .. } if item.id == id)));
    }

    #[test]
    fn store_applies_and_undoes_step_add() {
        let mut store = SequenceStore::new(create_document_envelope(SEQUENCE_DOCUMENT_SCHEMA, "sequence", default_snapshot(), None));
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![SequenceMutation::StepsAdd {
                    index: 2,
                    item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false },
                }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.snapshot().expect("snapshot").steps.len(), 3);
    }
}
//#endregion 🧪️Tests

/// ▶️ Applies `mutation` via its diff.
pub fn apply_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> SequenceSnapshot {
    protocol::MutationDiff::apply(&mutation.diff(snapshot), snapshot)
}

pub fn inverse_sequence_mutation(snapshot: &SequenceSnapshot, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    mutation.inverse(snapshot)
}

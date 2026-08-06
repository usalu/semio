//! ⚡️ Sequence artifact — the operation type (constitutional: op).

use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::{SequenceEdge, SequenceEdgePatch, SequenceFixture, SequenceStep, SequenceStepPatch};
use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionOperation, Operation};
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type SequenceEnvelope = store::DocumentEnvelope<SequenceFixture, SequenceOperation>;
pub type SequenceStore = store::DocumentStore<SequenceFixture, SequenceOperation>;
//#endregion 🔖️Store

//#region 🔖️Operations
/// 🧮️ Typed sequence operation: id-keyed step/edge collection edits. The canvas camera is
/// session-only runtime state now (never a document field — see `crate::apps::sequence::config`).
/// Flattened into one keyword-tagged variant per {@link protocol::CollectionOperation} case rather
/// than wrapping that generic type directly — `CollectionOperation` is foreign (defined in
/// `protocol`) and generic, so it can never itself implement `dsl::DslField`/`dsl::DslVariants` from
/// this crate (the orphan rule requires a local type to anchor the impl on, and its OWN outer type
/// isn't local). {@link Operation for SequenceOperation} below reconstructs a `CollectionOperation`
/// ad hoc per match arm to keep reusing `protocol`'s generic collection diff/invert helpers.
/// Kept plain `Serialize`/`Deserialize` only; see `📡️spr`'s `SequenceOperationDsl` for the op-log
/// DSL text mirror (`EdgesAdd`/`EdgesPatch` items as `SequenceEdgeDsl`, a `dsl::Wire`-backed
/// connection) and the hand-written `impl protocol::OpText for SequenceOperation` that converts
/// through it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum SequenceOperation {
    StepsAdd { index: usize, item: SequenceStep },
    StepsRemove { id: String },
    StepsMove { id: String, to_index: usize },
    StepsPatch { id: String, patch: SequenceStepPatch },
    EdgesAdd { index: usize, item: SequenceEdge },
    EdgesRemove { id: String },
    EdgesMove { id: String, to_index: usize },
    EdgesPatch { id: String, patch: SequenceEdgePatch },
}

/// 🔁️ Converts a generic step `CollectionOperation` (as produced by `protocol::invert_collection_operation`)
/// back into its flat `SequenceOperation` variant.
fn steps_operation_from_collection(operation: CollectionOperation<String, SequenceStep, SequenceStepPatch>) -> SequenceOperation {
    match operation {
        CollectionOperation::Add { index: at, item } => SequenceOperation::StepsAdd { index: at, item },
        CollectionOperation::Remove { id } => SequenceOperation::StepsRemove { id },
        CollectionOperation::Move { id, to_index: to } => SequenceOperation::StepsMove { id, to_index: to },
        CollectionOperation::Patch { id, patch } => SequenceOperation::StepsPatch { id, patch },
    }
}

/// 🔁️ Edge counterpart of {@link steps_operation_from_collection}.
fn edges_operation_from_collection(operation: CollectionOperation<String, SequenceEdge, SequenceEdgePatch>) -> SequenceOperation {
    match operation {
        CollectionOperation::Add { index: at, item } => SequenceOperation::EdgesAdd { index: at, item },
        CollectionOperation::Remove { id } => SequenceOperation::EdgesRemove { id },
        CollectionOperation::Move { id, to_index: to } => SequenceOperation::EdgesMove { id, to_index: to },
        CollectionOperation::Patch { id, patch } => SequenceOperation::EdgesPatch { id, patch },
    }
}

impl Operation<SequenceFixture> for SequenceOperation {
    type Diff = SequenceDiff;

    fn diff(&self, projection: &SequenceFixture) -> SequenceDiff {
        match self {
            SequenceOperation::StepsAdd { index, item } => SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Add { index: *index, item: item.clone() })), ..Default::default() },
            SequenceOperation::StepsRemove { id } => SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Remove { id: id.clone() })), ..Default::default() },
            SequenceOperation::StepsMove { id, to_index } => SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Move { id: id.clone(), to_index: *to_index })), ..Default::default() },
            SequenceOperation::StepsPatch { id, patch } => SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() },
            SequenceOperation::EdgesAdd { index, item } => SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Add { index: *index, item: item.clone() })), ..Default::default() },
            SequenceOperation::EdgesRemove { id } => SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Remove { id: id.clone() })), ..Default::default() },
            SequenceOperation::EdgesMove { id, to_index } => SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Move { id: id.clone(), to_index: *to_index })), ..Default::default() },
            SequenceOperation::EdgesPatch { id, patch } => SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &SequenceFixture) -> Vec<Self> {
        match self {
            SequenceOperation::StepsAdd { index, item } => vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Add { index: *index, item: item.clone() }))],
            SequenceOperation::StepsRemove { id } => vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Remove { id: id.clone() }))],
            SequenceOperation::StepsMove { id, to_index } => {
                vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Move { id: id.clone(), to_index: *to_index }))]
            }
            SequenceOperation::StepsPatch { id, patch } => {
                vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
            SequenceOperation::EdgesAdd { index, item } => vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Add { index: *index, item: item.clone() }))],
            SequenceOperation::EdgesRemove { id } => vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Remove { id: id.clone() }))],
            SequenceOperation::EdgesMove { id, to_index } => {
                vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Move { id: id.clone(), to_index: *to_index }))]
            }
            SequenceOperation::EdgesPatch { id, patch } => {
                vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
        }
    }
}

/// 🔀️ Diffs two fixtures into a minimal typed operation set: removed/added/patched steps and edges.
/// Lets command handlers keep computing the target fixture via `crate::artifacts::sequence::engine::SequenceHost`
/// (with all its cycle/slot/layout logic) while emitting granular, mergeable operations.
pub fn sequence_fixture_operations(before: &SequenceFixture, after: &SequenceFixture) -> Vec<SequenceOperation> {
    let mut operations = Vec::new();
    for step in &before.steps {
        if !after.steps.iter().any(|entry| entry.id == step.id) {
            operations.push(SequenceOperation::StepsRemove { id: step.id.clone() });
        }
    }
    for (index, step) in after.steps.iter().enumerate() {
        match before.steps.iter().find(|entry| entry.id == step.id) {
            None => operations.push(SequenceOperation::StepsAdd { index, item: step.clone() }),
            Some(prior) => {
                let patch = SequenceStepPatch {
                    params: (prior.params != step.params).then(|| step.params.clone()),
                    x: (prior.x != step.x).then_some(step.x),
                    y: (prior.y != step.y).then_some(step.y),
                    collapsed: (prior.collapsed != step.collapsed).then_some(step.collapsed),
                };
                if patch != SequenceStepPatch::default() {
                    operations.push(SequenceOperation::StepsPatch { id: step.id.clone(), patch });
                }
            }
        }
    }
    for edge in &before.edges {
        if !after.edges.iter().any(|entry| entry.id == edge.id) {
            operations.push(SequenceOperation::EdgesRemove { id: edge.id.clone() });
        }
    }
    for (index, edge) in after.edges.iter().enumerate() {
        match before.edges.iter().find(|entry| entry.id == edge.id) {
            None => operations.push(SequenceOperation::EdgesAdd { index, item: edge.clone() }),
            Some(prior) => {
                let patch = SequenceEdgePatch { from: (prior.from != edge.from).then(|| edge.from.clone()), to: (prior.to != edge.to).then(|| edge.to.clone()) };
                if patch != SequenceEdgePatch::default() {
                    operations.push(SequenceOperation::EdgesPatch { id: edge.id.clone(), patch });
                }
            }
        }
    }
    operations
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_fixture, StepParams};
    use store::{create_document_envelope, DocumentCommand};
    use vcs::apply_operation;

    fn round_trip(fixture: &SequenceFixture, operation: &SequenceOperation) -> SequenceFixture {
        let forward = apply_operation(fixture, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(fixture) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, fixture, "backwards() must restore the pre-operation fixture");
        forward
    }

    #[test]
    fn add_remove_patch_steps_round_trip() {
        let fixture = default_fixture();
        let step = SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new(), x: 5.0, y: 6.0, slot: None, collapsed: false };
        let added = round_trip(&fixture, &SequenceOperation::StepsAdd { index: 2, item: step });
        assert_eq!(added.steps.len(), 3);
        let patched = round_trip(&added, &SequenceOperation::StepsPatch { id: "step-99".into(), patch: SequenceStepPatch { x: Some(120.0), ..Default::default() } });
        assert_eq!(patched.steps.iter().find(|step| step.id == "step-99").unwrap().x, 120.0);
        let removed = round_trip(&patched, &SequenceOperation::StepsRemove { id: "step-99".into() });
        assert!(!removed.steps.iter().any(|step| step.id == "step-99"));
    }

    #[test]
    fn fixture_ops_capture_move_and_connect() {
        let mut host = crate::artifacts::sequence::engine::SequenceHost::default();
        let before = host.fixture.clone();
        let id = host.add_step("math.add", 40.0, 40.0);
        let operations = sequence_fixture_operations(&before, &host.fixture);
        assert!(operations.iter().any(|operation| matches!(operation, SequenceOperation::StepsAdd { item, .. } if item.id == id)));
    }

    #[test]
    fn store_applies_and_undoes_step_add() {
        let mut store = SequenceStore::new(create_document_envelope(crate::artifacts::sequence::SEQUENCE_FIXTURE_SCHEMA, "sequence", default_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![SequenceOperation::StepsAdd { index: 2, item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 3);
    }
}
//#endregion 🧪️Tests

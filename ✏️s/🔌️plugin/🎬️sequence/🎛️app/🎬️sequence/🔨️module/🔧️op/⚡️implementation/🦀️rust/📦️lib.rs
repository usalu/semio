//! ⚡️ Sequence app — operation enum + laws (constitutional: op).

use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionDiff, CollectionOperation, Operation, OperationDiff};
use sequence::{sequence_edge_from_dsl, sequence_edge_to_dsl, SequenceCamera, SequenceEdge, SequenceEdgeDsl, SequenceEdgePatch, SequenceFixture, SequenceStep, SequenceStepPatch};
use serde::{Deserialize, Serialize};

//#region 🔖️Store
pub type SequenceEnvelope = store::DocumentEnvelope<SequenceFixture, SequenceOperation>;
pub type SequenceStore = store::DocumentStore<SequenceFixture, SequenceOperation>;
//#endregion 🔖️Store

//#region 🔖️Collections
fn apply_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: protocol::Identified<TId> + Clone + protocol::Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

fn absorb_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}
//#endregion 🔖️Collections

//#region 🔖️Operations
/// 🧮️ Typed sequence operation: id-keyed step/edge collection edits. The canvas camera is
/// session-only runtime state now (never a document field — see `SequencePlayRuntime::camera` in the
/// ui crate). Flattened into one keyword-tagged variant per {@link protocol::CollectionOperation} case rather
/// than wrapping that generic type directly — `CollectionOperation` is foreign (defined in
/// `protocol`) and generic, so it can never itself implement `dsl::DslField`/`dsl::DslVariants` from
/// this crate (the orphan rule requires a local type to anchor the impl on, and its OWN outer type
/// isn't local). {@link Operation for SequenceOperation} below reconstructs a `CollectionOperation`
/// ad hoc per match arm to keep reusing `protocol`'s generic collection diff/invert helpers.
/// 🧮️ Typed sequence operation — kept plain `Serialize`/`Deserialize` only; see `SequenceOperationDsl`
/// (`🔖️OpText` region) for the op-log DSL text mirror (`EdgesAdd`/`EdgesPatch` items as
/// `SequenceEdgeDsl`, a `dsl::Wire`-backed connection) and the hand-written `impl protocol::OpText for
/// SequenceOperation` that converts through it.
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
    EdgesPatch { id: String, patch: SequenceEdgePatch }
}

/// 🔁️ Converts a generic step `CollectionOperation` (as produced by `protocol::invert_collection_operation`)
/// back into its flat `SequenceOperation` variant.
fn steps_operation_from_collection(operation: CollectionOperation<String, SequenceStep, SequenceStepPatch>) -> SequenceOperation {
    match operation {
        CollectionOperation::Add { id: _id, item, at } => SequenceOperation::StepsAdd { index: at, item },
        CollectionOperation::Remove { id } => SequenceOperation::StepsRemove { id },
        CollectionOperation::Move { id, to } => SequenceOperation::StepsMove { id, to_index: to },
        CollectionOperation::Patch { id, patch } => SequenceOperation::StepsPatch { id, patch },
    }
}

/// 🔁️ Edge counterpart of {@link steps_operation_from_collection}.
fn edges_operation_from_collection(operation: CollectionOperation<String, SequenceEdge, SequenceEdgePatch>) -> SequenceOperation {
    match operation {
        CollectionOperation::Add { id: _id, item, at } => SequenceOperation::EdgesAdd { index: at, item },
        CollectionOperation::Remove { id } => SequenceOperation::EdgesRemove { id },
        CollectionOperation::Move { id, to } => SequenceOperation::EdgesMove { id, to_index: to },
        CollectionOperation::Patch { id, patch } => SequenceOperation::EdgesPatch { id, patch },
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceDiff {
    pub steps: Option<CollectionDiff<String, SequenceStepPatch, SequenceStep>>,
    pub edges: Option<CollectionDiff<String, SequenceEdgePatch, SequenceEdge>>,
}

impl OperationDiff<SequenceFixture> for SequenceDiff {
    fn apply(&self, projection: &SequenceFixture) -> SequenceFixture {
        let mut next = projection.clone();
        if let Some(diff) = &self.steps {
            apply_collection_diff(&mut next.steps, diff);
        }
        if let Some(diff) = &self.edges {
            apply_collection_diff(&mut next.edges, diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        absorb_collection_diff(&mut self.steps, other.steps);
        absorb_collection_diff(&mut self.edges, other.edges);
    }
}

impl Operation<SequenceFixture> for SequenceOperation {
    type Diff = SequenceDiff;

    fn diff(&self, projection: &SequenceFixture) -> SequenceDiff {
        match self {
            SequenceOperation::StepsAdd { index, item } => {
                SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index })), ..Default::default() }
            }
            SequenceOperation::StepsRemove { id } => SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Remove { id: id.clone() })), ..Default::default() },
            SequenceOperation::StepsMove { id, to_index } => {
                SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Move { id: id.clone(), to: *to_index })), ..Default::default() }
            }
            SequenceOperation::StepsPatch { id, patch } => {
                SequenceDiff { steps: Some(collection_diff_from_operation(&projection.steps, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() }
            }
            SequenceOperation::EdgesAdd { index, item } => {
                SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index })), ..Default::default() }
            }
            SequenceOperation::EdgesRemove { id } => SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Remove { id: id.clone() })), ..Default::default() },
            SequenceOperation::EdgesMove { id, to_index } => {
                SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Move { id: id.clone(), to: *to_index })), ..Default::default() }
            }
            SequenceOperation::EdgesPatch { id, patch } => {
                SequenceDiff { edges: Some(collection_diff_from_operation(&projection.edges, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() })), ..Default::default() }
            }
        }
    }

    fn backwards(&self, projection: &SequenceFixture) -> Vec<Self> {
        match self {
            SequenceOperation::StepsAdd { index, item } => vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index }))],
            SequenceOperation::StepsRemove { id } => vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Remove { id: id.clone() }))],
            SequenceOperation::StepsMove { id, to_index } => {
                vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Move { id: id.clone(), to: *to_index }))]
            }
            SequenceOperation::StepsPatch { id, patch } => {
                vec![steps_operation_from_collection(invert_collection_operation(&projection.steps, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
            SequenceOperation::EdgesAdd { index, item } => vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Add { id: item.id.clone(), item: item.clone(), at: *index }))],
            SequenceOperation::EdgesRemove { id } => vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Remove { id: id.clone() }))],
            SequenceOperation::EdgesMove { id, to_index } => {
                vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Move { id: id.clone(), to: *to_index }))]
            }
            SequenceOperation::EdgesPatch { id, patch } => {
                vec![edges_operation_from_collection(invert_collection_operation(&projection.edges, &CollectionOperation::Patch { id: id.clone(), patch: patch.clone() }))]
            }
        }
    }
}

/// 🔀️ Diffs two fixtures into a minimal typed operation set: removed/added/patched steps and edges.
/// Lets action handlers keep computing the target fixture via `sequence_engine::SequenceHost`
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

//#region 🔖️OpText
/// ✂️ DSL-only mirror of `SequenceOperation` — identical shape except `EdgesAdd.item` goes through
/// `SequenceEdgeDsl` for the unified wire syntax (see `sequence`'s `🔖️Dsl` doc comment on
/// `SequenceEdgeDsl` for why `EdgesPatch.patch` stays a plain `SequenceEdgePatch`, not a wire).
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum SequenceOperationDsl {
    StepsAdd {
        index: usize,
        #[dsl(block)]
        item: SequenceStep,
    },
    StepsRemove { id: String },
    StepsMove { id: String, to_index: usize },
    StepsPatch {
        id: String,
        #[dsl(block)]
        patch: SequenceStepPatch,
    },
    EdgesAdd {
        index: usize,
        #[dsl(block)]
        item: SequenceEdgeDsl,
    },
    EdgesRemove { id: String },
    EdgesMove { id: String, to_index: usize },
    EdgesPatch {
        id: String,
        #[dsl(block)]
        patch: SequenceEdgePatch,
    }
}

fn sequence_operation_to_dsl(operation: &SequenceOperation) -> SequenceOperationDsl {
    match operation {
        SequenceOperation::StepsAdd { index, item } => SequenceOperationDsl::StepsAdd { index: *index, item: item.clone() },
        SequenceOperation::StepsRemove { id } => SequenceOperationDsl::StepsRemove { id: id.clone() },
        SequenceOperation::StepsMove { id, to_index } => SequenceOperationDsl::StepsMove { id: id.clone(), to_index: *to_index },
        SequenceOperation::StepsPatch { id, patch } => SequenceOperationDsl::StepsPatch { id: id.clone(), patch: patch.clone() },
        SequenceOperation::EdgesAdd { index, item } => SequenceOperationDsl::EdgesAdd { index: *index, item: sequence_edge_to_dsl(item) },
        SequenceOperation::EdgesRemove { id } => SequenceOperationDsl::EdgesRemove { id: id.clone() },
        SequenceOperation::EdgesMove { id, to_index } => SequenceOperationDsl::EdgesMove { id: id.clone(), to_index: *to_index },
        SequenceOperation::EdgesPatch { id, patch } => SequenceOperationDsl::EdgesPatch { id: id.clone(), patch: patch.clone() },
    }
}

fn sequence_operation_from_dsl(operation: SequenceOperationDsl) -> Result<SequenceOperation, String> {
    Ok(match operation {
        SequenceOperationDsl::StepsAdd { index, item } => SequenceOperation::StepsAdd { index, item },
        SequenceOperationDsl::StepsRemove { id } => SequenceOperation::StepsRemove { id },
        SequenceOperationDsl::StepsMove { id, to_index } => SequenceOperation::StepsMove { id, to_index },
        SequenceOperationDsl::StepsPatch { id, patch } => SequenceOperation::StepsPatch { id, patch },
        SequenceOperationDsl::EdgesAdd { index, item } => SequenceOperation::EdgesAdd { index, item: sequence_edge_from_dsl(item)? },
        SequenceOperationDsl::EdgesRemove { id } => SequenceOperation::EdgesRemove { id },
        SequenceOperationDsl::EdgesMove { id, to_index } => SequenceOperation::EdgesMove { id, to_index },
        SequenceOperationDsl::EdgesPatch { id, patch } => SequenceOperation::EdgesPatch { id, patch },
    })
}

impl protocol::OpText for SequenceOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let dsl_operation = <SequenceOperationDsl as protocol::OpText>::parse_op(line)?;
        sequence_operation_from_dsl(dsl_operation).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <SequenceOperationDsl as protocol::OpText>::print_op(&sequence_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `SequenceOperationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for SequenceOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        sequence_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let dsl_operation = SequenceOperationDsl::decode_op(bytes)?;
        sequence_operation_from_dsl(dsl_operation).map_err(|message| protocol::ProtocolError::Malformed { what: "sequence operation", offset: 0, detail: message })
    }
}
//#endregion 🔖️OpText

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `sequence_engine::SequenceConfig`'s operation enum — one variant per settled
/// interaction (mirrors the pre-B1 `SequencePlayRuntime` field writes), plus a generic `Snapshot`
/// every variant's `backwards()` returns — same "whole-config snapshot is the simplest correct
/// inverse" shape as `shooting_op::ShootingConfigOperation`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SequenceConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: sequence_engine::SequenceConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { step_ids: Vec<String> },
    #[dsl(key = "last-run")]
    SetLastRun { json: String },
    #[dsl(key = "orientation")]
    SetOrientation { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: SequenceCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<sequence_engine::SequenceConfig> for SequenceConfigOperation {
    type Diff = sequence_engine::SequenceConfig;

    fn diff(&self, base: &sequence_engine::SequenceConfig) -> sequence_engine::SequenceConfig {
        let mut next = base.clone();
        match self {
            SequenceConfigOperation::Snapshot { config } => return config.clone(),
            SequenceConfigOperation::SetSelection { step_ids } => next.selected_step_ids = step_ids.clone(),
            SequenceConfigOperation::SetLastRun { json } => next.last_run_json = json.clone(),
            SequenceConfigOperation::SetOrientation { value } => next.orientation = value.clone(),
            SequenceConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            SequenceConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &sequence_engine::SequenceConfig) -> Vec<Self> {
        vec![SequenceConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Dictionary, Value};
    use sequence::default_fixture;
    use sequence::{SlotRef, StepParams};
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
        let mut host = sequence_engine::SequenceHost::default();
        let before = host.fixture.clone();
        let id = host.add_step("math.add", 40.0, 40.0);
        let operations = sequence_fixture_operations(&before, &host.fixture);
        assert!(operations.iter().any(|operation| matches!(operation, SequenceOperation::StepsAdd { item, .. } if item.id == id)));
    }

    #[test]
    fn store_applies_and_undoes_step_add() {
        let mut store = SequenceStore::new(create_document_envelope(sequence::SEQUENCE_FIXTURE_SCHEMA, "sequence", default_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![SequenceOperation::StepsAdd { index: 2, item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 3);
    }

    #[test]
    fn op_text_round_trips_steps_add() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsAdd {
            index: 2,
            item: SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new().insert("message", Value::Atom(Atom::String("hi there".into()))), x: 5.0, y: -6.5, slot: None, collapsed: false },
        });
    }

    #[test]
    fn op_text_round_trips_steps_add_with_slot() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsAdd {
            index: 0,
            item: SequenceStep { id: "step-98".into(), kind: "control.while".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "body".into() }), collapsed: true },
        });
    }

    #[test]
    fn op_text_round_trips_steps_remove() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsRemove { id: "step-99".into() });
    }

    #[test]
    fn op_text_round_trips_steps_move() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsMove { id: "step-99".into(), to_index: 3 });
    }

    #[test]
    fn op_text_round_trips_steps_patch() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsPatch {
            id: "step-99".into(),
            patch: SequenceStepPatch {
                params: Some(StepParams::new().insert("value", Value::Atom(Atom::Decimal(120.0))).insert("meta", Value::Dictionary(Dictionary::new().insert("k", Value::Atom(Atom::Null))))),
                x: Some(120.0),
                y: None,
                collapsed: Some(true),
            },
        });
    }

    #[test]
    fn op_text_round_trips_steps_patch_with_no_fields() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::StepsPatch { id: "step-99".into(), patch: SequenceStepPatch::default() });
    }

    #[test]
    fn op_text_round_trips_edges_add() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesAdd { index: 1, item: SequenceEdge { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() } });
    }

    #[test]
    fn op_text_round_trips_edges_remove() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesRemove { id: "edge-1".into() });
    }

    #[test]
    fn op_text_round_trips_edges_move() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesMove { id: "edge-1".into(), to_index: 0 });
    }

    #[test]
    fn op_text_round_trips_edges_patch() {
        store::test_support::assert_op_line_round_trip(&SequenceOperation::EdgesPatch { id: "edge-1".into(), patch: SequenceEdgePatch { from: Some("step-3".into()), to: None } });
    }

    //#region 🔖️ConfigOperationTests
    fn round_trip_config(config: &sequence_engine::SequenceConfig, operation: &SequenceConfigOperation) -> sequence_engine::SequenceConfig {
        let forward = operation.diff(config);
        let backwards = operation.backwards(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward);
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_selection_round_trips() {
        let config = sequence_engine::SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetSelection { step_ids: vec!["step-1".into()] });
        assert_eq!(next.selected_step_ids, vec!["step-1".to_string()]);
    }

    #[test]
    fn config_set_last_run_round_trips() {
        let config = sequence_engine::SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetLastRun { json: "{\"ok\":true}".into() });
        assert_eq!(next.last_run_json, "{\"ok\":true}");
    }

    #[test]
    fn config_set_orientation_round_trips() {
        let config = sequence_engine::SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetOrientation { value: "topBottom".into() });
        assert_eq!(next.orientation, "topBottom");
    }

    #[test]
    fn config_set_camera_round_trips() {
        let config = sequence_engine::SequenceConfig::default();
        let camera = SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let next = round_trip_config(&config, &SequenceConfigOperation::SetCamera { camera: camera.clone() });
        assert_eq!(next.camera, camera);
    }

    #[test]
    fn config_set_locale_round_trips() {
        let config = sequence_engine::SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigOperation::SetLocale { value: "de-DE".into() });
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::Snapshot { config: sequence_engine::SequenceConfig::default() });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetSelection { step_ids: vec!["step-1".into(), "step-2".into()] });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetLastRun { json: "{}".into() });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetOrientation { value: "leftRight".into() });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetCamera { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&SequenceConfigOperation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🔖️ConfigOperationTests
}
//#endregion 🧪️Tests

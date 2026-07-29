//! ✂️ Imperative app — operation enum + laws (constitutional: op).

use imperative::{step_node_dsl_to_step, step_to_step_node_dsl, value_dsl_map_to_dictionary, dictionary_to_value_dsl_map, Dictionary, ImperativeDocument, PathRef, Step, StepNodeDsl};
use serde::{Deserialize, Serialize};

//#region 🔖Operation
/// @emoji ✂️ A step-collection edit at a `PathRef` — root path or a nested `control.*` step's slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeOperation {
    pub path_ref: PathRef,
    pub collection: protocol::CollectionOperation<String, Step, Dictionary>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ImperativeDiff(pub Option<ImperativeOperation>);

impl protocol::OperationDiff<ImperativeDocument> for ImperativeDiff {
    fn apply(&self, projection: &ImperativeDocument) -> ImperativeDocument {
        let mut next = projection.clone();
        if let Some(operation) = &self.0 {
            if let Some(steps) = resolve_steps_mut(&mut next, &operation.path_ref) {
                protocol::apply_collection_operation(steps, &operation.collection);
            }
            prune_empty_slot(&mut next, &operation.path_ref);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.0.is_some() {
            self.0 = other.0;
        }
    }
}

impl protocol::Operation<ImperativeDocument> for ImperativeOperation {
    type Diff = ImperativeDiff;

    fn diff(&self, _projection: &ImperativeDocument) -> Self::Diff {
        ImperativeDiff(Some(self.clone()))
    }

    fn backwards(&self, projection: &ImperativeDocument) -> Vec<Self> {
        match resolve_steps(projection, &self.path_ref) {
            Some(steps) => vec![ImperativeOperation { path_ref: self.path_ref.clone(), collection: protocol::invert_collection_operation(steps, &self.collection) }],
            None => Vec::new(),
        }
    }
}

/// 🔎 Resolves the step list a `PathRef` addresses; a not-yet-materialized nested slot reads as empty.
fn resolve_steps<'a>(document: &'a ImperativeDocument, path_ref: &PathRef) -> Option<&'a [Step]> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&document.path.steps);
    }
    let owner = path_ref.owner.as_ref()?;
    let slot = path_ref.slot.as_ref()?;
    let owner_step = document.path.steps.iter().find(|step| &step.id == owner)?;
    Some(owner_step.bodies.get(slot).map_or(&[] as &[Step], |path| path.steps.as_slice()))
}

fn resolve_steps_mut<'a>(document: &'a mut ImperativeDocument, path_ref: &PathRef) -> Option<&'a mut Vec<Step>> {
    if path_ref.owner.is_none() && path_ref.slot.is_none() {
        return Some(&mut document.path.steps);
    }
    let owner = path_ref.owner.clone()?;
    let slot = path_ref.slot.clone()?;
    let owner_step = document.path.steps.iter_mut().find(|step| step.id == owner)?;
    Some(&mut owner_step.bodies.entry(slot).or_insert_with(imperative::Path::new).steps)
}

/// 🧹 Drops a nested slot's `bodies` entry once it's empty, so an emptied slot is bit-identical to
/// a never-touched one — required for `Add` then `Remove` to be a true, exact inverse pair.
fn prune_empty_slot(document: &mut ImperativeDocument, path_ref: &PathRef) {
    let (Some(owner), Some(slot)) = (&path_ref.owner, &path_ref.slot) else {
        return;
    };
    if let Some(owner_step) = document.path.steps.iter_mut().find(|step| &step.id == owner) {
        if owner_step.bodies.get(slot).is_some_and(|path| path.steps.is_empty()) {
            owner_step.bodies.remove(slot);
        }
    }
}
//#endregion 🔖Operation

//#region 🔖OpText
/// ✂️ Local mirror of `ImperativeOperation` — flattens `PathRef` into bare `owner`/`slot`
/// `Option<String>` fields (printed bare when the value lexes as a bare ident, per the engine's
/// default `Shape::Text` behavior — no per-field opt-in needed) since a `store::Operation` grammar is
/// a genuinely tagged enum (`#[derive(dsl::DslOps)]` requires an enum), not the single generic-struct
/// shape `ImperativeOperation`/`vcs::CollectionOperation` use at the Rust level.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum ImperativeOperationDsl {
    Add {
        owner: Option<String>,
        slot: Option<String>,
        index: usize,
        #[dsl(statements)]
        item: Box<StepNodeDsl>,
    },
    Remove { owner: Option<String>, slot: Option<String>, id: String },
    Move {
        owner: Option<String>,
        slot: Option<String>,
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    Patch { owner: Option<String>, slot: Option<String>, id: String, patch: std::collections::BTreeMap<String, imperative::ValueDsl> },
}

fn imperative_operation_to_dsl(operation: &ImperativeOperation) -> ImperativeOperationDsl {
    let owner = operation.path_ref.owner.clone();
    let slot = operation.path_ref.slot.clone();
    match &operation.collection {
        // 🔒 `id` is intentionally dropped in the DSL's `Add` shape (unchanged on-disk text
        // format) — `Step.id` round-trips it losslessly, recovered on the reverse conversion below.
        protocol::CollectionOperation::Add { id: _id, item, at } => ImperativeOperationDsl::Add { owner, slot, index: *at, item: Box::new(step_to_step_node_dsl(item)) },
        protocol::CollectionOperation::Remove { id } => ImperativeOperationDsl::Remove { owner, slot, id: id.clone() },
        protocol::CollectionOperation::Move { id, to } => ImperativeOperationDsl::Move { owner, slot, id: id.clone(), to_index: *to },
        protocol::CollectionOperation::Patch { id, patch } => ImperativeOperationDsl::Patch { owner, slot, id: id.clone(), patch: dictionary_to_value_dsl_map(patch) },
    }
}

fn imperative_operation_from_dsl(dsl_op: ImperativeOperationDsl) -> ImperativeOperation {
    match dsl_op {
        ImperativeOperationDsl::Add { owner, slot, index, item } => {
            let item = step_node_dsl_to_step(*item);
            let id = item.id.clone();
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Add { id, item, at: index } }
        }
        ImperativeOperationDsl::Remove { owner, slot, id } => ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Remove { id } },
        ImperativeOperationDsl::Move { owner, slot, id, to_index } => {
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Move { id, to: to_index } }
        }
        ImperativeOperationDsl::Patch { owner, slot, id, patch } => {
            ImperativeOperation { path_ref: PathRef { owner, slot }, collection: protocol::CollectionOperation::Patch { id, patch: value_dsl_map_to_dictionary(&patch) } }
        }
    }
}

impl protocol::OpText for ImperativeOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(imperative_operation_from_dsl(<ImperativeOperationDsl as protocol::OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <ImperativeOperationDsl as protocol::OpText>::print_op(&imperative_operation_to_dsl(self))
    }
}

/// ⚡ Binary mirror of the `OpText` impl above — `ImperativeOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl protocol::OpBinary for ImperativeOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        imperative_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(imperative_operation_from_dsl(ImperativeOperationDsl::decode_op(bytes)?))
    }
}
//#endregion 🔖OpText

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn step(id: &str, kind: &str) -> Step {
        Step { id: id.into(), kind: kind.into(), params: Dictionary::new(), bodies: BTreeMap::new() }
    }

    #[test]
    fn add_step_op_round_trips() {
        let document = imperative_engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Add { id: "step-x".to_string(), item: step("step-x", "log.print"), at: 0 } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn remove_step_op_round_trips() {
        let document = imperative_engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-1".into() } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn move_step_op_round_trips() {
        let document = imperative_engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Move { id: "step-1".into(), to: 1 } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn patch_step_params_op_round_trips() {
        use neural_engine::{Atom, Value};
        let document = imperative_engine::default_document();
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Patch { id: "step-1".into(), patch: Dictionary::new().insert("key", Value::Atom(Atom::String("renamed".into()))) } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        store::test_support::assert_store_roundtrip(document, operation);
    }

    #[test]
    fn add_step_into_nested_control_body_round_trips() {
        let mut document = imperative_engine::default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        let operation = ImperativeOperation { path_ref, collection: protocol::CollectionOperation::Add { id: "step-nested".to_string(), item: step("step-nested", "log.print"), at: 0 } };
        store::test_support::assert_operation_round_trip(&document, operation.clone());
        store::test_support::assert_op_line_round_trip(&operation);
        let post = vcs::apply_operation(&document, &operation);
        let owner_step = post.path.steps.iter().find(|entry| entry.id == "step-if").expect("owner step");
        assert_eq!(owner_step.bodies.get("then").map(|body| body.steps.len()), Some(1));
        store::test_support::assert_store_roundtrip(document, operation);
    }

    //#region resolve_steps / resolve_steps_mut / prune_empty_slot
    #[test]
    fn resolve_steps_root_returns_document_steps() {
        let document = imperative_engine::default_document();
        let steps = resolve_steps(&document, &PathRef::default()).expect("root always resolves");
        assert_eq!(steps.len(), document.path.steps.len());
    }

    #[test]
    fn resolve_steps_unknown_owner_is_none() {
        let document = imperative_engine::default_document();
        let path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(resolve_steps(&document, &path_ref).is_none());
    }

    #[test]
    fn resolve_steps_missing_owner_or_slot_is_none() {
        let document = imperative_engine::default_document();
        assert!(resolve_steps(&document, &PathRef { owner: Some("step-1".into()), slot: None }).is_none());
        assert!(resolve_steps(&document, &PathRef { owner: None, slot: Some("then".into()) }).is_none());
    }

    #[test]
    fn resolve_steps_unmaterialized_slot_reads_empty() {
        let mut document = imperative_engine::default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        assert_eq!(resolve_steps(&document, &path_ref), Some(&[][..]));
    }

    #[test]
    fn resolve_steps_mut_unknown_owner_is_none() {
        let mut document = imperative_engine::default_document();
        let path_ref = PathRef { owner: Some("missing".into()), slot: Some("then".into()) };
        assert!(resolve_steps_mut(&mut document, &path_ref).is_none());
    }

    #[test]
    fn prune_empty_slot_removes_emptied_bodies_entry() {
        let mut document = imperative_engine::default_document();
        document.path.steps.push(step("step-if", "control.if"));
        let path_ref = PathRef { owner: Some("step-if".into()), slot: Some("then".into()) };
        resolve_steps_mut(&mut document, &path_ref).expect("materializes slot").push(step("step-nested", "log.print"));
        let owner_step = document.path.steps.iter().find(|s| s.id == "step-if").expect("owner");
        assert!(owner_step.bodies.contains_key("then"));
        resolve_steps_mut(&mut document, &path_ref).expect("slot exists").clear();
        prune_empty_slot(&mut document, &path_ref);
        let owner_step = document.path.steps.iter().find(|s| s.id == "step-if").expect("owner");
        assert!(!owner_step.bodies.contains_key("then"));
    }

    #[test]
    fn prune_empty_slot_noop_without_owner_or_slot() {
        let mut document = imperative_engine::default_document();
        prune_empty_slot(&mut document, &PathRef::default());
    }

    #[test]
    fn operation_backwards_on_unresolvable_path_ref_is_empty() {
        let document = imperative_engine::default_document();
        let operation = ImperativeOperation {
            path_ref: PathRef { owner: Some("missing".into()), slot: Some("then".into()) },
            collection: protocol::CollectionOperation::Remove { id: "step-x".into() },
        };
        assert!(protocol::Operation::backwards(&operation, &document).is_empty());
    }

    #[test]
    fn imperative_diff_absorb_keeps_latest_some_and_ignores_none() {
        use protocol::OperationDiff;
        let first = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-1".into() } };
        let second = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-2".into() } };
        let mut diff = ImperativeDiff(Some(first));
        diff.absorb(ImperativeDiff(None));
        assert!(matches!(&diff.0, Some(op) if matches!(&op.collection, protocol::CollectionOperation::Remove { id } if id == "step-1")));
        diff.absorb(ImperativeDiff(Some(second)));
        assert!(matches!(&diff.0, Some(op) if matches!(&op.collection, protocol::CollectionOperation::Remove { id } if id == "step-2")));
    }
    //#endregion resolve_steps / resolve_steps_mut / prune_empty_slot

    //#region op text
    #[test]
    fn op_text_rejects_unknown_operation_keyword() {
        let line = r#"frobnicate owner=- slot=- id="step-1""#;
        assert!(<ImperativeOperation as protocol::OpText>::parse_op(line).is_err());
    }

    #[test]
    fn op_text_round_trips_add_with_owner_and_slot() {
        let operation = ImperativeOperation {
            path_ref: PathRef { owner: Some("step-if".into()), slot: Some("then".into()) },
            collection: protocol::CollectionOperation::Add { id: "step-nested".to_string(), item: step("step-nested", "log.print"), at: 0 },
        };
        let printed = <ImperativeOperation as protocol::OpText>::print_op(&operation);
        assert!(printed.contains("owner=step-if"), "printed: {printed}");
        assert!(printed.contains("slot=then"), "printed: {printed}");
        let parsed = <ImperativeOperation as protocol::OpText>::parse_op(&printed).expect("round trips");
        assert_eq!(parsed, operation);
    }
    //#endregion op text
}
//#endregion 🧪Tests

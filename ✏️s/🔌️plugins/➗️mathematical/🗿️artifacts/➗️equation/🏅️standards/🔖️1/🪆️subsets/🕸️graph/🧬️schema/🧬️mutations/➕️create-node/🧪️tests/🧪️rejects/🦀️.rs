//! 🧪️ `create-node` fixture — `🚫️rejects-a-duplicate-node-id`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE, and the
//! `notation`/`results`/`computed` handles are the content addresses of their derivation (`crate::equation_children`), kept
//! exact by the fixture writer law.
//!
//! 🌱 `create-node` refuses only a colliding id, so — following dag's `🚫️rejects-a-duplicate-node-id` precedent — the committed
//! graph already holds exactly the node the committed payload asks to create: its `id`/`label`/`x`/`y` ARE the mutation JSON's.

use crate::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode;
use crate::{EquationDiff, EquationGraph, EquationMutation, EquationNode, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/🎯️outcome/🔣️.json");

fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// 🟢️ The committed payload, unwrapped — the colliding node is built from it and nothing else.
fn payload() -> CreateNode {
    let EquationMutation::CreateNode(payload) = mutation() else {
        panic!("rejects-a-duplicate-node-id's committed mutation must be a create-node");
    };
    payload
}

/// 🌱 The graph both committed snapshots hold: it already contains the very node the payload tries to create — the
/// collision `mutation.duplicate-id` guards against.
fn colliding_graph() -> EquationGraph {
    let payload = payload();
    EquationGraph { directed: true, nodes: vec![EquationNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y }], edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }
}

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `create-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert_eq!(base.graph, colliding_graph(), "rejects-a-duplicate-node-id's before-snapshot holds exactly the payload's node");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "create-node/rejects-a-duplicate-node-id: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected create must not mint a fresh notation/results/computed triple");
}

/// 🚨️ A colliding node id is a FATAL `mutation.duplicate-id`, not the Error-level `target-missing`
/// the rest of this vocabulary uses — a duplicate identity is an invariant breach, not a miss. The
/// diagnostic names the NODE id the payload carried, never its label or coordinates.
#[semio_framework_async_macros::async_test]
async fn a_colliding_id_is_a_fatal_duplicate_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting create-node must carry an empty diff, never a half-built child triple");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec![payload().id.clone()], "the diagnostic addresses the colliding node id");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "node", "create-node", "CreatedNode"), "the fixture must be bound to create-node's own descriptor");
}

/// ↩️ Equation's `create-node` inverse is BASE-GUARDED, not unconditionally payload-derived:
/// because `base` already carries this id, the create is treated as having changed nothing and the
/// undo is empty — a deliberate divergence from dag's create-node, whose inverse is always a
/// delete of the requested id.
#[semio_framework_async_macros::async_test]
async fn an_already_present_id_leaves_nothing_to_undo() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "create-node/rejects-a-duplicate-node-id: a create whose id BASE already holds must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed
/// point. `x`/`y` are `f64`, so they always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "create-node/rejects-a-duplicate-node-id: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "create-node/rejects-a-duplicate-node-id: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "create-node/rejects-a-duplicate-node-id declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(semio_framework_pack_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(semio_framework_pack_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

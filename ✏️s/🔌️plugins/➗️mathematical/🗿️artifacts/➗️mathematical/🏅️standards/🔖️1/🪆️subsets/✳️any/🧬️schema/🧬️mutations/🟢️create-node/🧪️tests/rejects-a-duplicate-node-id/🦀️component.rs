//! 🧪️ `create-node` fixture — `rejects-a-duplicate-node-id`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `MathematicalSnapshot` keeps its graph and its point
//! cloud in three co-derived composed CHILDREN (`notation`/`results`/`computed`,
//! `🔖️WorkingScene`), and every APPLIED mathematical diff re-mints all three through
//! `mathematical_children_from_state`, whose `child_id` is a `DefaultHasher` digest of the child
//! content. Hand-authoring such an `➡️after` would mean hand-forging a value from `std`'s
//! deliberately unspecified default hasher; the rejection branch reaches no digest at all.
//!
//! 🌱 `create-node` is the one mathematical graph verb with NO rejection on an empty scene, so —
//! following dag's `rejects-a-duplicate-node-id` precedent — this case resolves the committed
//! handles against a scene holding exactly the node the committed payload asks to create. Nothing
//! here is invented: the seeded node IS the mutation JSON's own `id`/`label`/`x`/`y`. This plugin
//! exposes no seed-by-id helper (dag's `cache_dag_content` has no twin here), so the seeding goes
//! through `mathematical_children_from_state`, which mints AND caches in one call — the committed
//! `childId`s are therefore documented placeholders for that digest.

use crate::artifacts::mathematical::mutations::create_node::mutation::CreateNode;
use crate::artifacts::mathematical::{mathematical_children_from_state, mathematical_graph, MathematicalDiff, MathematicalGeometry, MathematicalGraph, MathematicalMutation, MathematicalNode, MathematicalSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> MathematicalMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// 🟢️ The committed payload, unwrapped — the colliding node is built from it and nothing else.
fn payload() -> CreateNode {
    let MathematicalMutation::CreateNode(payload) = mutation() else {
        panic!("rejects-a-duplicate-node-id's committed mutation must be a create-node");
    };
    payload
}

/// 🌱 The scene both committed snapshots resolve to: a graph already holding the very node the
/// payload tries to create — the collision `mutation.duplicate-id` guards against.
fn colliding_graph() -> MathematicalGraph {
    let payload = payload();
    MathematicalGraph { directed: true, nodes: vec![MathematicalNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y }], edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }
}

/// 🧩️ Swaps the committed placeholder handles for the digests this plugin mints for the colliding
/// scene, caching it in the same call so the snapshot resolves instead of failing soft.
fn resolved(text: &str) -> MathematicalSnapshot {
    let mut snapshot: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
    let (notation, results, computed) = mathematical_children_from_state(&colliding_graph(), &MathematicalGeometry { points: Vec::new() });
    snapshot.notation = notation;
    snapshot.results = results;
    snapshot.computed = computed;
    snapshot
}

fn before() -> MathematicalSnapshot {
    resolved(BEFORE)
}
fn expected_after() -> MathematicalSnapshot {
    resolved(AFTER)
}
fn produced() -> protocol::MutationOutcome<MathematicalDiff> {
    <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `create-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(mathematical_graph(&base).nodes.iter().any(|node| node.id == payload().id), "rejects-a-duplicate-node-id's before-snapshot must resolve to a scene that already holds the payload's node id");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "create-node/rejects-a-duplicate-node-id: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected create must not mint a fresh notation/results/computed triple");
}

/// 🚨️ A colliding node id is a FATAL `mutation.duplicate-id`, not the Error-level `target-missing`
/// the rest of this vocabulary uses — a duplicate identity is an invariant breach, not a miss. The
/// diagnostic names the NODE id the payload carried, never its label or coordinates.
#[semio_framework_async_macros::async_test]
async fn a_colliding_id_is_a_fatal_duplicate_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &MathematicalDiff::default(), "a rejecting create-node must carry an empty diff, never a half-built child triple");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec![payload().id.clone()], "the diagnostic addresses the colliding node id");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "node", "create-node", "CreatedNode"), "the fixture must be bound to create-node's own descriptor");
}

/// ↩️ Mathematical's `create-node` inverse is BASE-GUARDED, not unconditionally payload-derived:
/// because `base` already carries this id, the create is treated as having changed nothing and the
/// undo is empty — a deliberate divergence from dag's create-node, whose inverse is always a
/// delete of the requested id.
#[semio_framework_async_macros::async_test]
async fn an_already_present_id_leaves_nothing_to_undo() {
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "create-node/rejects-a-duplicate-node-id: a create whose id BASE already holds must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed
/// point. `x`/`y` are `f64`, so they always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/rejects-a-duplicate-node-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-node/rejects-a-duplicate-node-id: committed mutation JSON is not canonical");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-node/rejects-a-duplicate-node-id declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

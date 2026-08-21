//! 🧪️ `connect-nodes` fixture — `rejects-an-edge-between-two-absent-endpoints`.
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
//! deliberately unspecified default hasher. A committed snapshot therefore decodes to an
//! UNRESOLVED handle and `mathematical_scene` fails soft to an empty graph — the state this case
//! pins, and the state in which `connect-nodes`' endpoint `mutation.target-missing` fires.

use crate::artifacts::mathematical::mutations::disconnect_nodes::mutation::DisconnectNodes;
use crate::artifacts::mathematical::{mathematical_graph, MathematicalDiff, MathematicalMutation, MathematicalSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> MathematicalSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> MathematicalSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> MathematicalMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<MathematicalDiff> {
    <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `connect-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(mathematical_graph(&base).nodes.is_empty() && mathematical_graph(&base).edges.is_empty(), "rejects-an-edge-between-two-absent-endpoints' before-snapshot must decode to an unresolved, empty graph");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "connect-nodes/rejects-an-edge-between-two-absent-endpoints: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected connect must not mint a fresh notation/results/computed triple");
}

/// 🔗️ `connect-nodes` is the only verb in this vocabulary whose diagnostic names something OTHER
/// than the entity it was asked to create: the edge id `e-alpha-beta` never appears — the target is
/// the list of missing ENDPOINT node ids, both of them, in source-then-target order. (Its own id is
/// only ever reported by the earlier Fatal `mutation.duplicate-id` branch, which an edge-less graph
/// cannot reach.)
#[semio_framework_async_macros::async_test]
async fn the_missing_endpoints_are_named_not_the_new_edge_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &MathematicalDiff::default(), "a rejecting connect-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "absent endpoints are reported as target-missing, not duplicate-id");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing endpoint is an Error — the Fatal is reserved for a colliding edge id");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string(), "n-beta".to_string()], "the diagnostic names the endpoint nodes, in source-then-target order");
    assert!(!messages[0].target.contains(&"e-alpha-beta".to_string()), "the edge id must not appear in a target-missing diagnostic");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("connect", "node", "connect-nodes", "ConnectedNodes"), "the fixture must be bound to connect-nodes' own descriptor — a PLURAL kind over a SINGULAR entity");
}

/// ↩️ `connect-nodes`' inverse is PAYLOAD-derived, unlike every other refusing verb in this
/// vocabulary: because BASE holds no edge under that id, the undo is a `disconnect-nodes` of the id
/// it was asked to create, even though the connect itself was refused.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_disconnect_of_the_requested_edge_id() {
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &before());
    assert_eq!(inverse, vec![MathematicalMutation::DisconnectNodes(DisconnectNodes { id: "e-alpha-beta".to_string() })], "connect-nodes undoes with exactly one disconnect of the requested edge id, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload carries three
/// flat id strings — `id`, `source`, `target` — never a nested endpoint object.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-nodes/rejects-an-edge-between-two-absent-endpoints: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-nodes/rejects-an-edge-between-two-absent-endpoints: committed mutation JSON is not canonical");
    assert_eq!(original.pointer("/ConnectNodes/source").and_then(serde_json::Value::as_str), Some("n-alpha"), "the payload addresses its endpoints by bare node id");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "connect-nodes/rejects-an-edge-between-two-absent-endpoints declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome
        .get("path")
        .and_then(serde_json::Value::as_array)
        .expect("a rejected outcome declares a path")
        .iter()
        .map(|entry| entry.as_str().expect("path segments are strings").to_string())
        .collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

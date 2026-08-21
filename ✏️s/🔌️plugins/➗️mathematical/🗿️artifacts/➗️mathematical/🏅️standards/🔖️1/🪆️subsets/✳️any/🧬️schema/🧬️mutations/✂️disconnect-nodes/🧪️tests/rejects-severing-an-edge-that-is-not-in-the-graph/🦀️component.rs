//! 🧪️ `disconnect-nodes` fixture — `rejects-severing-an-edge-that-is-not-in-the-graph`.
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
//! UNRESOLVED handle and `mathematical_scene` fails soft to an edge-less graph — the state this
//! case pins, and the state in which `disconnect-nodes`' own `mutation.target-missing` fires.

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

/// ▶️ A rejected `disconnect-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(mathematical_graph(&base).edges.is_empty(), "rejects-severing-an-edge-that-is-not-in-the-graph's before-snapshot must decode to an unresolved, edge-less graph");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected disconnect must not mint a fresh notation/results/computed triple");
}

/// ✂️ `disconnect-nodes` is this vocabulary's only verb addressed by EDGE id: it searches
/// `graph.edges`, never `graph.nodes`, so the reported target is the edge id verbatim and neither
/// endpoint is ever named. It also has no duplicate/invariant branch — one Error and nothing else.
#[semio_framework_async_macros::async_test]
async fn a_missing_edge_is_reported_by_its_edge_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &MathematicalDiff::default(), "a rejecting disconnect-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing edge is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "disconnect-nodes has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["e-alpha-beta".to_string()], "the diagnostic names the EDGE id, not either endpoint node");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("disconnect", "node", "disconnect-nodes", "DisconnectedNodes"), "the fixture must be bound to disconnect-nodes' own descriptor");
}

/// ↩️ `disconnect-nodes` inverts by reconstructing the exact edge BASE showed — id plus both
/// endpoints. With no such edge captured the inverse is empty, the mirror image of its
/// `connect-nodes` twin, whose inverse is payload-derived and therefore never empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_edge_to_reconnect() {
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: a rejected disconnect must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload is a single
/// `id` — the edge's — with no source/target pair, which is what makes this verb edge-addressed.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed mutation JSON is not canonical");
    assert_eq!(original.pointer("/DisconnectNodes").and_then(serde_json::Value::as_object).map(|fields| fields.len()), Some(1), "the payload carries the edge id and nothing else");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

//! 🧪️ `connect-nodes` fixture — `rejects-an-edge-whose-source-node-is-absent`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION: a successful `connect-nodes` runs `fixtures_after_add_edge`
//! into `diff_wires_and_board`, whose board half calls `diff_board_fixture` and re-mints the
//! composed `s.stdio.semio.graph` child handle as `format!("wires-content-{hash:016x}")` over a
//! `DefaultHasher` digest of the child content — a value from `std`'s deliberately unspecified
//! default hasher, which may not be hand-forged into an `➡️after`. This verb has no no-op guard
//! (connecting is never idempotent), so a rejection branch is what this case pins.
//!
//! 🔗 Of `connect-nodes`' two rejection branches — a Fatal `duplicate-id` on the edge and this
//! Error-level endpoint check — the endpoint one is the more telling: it is the only place in the
//! wires vocabulary where the diagnostic's target and the mutation's own `target()` disagree. The
//! payload's edge is `edge-alpha-beta`, but the reported id is the missing ENDPOINT `node-alpha`.
//! The committed BASE deliberately holds the edge's `target` endpoint (`node-beta`) so that the
//! `["source", "target"]` scan order is what decides which id gets named.

use crate::artifacts::wires::mutations::{DisconnectNodes, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::{find_board_edge, find_board_node};
use crate::artifacts::wires::{cache_wires_content, WiresDiff, WiresSnapshot};
use dsl::DslValue;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn board_entries(board: &DslValue, key: &str) -> Vec<DslValue> {
    board.get(key).and_then(|value| value.as_array()).map(|items| items.to_vec()).unwrap_or_default()
}

/// 🌱 The committed `⬅️before` with its composed content child resolved into the working-scene
/// cache from that snapshot's own persisted `wiresFixture.board` mirror — one node, `node-beta`,
/// and no edges at all.
fn before() -> WiresSnapshot {
    let snapshot: WiresSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let board = snapshot.wires_fixture.get("board").cloned().unwrap_or(DslValue::Null);
    cache_wires_content(&snapshot.content.child_id, board_entries(&board, "nodes"), board_entries(&board, "edges"));
    snapshot
}
fn expected_after() -> WiresSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> WiresMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `connect-nodes` leaves the document byte-identical to the committed `after`:
/// no edge on the board, and no relationship row appended to the wires lane either.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let (snapshot, _messages) = store::apply_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "connect-nodes/rejects-an-edge-whose-source-node-is-absent: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected connect must not mint a new content handle");
    assert_eq!(snapshot.wires_fixture, base.wires_fixture, "a rejected connect must not append the payload's relationship to the identities/relationships lane");
    assert!(find_board_edge(&snapshot, "edge-alpha-beta").is_none(), "the refused edge must not appear on the board");
}

/// 🔗 The endpoint check walks `["source", "target"]` in that order and reports the FIRST missing
/// endpoint's NODE id — not the edge id the mutation is otherwise addressed by. This is the one
/// wires verb whose diagnostic target and `SemanticMutation::target()` differ.
#[semio_framework_async_macros::async_test]
async fn the_missing_endpoint_node_is_reported_rather_than_the_edge() {
    let base = before();
    assert!(find_board_node(&base, "node-alpha").is_none(), "the edge's source endpoint must genuinely be absent");
    assert!(find_board_node(&base, "node-beta").is_some(), "while its target endpoint is present — so the scan order, not mere absence, decides the reported id");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &WiresDiff::default(), "a rejecting connect-nodes must carry an empty diff — neither the board nor the wires half of diff_wires_and_board may run");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected — the scan returns at the first miss, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a dangling endpoint is reported as target-missing, not as the duplicate-id Fatal a colliding edge id would raise");
    assert_eq!(messages[0].level, protocol::Severity::Error, "an unresolvable endpoint is recoverable, so it stays at Error");
    assert_eq!(messages[0].target, vec!["node-alpha".to_string()], "the diagnostic names the missing ENDPOINT node, never the payload's own edge id");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::target(&mutation()), vec!["edge-alpha-beta".to_string()], "while the mutation's own target stays the edge — the two deliberately disagree here");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("connect", "relationship", "connect-nodes", "ConnectedNodes"), "the fixture must be bound to connect-nodes' own descriptor — entity `relationship`, not `node`");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Connect nodes via edge \"edge-alpha-beta\"", "connect-nodes' undo label quotes the edge, matching its target() rather than its diagnostic");
}

/// ↩️ `connect-nodes`' inverse is PAYLOAD-derived — a `disconnect-nodes` of the edge id carried on
/// the payload, emitted even though the connect was refused and no BASE lookup ever succeeded.
/// Its `disconnect-nodes` counterpart, which reads BASE, would have produced nothing at all.
#[semio_framework_async_macros::async_test]
async fn inverse_is_always_a_disconnect_of_the_payload_edge_id() {
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &before());
    assert_eq!(inverse.len(), 1, "connect-nodes always undoes with exactly one step, got {inverse:?}");
    let WiresMutation::DisconnectNodes(DisconnectNodes { edge_id }) = &inverse[0] else {
        panic!("connect-nodes' inverse must be a disconnect-nodes, got {:?}", inverse[0]);
    };
    assert_eq!(edge_id, "edge-alpha-beta", "the inverse cuts exactly the edge id the payload's edge blob carried");
}

/// 🔣️ Both committed snapshots are canonical, and so is the committed `connectNodes` payload —
/// the vocabulary's only two-blob payload, carrying the board edge and its wires relationship side
/// by side, neither with a `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-nodes/rejects-an-edge-whose-source-node-is-absent: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-nodes/rejects-an-edge-whose-source-node-is-absent: committed connectNodes JSON is not canonical");
    assert!(original.get("edge").is_some() && original.get("relationship").is_some(), "both halves must be written out — a relationship-less edge is expressed as an explicit null, never by omission");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what `connect-nodes`' own diff
/// builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "connect-nodes/rejects-an-edge-whose-source-node-is-absent declares a rejected outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target — the endpoint node id, not the edge id");
}

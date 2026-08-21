//! 🧪️ `disconnect-nodes` fixture — `rejects-cutting-an-edge-the-board-never-carried`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION: a successful `disconnect-nodes` goes through
//! `fixtures_after_remove_edge` into `diff_wires_and_board`, and that builder's board half calls
//! `diff_board_fixture`, which re-mints the composed `s.stdio.semio.graph` child handle as
//! `format!("wires-content-{hash:016x}")` over a `DefaultHasher` digest. Hand-authoring the
//! resulting `➡️after` would mean hand-forging a value from `std`'s deliberately unspecified
//! default hasher. This verb has no no-op guard, so the `mutation.target-missing` branch — which
//! returns before either half of that pair runs — is what this case pins.
//!
//! ✂️ The fixture keeps a REAL edge (`edge-owns`) and its wires-level relationship in BASE. A
//! missed cut must leave both standing: `disconnect-nodes` is one of only two wires verbs that
//! write the `wiresFixture` identities/relationships lane at all, so proving it wrote nothing is
//! the point.

use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::{find_board_edge, find_relationship};
use crate::artifacts::wires::{cache_wires_content, wires_working_scene, WiresDiff, WiresSnapshot};
use dsl::DslValue;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn board_entries(board: &DslValue, key: &str) -> Vec<DslValue> {
    board.get(key).and_then(|value| value.as_array()).map(|items| items.to_vec()).unwrap_or_default()
}

/// 🌱 The committed `⬅️before` with its composed content child resolved into the working-scene
/// cache from that snapshot's own persisted `wiresFixture.board` mirror — two nodes and the one
/// edge that really does join them.
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

/// ▶️ A rejected `disconnect-nodes` leaves the document byte-identical to the committed `after` —
/// both the composed board edge and its wires-level relationship survive intact.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let (snapshot, _messages) = store::apply_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "disconnect-nodes/rejects-cutting-an-edge-the-board-never-carried: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected disconnect must not mint a new content handle");
    assert_eq!(snapshot.wires_fixture, base.wires_fixture, "a rejected disconnect must not touch the identities/relationships lane it would otherwise rewrite");
}

/// ✂️ `disconnect-nodes` is the only wires verb addressed by EDGE id: it searches `scene.edges`,
/// never `scene.nodes`, and reports the edge id verbatim rather than either endpoint node. Its
/// entity is `relationship`, not `node`.
#[semio_framework_async_macros::async_test]
async fn a_missing_edge_is_reported_by_its_edge_id_not_by_an_endpoint() {
    let base = before();
    let scene = wires_working_scene(&base);
    assert_eq!((scene.nodes.len(), scene.edges.len()), (2, 1), "the before-snapshot must resolve to a scene that really carries one edge, so the miss is a real scan");
    assert!(find_board_edge(&base, "edge-severed").is_none(), "edge-severed must genuinely be absent");
    assert!(find_board_edge(&base, "edge-owns").is_some(), "while edge-owns, the bystander, is present");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &WiresDiff::default(), "a rejecting disconnect-nodes must carry an empty diff — neither the board half nor the wires half of diff_wires_and_board may run");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing edge is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "disconnect-nodes has no Fatal branch at all — unlike its connect-nodes counterpart, which can raise duplicate-id");
    assert_eq!(messages[0].target, vec!["edge-severed".to_string()], "the diagnostic names the EDGE id, never node-source or node-sink");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("disconnect", "relationship", "disconnect-nodes", "DisconnectedNodes"),
        "the fixture must be bound to disconnect-nodes' own descriptor — entity `relationship`, not `node`"
    );
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Disconnect edge \"edge-severed\"", "disconnect-nodes' undo label quotes the edge id");
}

/// ↩️ `disconnect-nodes` inverts by rebuilding a `connect-nodes` from BOTH halves captured off
/// BASE — the board edge AND its `wiresFixture.relationships` row. With no such edge there is
/// nothing to rebuild, so the inverse is empty even though a matching relationship row for the
/// OTHER edge is sitting right there.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_edge_and_relationship_pair_to_restore() {
    let base = before();
    assert!(find_relationship(&base, "edge-owns").is_some(), "BASE really does carry a relationship row — for the bystander edge, not for the missed one");
    assert!(find_relationship(&base, "edge-severed").is_none(), "and none at all for the edge this mutation names");
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &base);
    assert!(inverse.is_empty(), "disconnect-nodes/rejects-cutting-an-edge-the-board-never-carried: a rejected disconnect must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are canonical, and so is the committed `disconnectNodes` payload —
/// a single `edgeId`, the narrowest payload in this vocabulary.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "disconnect-nodes/rejects-cutting-an-edge-the-board-never-carried: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "disconnect-nodes/rejects-cutting-an-edge-the-board-never-carried: committed disconnectNodes JSON is not canonical");
    assert_eq!(original.as_object().map(|fields| fields.len()), Some(2), "the payload is the tag plus edgeId and nothing else");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what `disconnect-nodes`' own
/// diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "disconnect-nodes/rejects-cutting-an-edge-the-board-never-carried declares a rejected outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

//! 🧪️ `delete-node` fixture — `rejects-deleting-a-node-the-board-never-held`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION rather than a real deletion: `WiresSnapshot` persists its
//! nodes/edges in a composed `s.stdio.semio.graph` CHILD, and every board-writing wires diff goes
//! through `diff_board_fixture`, which re-mints that child's handle as
//! `format!("wires-content-{hash:016x}")` over a `std::collections::hash_map::DefaultHasher`
//! digest of the child content. Hand-authoring the `➡️after` of a real delete would mean
//! hand-forging a value from `std`'s deliberately unspecified default hasher. `delete-node` has no
//! no-op guard to fall back on — a delete either removes something or misses — so the
//! `mutation.target-missing` branch, which reaches no hash at all, is what this case pins.
//!
//! 🗑️ The board here is NOT empty: it really holds `node-anchor`. The miss is therefore a genuine
//! scan of a populated node list, not the vacuous "nothing is resolved" case.

use crate::artifacts::wires::mutations::WiresMutation;
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
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
/// cache. Nothing is invented: the seeded node is the committed snapshot's own persisted
/// `wiresFixture.board` mirror, so `node-anchor` exists and `node-phantom` genuinely does not.
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

/// ▶️ A rejected `delete-node` leaves the document byte-identical to the committed `after` — and,
/// crucially for a composed artifact, leaves `node-anchor` standing.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let (snapshot, _messages) = store::apply_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-node/rejects-deleting-a-node-the-board-never-held: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected delete must not mint a new content handle");
    assert!(find_board_node(&snapshot, "node-anchor").is_some(), "the bystander node must survive a missed delete untouched");
}

/// 🗑️ `delete-node` addresses `scene.nodes` by node id and reports the id it was handed — an
/// Error, not the Fatal a duplicate identity would raise, because a missed delete breaks no
/// invariant.
#[semio_framework_async_macros::async_test]
async fn a_missing_node_is_reported_by_the_node_id_it_was_handed() {
    let base = before();
    assert_eq!(wires_working_scene(&base).nodes.len(), 1, "the before-snapshot must resolve to a populated scene, so the miss is a real scan");
    assert!(find_board_node(&base, "node-phantom").is_none(), "node-phantom must genuinely be absent from that scene");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &WiresDiff::default(), "a rejecting delete-node must carry an empty diff, never a half-rebuilt board");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "delete-node has no Fatal branch — a miss is recoverable, an id collision would not be");
    assert_eq!(messages[0].target, vec!["node-phantom".to_string()], "the diagnostic names the node id the payload carried, never the bystander that was actually on the board");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "node", "delete-node", "DeletedNode"), "the fixture must be bound to delete-node's own descriptor");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Delete node \"node-phantom\"", "delete-node's undo label quotes the id it was asked to remove");
}

/// ↩️ `delete-node`'s inverse is BASE-derived — it recreates the removed node from the full blob
/// it captured off the board, the exact opposite of `create-node`'s payload-derived inverse. With
/// nothing captured there is nothing to recreate, so the inverse is empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_captured_node_to_recreate() {
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "delete-node/rejects-deleting-a-node-the-board-never-held: a rejected delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point, and the
/// committed `deleteNode` payload is too.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-node/rejects-deleting-a-node-the-board-never-held: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-node/rejects-deleting-a-node-the-board-never-held: committed deleteNode JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what `delete-node`'s own diff
/// builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-node/rejects-deleting-a-node-the-board-never-held declares a rejected outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

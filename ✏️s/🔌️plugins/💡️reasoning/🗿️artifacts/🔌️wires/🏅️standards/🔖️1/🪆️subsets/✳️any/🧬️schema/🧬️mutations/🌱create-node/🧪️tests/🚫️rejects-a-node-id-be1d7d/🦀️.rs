//! 🧪️ `create-node` fixture — `🚫️rejects-a-node-id-the-board-already-holds`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION: a successful `create-node` runs `board_after_add_node` into
//! `diff_board_fixture`, which re-mints the composed `s.stdio.semio.graph` child handle as
//! `format!("wires-content-{hash:016x}")` over a `DefaultHasher` digest of the child content.
//! Hand-authoring that `➡️after` would mean hand-forging a value from `std`'s deliberately
//! unspecified default hasher. `create-node` has no no-op guard either — creating is never
//! idempotent — so the `mutation.duplicate-id` branch, which never reaches the hasher, is what
//! this case pins.
//!
//! 🌱 The collision is authentic in both directions: the committed `⬅️before`'s own persisted
//! `wiresFixture.board` mirror (which is what seeds the working scene) holds a node that is
//! field-for-field the node the committed payload asks to create.

use crate::artifacts::wires::mutations::{CreateNode, DeleteNode, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::{materialize_wires_content, WiresDiff, WiresSnapshot};
use dsl::DslValue;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn board_entries(board: &DslValue, key: &str) -> Vec<DslValue> {
    board.get(key).and_then(|value| value.as_array()).map(|items| items.to_vec()).unwrap_or_default()
}

/// 🌱 The committed `⬅️before` with its composed content child resolved into the working-scene
/// the exact child owner from that snapshot's own persisted `wiresFixture.board` mirror — the occupant whose id
/// the committed payload collides with.
fn before() -> WiresSnapshot {
    let mut snapshot: WiresSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let board = snapshot.wires_fixture.get("board").cloned().unwrap_or(DslValue::Null);
    materialize_wires_content(&mut snapshot.content, board_entries(&board, "nodes"), board_entries(&board, "edges"));
    snapshot
}
fn expected_after() -> WiresSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> WiresMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ A rejected `create-node` leaves the document byte-identical to the committed `after`; the
/// board keeps exactly the one `node-alpha` it started with.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let (snapshot, _messages) = store::apply_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-node/rejects-a-node-id-the-board-already-holds: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected create must not mint a new content handle");
}

/// 🚨️ A colliding node id is a FATAL `mutation.duplicate-id` — not the Error-level
/// `target-missing` the rest of this vocabulary raises. `create-node` is the only wires verb whose
/// target is read OUT OF the payload's node blob rather than from a dedicated id field, and the
/// diagnostic addresses exactly that id.
#[semio_framework_async_macros::async_test]
async fn a_colliding_node_id_is_a_fatal_duplicate_id() {
    let base = before();
    let WiresMutation::CreateNode(CreateNode { node }) = mutation() else {
        panic!("rejects-a-node-id-the-board-already-holds's committed mutation must be a create-node");
    };
    assert_eq!(find_board_node(&base, "node-alpha").as_ref(), Some(&node), "the occupant seeded from the before-snapshot must be field-for-field the node the payload asks to create");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &WiresDiff::default(), "a rejecting create-node must carry an empty diff, never a half-built content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb a broken identity");
    assert_eq!(messages[0].target, vec!["node-alpha".to_string()], "the diagnostic addresses the colliding id lifted out of the payload's node blob");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "node", "create-node", "CreatedNode"), "the fixture must be bound to create-node's own descriptor");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Add node \"node-alpha\"", "create-node's undo label reads \"Add\", the one place this verb's prose and its `create` descriptor diverge");
}

/// ↩️ `create-node`'s inverse is PAYLOAD-derived, not BASE-derived: it is a `delete-node` of the id
/// it was asked to create, emitted even when the create itself was refused — the mirror image of
/// `delete-node`, which has nothing to undo unless BASE really held the node.
#[semio_framework_async_macros::async_test]
async fn inverse_is_always_a_delete_of_the_requested_id() {
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &before());
    assert_eq!(inverse.len(), 1, "create-node always undoes with exactly one step, got {inverse:?}");
    let WiresMutation::DeleteNode(DeleteNode { node_id }) = &inverse[0] else {
        panic!("create-node's inverse must be a delete-node, got {:?}", inverse[0]);
    };
    assert_eq!(node_id, "node-alpha", "the inverse deletes exactly the id the payload's node blob carried");
}

/// 🔣️ Both committed snapshots are canonical, and so is the committed `createNode` payload —
/// whose whole `node` blob is a `DslValue`, so every one of its numbers re-encodes with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/rejects-a-node-id-the-board-already-holds: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-node/rejects-a-node-id-the-board-already-holds: committed createNode JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what `create-node`'s own diff
/// builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-node/rejects-a-node-id-the-board-already-holds declares a rejected outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

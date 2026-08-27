//! 🧪️ `change-node-kind` fixture — `reports-a-no-op-when-the-kind-already-reads-topic`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` twins are generated from it by `fixtures generate`.
//!
//! ⚠️ Wires keeps its board in a composed `s.stdio.semio.graph` CHILD whose `child_id` is a
//! `DefaultHasher` digest of the child content, re-minted by `diff_board_fixture` on every
//! board-writing diff — a kind-CHANGING `➡️after` would therefore need a hand-forged `std`
//! default-hash value. `change-node-kind`'s own guard gives an honest way out that is still an
//! APPLIED case: when `nodeKind` already reads what the payload asks for, the builder returns
//! `MutationOutcome::empty().warn("mutation.no-op", …)` before `set_node_field` is ever called.
//!
//! 🏷️ `change-node-kind` and its sibling `change-node-shape` share verb AND entity (`change`/
//! `node`); only `kind`/`record` tell them apart, and this fixture pins that this leaf is the
//! `nodeKind` one.

use crate::artifacts::wires::mutations::{ChangeNodeKind, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::{materialize_wires_content, WiresDiff, WiresSnapshot};
use dsl::DslValue;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn board_entries(board: &DslValue, key: &str) -> Vec<DslValue> {
    board.get(key).and_then(|value| value.as_array()).map(|items| items.to_vec()).unwrap_or_default()
}

/// 🌱 The committed `⬅️before` with its composed content child resolved into the working-scene
/// child owner, materialized from that same snapshot's own persisted `wiresFixture.board` mirror — the inline
/// board copy this artifact still carries beside the composed child.
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

/// ▶️ Re-declaring `node-metabolism`'s kind as the `topic` it already is carries `before` to
/// exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _messages) = store::apply_mutation(&before(), &mutation()).expect("change-node-kind's empty no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.wires_fixture, before().wires_fixture, "change-node-kind never writes the wires-level identities/relationships layer — only connect/disconnect do");
}

/// ↩️ `change-node-kind` inverts with a second `change-node-kind` carrying BASE's own old kind
/// string back; replayed after the forward step it restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _messages) = store::apply_mutation(&base, &mutation).expect("forward kind change applies");
    for step in &inverse {
        snapshot = store::apply_mutation(&snapshot, step).expect("inverse kind-change step applies").0;
    }
    assert_eq!(snapshot, base, "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `changeNodeKind` payload are canonical:
/// decode→encode is a fixed point. `newNodeKind` has no `skip_serializing_if`, so it is always
/// written.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic: committed changeNodeKind JSON is not canonical");
    assert_eq!(original.get("mutation").and_then(serde_json::Value::as_str), Some("changeNodeKind"), "the internally-tagged variant name must be the camelCased ChangeNodeKind");
}

/// 🎯️ The declared outcome — `applied` with one `warn`/`mutation.no-op` — is exactly what the
/// already-that-kind guard emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic declares an applied outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), messages.len(), "the declared diagnostic count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "an already-that-kind change is declared at the fixture contract's `warn` level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "which is `Severity::Warning` in Rust — a redundant kind change is never an Error");
}

/// 🔺️ The produced delta is `WiresDiff::default()`: because the guard returns first, `content`
/// stays `null` instead of being swapped for a freshly hashed — and byte-for-byte identical —
/// child handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff(), &WiresDiff::default(), "a redundant kind change must carry the empty diff, never a re-minted content child");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `WiresDiff` — a `#[serde(default)]`
/// container with no per-field skips, so every slot is written out as `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic: committed diff JSON is not canonical");
    assert!(original.get("content").is_some_and(serde_json::Value::is_null), "the committed diff must state explicitly that no content child was minted");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — nothing at
/// all changes hands.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-node-kind/reports-a-no-op-when-the-kind-already-reads-topic: committed diff did not carry before to after");
}

/// 🏷️ The guard is a verbatim `Some(&str)` comparison against the node's `nodeKind` key, and the
/// inverse is BASE-derived — it reads the old kind back off the board rather than off the payload.
/// This test also pins the descriptor that separates this leaf from `change-node-shape`, with
/// which it shares both verb and entity.
#[semio_framework_async_macros::async_test]
async fn the_guard_compares_the_nodekind_key_verbatim() {
    let base = before();
    let node = find_board_node(&base, "node-metabolism").expect("the committed before-snapshot holds node-metabolism");
    assert_eq!(node.get("nodeKind").and_then(|value| value.as_str()), Some("topic"), "the guard fires because BASE's nodeKind key already reads exactly the payload's newNodeKind");
    assert_eq!(node.get("shape").and_then(|value| value.as_str()), Some("circle"), "a kind change never touches `shape` — that is change-node-shape's key");
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-node-kind always undoes with exactly one change-node-kind, got {inverse:?}");
    let WiresMutation::ChangeNodeKind(ChangeNodeKind { node_id, new_node_kind }) = &inverse[0] else {
        panic!("change-node-kind's inverse must itself be a change-node-kind, got {:?}", inverse[0]);
    };
    assert_eq!((node_id.as_str(), new_node_kind.as_str()), ("node-metabolism", "topic"), "the inverse restores BASE's own kind on BASE's own node");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "node", "change-node-kind", "ChangedNodeKind"), "the fixture must be bound to change-node-kind's own descriptor, not its change-node-shape sibling's");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Change node \"node-metabolism\" kind to \"topic\"", "change-node-kind's undo label quotes both the node and the kind");
}

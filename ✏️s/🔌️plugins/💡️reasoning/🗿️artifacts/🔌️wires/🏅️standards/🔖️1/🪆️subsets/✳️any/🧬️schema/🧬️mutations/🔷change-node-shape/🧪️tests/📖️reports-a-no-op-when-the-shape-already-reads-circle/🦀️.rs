//! 🧪️ `change-node-shape` fixture — `📖️reports-a-no-op-when-the-shape-already-reads-circle`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the `.semio` twins are generated from it.
//!
//! ⚠️ A shape-CHANGING `➡️after` is unhand-authorable for this artifact — `diff_board_fixture`
//! mints the composed `s.stdio.semio.graph` child handle from a `DefaultHasher` digest of the
//! child content. `change-node-shape`'s already-that-shape guard is a real APPLIED exit that never
//! reaches that hash: it returns `MutationOutcome::empty().warn("mutation.no-op", …)`.
//!
//! 🔷 This leaf owns exactly one key, `shape`. It deliberately does NOT strip or rewrite the
//! extent keys a former rectangle left behind — `resize-node` owns `radius`/`width`/`height` — so
//! the committed node keeps its `radius` untouched throughout.

use crate::artifacts::wires::mutations::{ChangeNodeShape, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::{materialize_wires_content, WiresDiff, WiresSnapshot};
use dsl::DslValue;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn board_entries(board: &DslValue, key: &str) -> Vec<DslValue> {
    board.get(key).and_then(|value| value.as_array()).map(|items| items.to_vec()).unwrap_or_default()
}

/// 🌱 The committed `⬅️before` with its composed content child resolved into the working-scene
/// child owner, materialized from the snapshot's own persisted `wiresFixture.board` mirror.
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

/// ▶️ Re-declaring `node-orbit` a circle when it already is one carries `before` to exactly the
/// committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _messages) = store::apply_mutation(&before(), &mutation()).expect("change-node-shape's empty no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, before().content, "a redundant shape change must leave the composed content handle exactly as it found it");
}

/// ↩️ `change-node-shape` inverts with a second `change-node-shape` carrying BASE's own old shape
/// string; replayed after the forward step it restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _messages) = store::apply_mutation(&base, &mutation).expect("forward shape change applies");
    for step in &inverse {
        snapshot = store::apply_mutation(&snapshot, step).expect("inverse shape-change step applies").0;
    }
    assert_eq!(snapshot, base, "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `changeNodeShape` payload are canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle: committed changeNodeShape JSON is not canonical");
    assert_eq!(original.get("mutation").and_then(serde_json::Value::as_str), Some("changeNodeShape"), "the internally-tagged variant name must be the camelCased ChangeNodeShape, not its ChangeNodeKind sibling");
}

/// 🎯️ The declared outcome — `applied` with one `warn`/`mutation.no-op` — is exactly what the
/// already-that-shape guard emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle declares an applied outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), messages.len(), "the declared diagnostic count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "an already-that-shape change is declared at the fixture contract's `warn` level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "which is `Severity::Warning` in Rust — a redundant shape change is not an error");
}

/// 🔺️ The produced delta is `WiresDiff::default()` — the guard returns before the `shape` write,
/// so nothing is hashed into a new content child.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff(), &WiresDiff::default(), "a redundant shape change must carry the empty diff, never a re-minted content child");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `WiresDiff`; its whole-artifact `artifact`
/// slot stays `null` — no wires verb replaces the document wholesale.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle: committed diff JSON is not canonical");
    assert!(original.get("artifact").is_some_and(serde_json::Value::is_null), "a sparse shape delta must never degrade into a whole-artifact replacement");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-node-shape/reports-a-no-op-when-the-shape-already-reads-circle: committed diff did not carry before to after");
}

/// 🔷 The guard reads the `shape` key alone; the node's extent keys are none of this verb's
/// business and stay exactly as BASE has them. The descriptor is what separates this leaf from
/// `change-node-kind`, with which it shares both verb and entity.
#[semio_framework_async_macros::async_test]
async fn the_shape_guard_leaves_every_extent_key_alone() {
    let base = before();
    let node = find_board_node(&base, "node-orbit").expect("the committed before-snapshot holds node-orbit");
    assert_eq!(node.get("shape").and_then(|value| value.as_str()), Some("circle"), "the guard fires because BASE's `shape` key already reads the payload's newShape");
    assert_eq!(node.get("radius").and_then(|value| value.as_f64()), Some(24.0), "the circle's radius belongs to resize-node and must survive a shape verb untouched");
    assert!(node.get("width").is_none() && node.get("height").is_none(), "change-node-shape never invents the extent keys the other shape would use");
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-node-shape always undoes with exactly one change-node-shape, got {inverse:?}");
    let WiresMutation::ChangeNodeShape(ChangeNodeShape { node_id, new_shape }) = &inverse[0] else {
        panic!("change-node-shape's inverse must itself be a change-node-shape, got {:?}", inverse[0]);
    };
    assert_eq!((node_id.as_str(), new_shape.as_str()), ("node-orbit", "circle"), "the inverse restores BASE's own shape on BASE's own node");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "node", "change-node-shape", "ChangedNodeShape"), "the fixture must be bound to change-node-shape's own descriptor, not its change-node-kind sibling's");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Change node \"node-orbit\" shape to \"circle\"", "change-node-shape's undo label quotes both the node and the shape");
}

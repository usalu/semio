//! 🧪️ `move-node` fixture — `reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); `fixtures generate` derives the `.semio` encodings
//! from it later.
//!
//! ⚠️ A position-CHANGING `➡️after` is not hand-authorable for this artifact: `diff_board_fixture`
//! re-mints the composed `s.stdio.semio.graph` child handle as a `DefaultHasher` digest of the
//! child content, and `std`'s default hasher is deliberately unspecified. `move-node` still yields
//! a real APPLIED case because its diff builder has three exits before that point — a
//! `target-missing` Error, a non-finite `invariant` Fatal, and the already-there `mutation.no-op`
//! warn this fixture pins.
//!
//! 🧭 The comparison is not against the raw `x`/`y` keys but against `node_position`, which
//! defaults a MISSING coordinate to the origin. The committed node carries `x` and no `y` at all,
//! so `newY: 0.0` is "already there" — a shape only this leaf's guard produces.

use crate::artifacts::wires::mutations::{MoveNode, WiresMutation};
use crate::artifacts::wires::schema::node_position;
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

/// ▶️ Moving `node-drifter` to the point it already occupies carries `before` to exactly the
/// committed `after`, and never fabricates the `y` key the node does not have.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _messages) = store::apply_mutation(&before(), &mutation()).expect("move-node's empty no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero: applied state differs from committed after-snapshot");
    assert!(find_board_node(&snapshot, "node-drifter").expect("node-drifter survives").get("y").is_none(), "a no-op move must not materialize the coordinate key it only read through a default");
}

/// ↩️ `move-node` inverts with a second `move-node` carrying BASE's own old position; replayed
/// after the forward step it restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _messages) = store::apply_mutation(&base, &mutation).expect("forward move applies");
    for step in &inverse {
        snapshot = store::apply_mutation(&snapshot, step).expect("inverse move step applies").0;
    }
    assert_eq!(snapshot, base, "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `moveNode` payload are canonical: decode→encode
/// is a fixed point. `newX`/`newY` are bare `f64`s, so both always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero: committed moveNode JSON is not canonical");
    assert_eq!(original.get("newY").and_then(serde_json::Value::as_f64), Some(0.0), "the payload asks for the origin ordinate explicitly, which is what the missing key defaults to");
}

/// 🎯️ The declared outcome — `applied` with one `warn`/`mutation.no-op` — is what the
/// already-at-that-point guard emits, and it is reached only after the finite-coordinate check
/// passes.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero declares an applied outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), messages.len(), "the declared diagnostic count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "an already-there move is declared at the fixture contract's `warn` level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "which is `Severity::Warning` — this is NOT the `Fatal` mutation.invariant a non-finite coordinate would raise");
}

/// 🔺️ The produced delta is `WiresDiff::default()`: the guard returns before either of the two
/// `set_node_field` writes (`x` then `y`) runs, so no content child is ever hashed.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff(), &WiresDiff::default(), "an already-there move must carry the empty diff, never a re-minted content child");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `WiresDiff`, whose drag slots stay `null` —
/// `move-node` is the persisted reposition, not the ephemeral canvas drag.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero: committed diff JSON is not canonical");
    for slot in ["dragNodeId", "dragLastX", "dragLastY"] {
        assert!(original.get(slot).is_some_and(serde_json::Value::is_null), "move-node must leave the ephemeral drag slot {slot} alone");
    }
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-node/reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero: committed diff did not carry before to after");
}

/// 🧭 `move-node` compares through `node_position`, so a node with no `y` key is genuinely "at
/// y = 0"; the inverse then writes that defaulted ordinate out as an explicit coordinate — the
/// undo step is fuller than the node it was read from.
#[semio_framework_async_macros::async_test]
async fn a_missing_ordinate_reads_as_the_origin_through_node_position() {
    let base = before();
    let node = find_board_node(&base, "node-drifter").expect("the committed before-snapshot holds node-drifter");
    assert!(node.get("y").is_none(), "the committed node deliberately carries no `y` key — that is what this case exercises");
    assert_eq!(node_position(&node), (12.0, 0.0), "node_position must default the missing ordinate to the origin while keeping the real abscissa");
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "move-node always undoes with exactly one move-node, got {inverse:?}");
    let WiresMutation::MoveNode(MoveNode { node_id, new_x, new_y }) = &inverse[0] else {
        panic!("move-node's inverse must itself be a move-node, got {:?}", inverse[0]);
    };
    assert_eq!(node_id, "node-drifter", "the inverse addresses the same node the payload did");
    assert_eq!((*new_x, *new_y), (12.0, 0.0), "the inverse is an ABSOLUTE position read off BASE, never a captured offset");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("move", "node", "move-node", "MovedNode"), "the fixture must be bound to move-node's own descriptor");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Move node \"node-drifter\" to (12, 0)", "move-node's undo label renders both coordinates unquoted and unpadded");
}

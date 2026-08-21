//! 🧪️ `set-node-root` fixture — `reports-a-no-op-when-an-unflagged-node-is-set-to-not-root`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the five `.semio` encodings are derived from it by
//! `fixtures generate`, not asserted here.
//!
//! ⚠️ Every board-WRITING wires diff funnels through `diff_board_fixture`, which re-mints the
//! composed `s.stdio.semio.graph` child handle as a `DefaultHasher` digest — unhand-authorable.
//! `set-node-root` reaches an APPLIED state without that hash because its guard short-circuits:
//! the flag already holds, so `MutationOutcome::empty().warn("mutation.no-op", …)` comes back and
//! `set_node_field(.., "root", ..)` is never called.
//!
//! 🚩 This leaf's guard is the ONE in the wires vocabulary that reads an ABSENT key as a value:
//! `node.get("root").and_then(as_bool).unwrap_or(false)`. The committed node therefore carries no
//! `root` key at all, and `newRoot: false` is still a no-op — a fixture the sibling
//! `change-node-kind`/`change-node-shape`/`edit-node-text` guards (strict `Some(_)` comparisons)
//! could not produce.

use crate::artifacts::wires::mutations::{SetNodeRoot, WiresMutation};
use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::artifacts::wires::{cache_wires_content, WiresDiff, WiresSnapshot};
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
/// cache from the snapshot's own persisted `wiresFixture.board` mirror — no node is invented here.
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

/// ▶️ Clearing a root flag `node-leaf` never had carries `before` to exactly the committed
/// `after` — and specifically does NOT materialize a `"root": false` key on the node.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _messages) = store::apply_mutation(&before(), &mutation()).expect("set-node-root's empty no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root: applied state differs from committed after-snapshot");
    assert!(find_board_node(&snapshot, "node-leaf").expect("node-leaf survives").get("root").is_none(), "a no-op set must not write the flag key it decided was already correct");
}

/// ↩️ `set-node-root` inverts with a second `set-node-root` carrying BASE's own old flag — here
/// the defaulted `false` — and replaying it restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _messages) = store::apply_mutation(&base, &mutation).expect("forward root set applies");
    for step in &inverse {
        snapshot = store::apply_mutation(&snapshot, step).expect("inverse root-set step applies").0;
    }
    assert_eq!(snapshot, base, "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `setNodeRoot` payload are canonical:
/// decode→encode is a fixed point. `newRoot` is a bare `bool`, so `false` is written, never
/// omitted.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root: committed setNodeRoot JSON is not canonical");
    assert_eq!(original.get("newRoot").and_then(serde_json::Value::as_bool), Some(false), "the payload must state the flag explicitly — a bare bool has no skip_serializing_if");
}

/// 🎯️ The declared outcome — `applied` with one `warn`/`mutation.no-op` — is exactly what the
/// already-that-flag guard emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root declares an applied outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), messages.len(), "the declared diagnostic count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a redundant flag set is declared at the fixture contract's `warn` level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "which is `Severity::Warning` in Rust — setting a flag to the value it already holds is not an error");
}

/// 🔺️ The produced delta is `WiresDiff::default()` — the guard returns before `diff_board_fixture`
/// can hash a new content child out of an unchanged board.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff(), &WiresDiff::default(), "a redundant root set must carry the empty diff, never a re-minted content child");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `WiresDiff`; every one of its slots is
/// spelled out as `null`, including the `config`-lane `locale`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root: committed diff JSON is not canonical");
    assert!(original.get("locale").is_some_and(serde_json::Value::is_null), "the config-lane slot must be present and null — a node-flag verb never touches the locale");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "set-node-root/reports-a-no-op-when-an-unflagged-node-is-set-to-not-root: committed diff did not carry before to after");
}

/// 🚩 An absent `root` key reads as `false`, not as "unknown": `unwrap_or(false)` is what makes
/// this case a no-op even though the node has no such key. The inverse materializes that same
/// default as an explicit `newRoot: false`.
#[semio_framework_async_macros::async_test]
async fn an_absent_root_key_reads_as_false_rather_than_as_missing() {
    let base = before();
    let node = find_board_node(&base, "node-leaf").expect("the committed before-snapshot holds node-leaf");
    assert!(node.get("root").is_none(), "the committed node deliberately carries NO root key — that is the whole point of this case");
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.messages().len(), 1, "the defaulted flag must still trip the no-op guard, got {:?}", outcome.messages());
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "set-node-root always undoes with exactly one set-node-root, got {inverse:?}");
    let WiresMutation::SetNodeRoot(SetNodeRoot { node_id, new_root }) = &inverse[0] else {
        panic!("set-node-root's inverse must itself be a set-node-root, got {:?}", inverse[0]);
    };
    assert_eq!(node_id, "node-leaf", "the inverse addresses the same node the payload did");
    assert!(!*new_root, "the inverse spells out the `false` the missing key stood for");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("set", "node", "set-node-root", "SetNodeRoot"), "set-node-root is the vocabulary's only `set` verb, and the only one whose record name is not past tense");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Set node \"node-leaf\" root to false", "set-node-root's undo label renders the bool unquoted");
}

//! 🧪️ `edit-node-text` fixture — `🔤️reports-a-no-op-when-the-label-is-retyped-verbatim`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the `.semio` encodings are generated from it.
//!
//! ⚠️ A text-CHANGING `➡️after` cannot be hand-authored here: `diff_board_fixture` re-mints the
//! composed `s.stdio.semio.graph` child handle as a `DefaultHasher` digest of the child content.
//! `edit-node-text`'s unchanged-text guard reaches an APPLIED outcome without touching that hash,
//! returning `MutationOutcome::empty().warn("mutation.no-op", …)` instead.
//!
//! ✏️ `edit-node-text` is the wires vocabulary's only `edit` verb — the board node's `text` is
//! treated as an authored content body rather than a bare scalar rename, which is why it does not
//! share `change`'s verb with `change-node-kind`/`change-node-shape`.

use crate::artifacts::wires::mutations::{EditNodeText, WiresMutation};
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

/// ▶️ Retyping `node-thesis`'s label as the string it already carries takes `before` to exactly
/// the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _messages) = store::apply_mutation(&before(), &mutation()).expect("edit-node-text's empty no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.camera, before().camera, "a label edit never touches the persisted board camera — that field lives outside the composed graph subset");
}

/// ↩️ `edit-node-text` inverts with a second `edit-node-text` carrying BASE's own old label back;
/// replayed after the forward step it restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _messages) = store::apply_mutation(&base, &mutation).expect("forward text edit applies");
    for step in &inverse {
        snapshot = store::apply_mutation(&snapshot, step).expect("inverse text-edit step applies").0;
    }
    assert_eq!(snapshot, base, "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `editNodeText` payload are canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim: committed editNodeText JSON is not canonical");
    assert_eq!(original.get("newText").and_then(serde_json::Value::as_str), Some("Thesis"), "the payload must carry the very label BASE already holds — otherwise this stops being a no-op case");
}

/// 🎯️ The declared outcome — `applied` with one `warn`/`mutation.no-op` — is exactly what the
/// unchanged-text guard emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim declares an applied outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), messages.len(), "the declared diagnostic count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a verbatim retype is declared at the fixture contract's `warn` level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "which is `Severity::Warning` in Rust — retyping the same label is not an error");
}

/// 🔺️ The produced delta is `WiresDiff::default()`: the guard returns before the `text` write, so
/// no content child is minted and `wiresFixture` is not restated either.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff(), &WiresDiff::default(), "a verbatim retype must carry the empty diff, never a re-minted content child");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `WiresDiff`; `wiresFixture` stays `null`
/// because only `connect-nodes`/`disconnect-nodes` ever write that lane.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim: committed diff JSON is not canonical");
    assert!(original.get("wiresFixture").is_some_and(serde_json::Value::is_null), "a node-text verb must never restate the identities/relationships lane");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-node-text/reports-a-no-op-when-the-label-is-retyped-verbatim: committed diff did not carry before to after");
}

/// ✏️ The `text` key is what this leaf reads and writes — never the node's `nodeKind` label
/// projection — and its inverse is BASE-derived: the old body is looked up on the board, not
/// carried on the payload.
#[semio_framework_async_macros::async_test]
async fn the_old_body_is_recovered_from_the_board_not_from_the_payload() {
    let base = before();
    let node = find_board_node(&base, "node-thesis").expect("the committed before-snapshot holds node-thesis");
    assert_eq!(node.get("text").and_then(|value| value.as_str()), Some("Thesis"), "the guard fires because BASE's `text` key already reads the payload's newText");
    assert_eq!(node.get("nodeKind").and_then(|value| value.as_str()), Some("identity"), "the node's kind is untouched and unrelated — `text` is the only key this verb owns");
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "edit-node-text always undoes with exactly one edit-node-text, got {inverse:?}");
    let WiresMutation::EditNodeText(EditNodeText { node_id, new_text }) = &inverse[0] else {
        panic!("edit-node-text's inverse must itself be an edit-node-text, got {:?}", inverse[0]);
    };
    assert_eq!((node_id.as_str(), new_text.as_str()), ("node-thesis", "Thesis"), "the inverse restores the body read off BASE");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("edit", "node", "edit-node-text", "EditedNodeText"), "edit-node-text is the vocabulary's only `edit` verb");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Edit node \"node-thesis\" text to \"Thesis\"", "edit-node-text's undo label quotes the body it wrote");
}

//! 🧪️ `resize-node` fixture — `reports-a-no-op-when-the-radius-already-matches`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ⚠️ `WiresSnapshot` keeps its nodes/edges in a composed `s.stdio.semio.graph` CHILD whose
//! `child_id` every board-writing `🔺️diff` re-mints as a `DefaultHasher` digest of the child
//! content (`wires_content_child_handle`), so a board-CHANGING `➡️after` is not hand-authorable.
//! `resize-node`'s own diff builder, however, returns through a real no-op guard BEFORE it ever
//! reaches `diff_board_fixture`: an extent that already matches yields `MutationOutcome::empty()`
//! plus a `warn`/`mutation.no-op`. That branch mints no handle at all, so this case is a genuine
//! APPLIED one — empty diff, `➡️after` equal to `⬅️before`, and every one of the seven fixture
//! assertions live.
//!
//! 📐 The committed node deliberately carries `width`/`height` alongside `radius` — the state a
//! board node is really left in after a `change-node-shape` rectangle→circle flip, which rewrites
//! `shape` alone and never strips the other extent keys. That is what makes this leaf's
//! payload-masked inverse observable.

use crate::artifacts::wires::mutations::{WiresMutation, ResizeNode};
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

/// 🌱 The committed `⬅️before`, with its composed content child resolved into the working-scene
/// cache. Nothing is invented: the seed is the committed snapshot's OWN persisted
/// `wiresFixture.board` mirror, the inline copy of the board that survived the composed-child
/// migration untouched.
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

/// ▶️ Resizing `node-nucleus` to the radius it already has carries `before` to exactly the
/// committed `after` — an applied mutation, not a refused one.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _messages) = store::apply_mutation(&before(), &mutation()).expect("resize-node's empty no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "resize-node/reports-a-no-op-when-the-radius-already-matches: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, before().content, "a no-op resize must leave the composed content handle exactly as it found it");
}

/// ↩️ `resize-node` inverts with a second `resize-node` carrying the OLD extent read off BASE;
/// replaying it after the forward step restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _messages) = store::apply_mutation(&base, &mutation).expect("forward resize applies");
    for step in &inverse {
        snapshot = store::apply_mutation(&snapshot, step).expect("inverse resize step applies").0;
    }
    assert_eq!(snapshot, base, "resize-node/reports-a-no-op-when-the-radius-already-matches: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `resizeNode` payload are already canonical:
/// decode→encode is a fixed point. `newWidth`/`newHeight` are absent from the payload JSON on
/// purpose — both carry `skip_serializing_if = "Option::is_none"`, so serde omits them.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: WiresSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-node/reports-a-no-op-when-the-radius-already-matches: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "resize-node/reports-a-no-op-when-the-radius-already-matches: committed resizeNode JSON is not canonical");
    assert!(original.get("newWidth").is_none() && original.get("newHeight").is_none(), "an untouched extent field must be OMITTED from the payload, never written as null");
}

/// 🎯️ The declared outcome — `applied` carrying one `warn`/`mutation.no-op` — is exactly what
/// `resize-node`'s unchanged-extent guard emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "resize-node/reports-a-no-op-when-the-radius-already-matches declares an applied outcome");
    let produced = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its diagnostics");
    assert_eq!(declared.len(), messages.len(), "the declared diagnostic count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a no-op resize is declared at the fixture contract's `warn` level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "which is `Severity::Warning` in Rust — an unchanged extent is a nudge, never an Error or a Fatal");
    assert!(messages[0].target.is_empty(), "resize-node's no-op is raised through the 2-arg `warn` builder, so it carries no target address");
}

/// 🔺️ The delta `resize-node` produces here is the artifact's own `WiresDiff::default()` — every
/// slot `null`. This is the whole point of the case: the unchanged-extent guard returns before
/// `diff_board_fixture` can mint a fresh content handle, so `content` stays absent rather than
/// being replaced by an identical-looking one.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.diff(), &WiresDiff::default(), "a no-op resize must carry the empty diff, never a re-minted content child");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "resize-node/reports-a-no-op-when-the-radius-already-matches: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to `WiresDiff`. `WiresDiff` carries a
/// container-level `#[serde(default)]` and no per-field `skip_serializing_if`, so all nine slots
/// must be present and `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "resize-node/reports-a-no-op-when-the-radius-already-matches: committed diff JSON is not canonical");
    assert_eq!(original.as_object().map(|slots| slots.len()), Some(9), "every WiresDiff slot must be written out, `null` included");
}

/// 🩹 Applying the committed diff straight to `before` yields the committed `after` — for a no-op
/// that means it must change nothing at all, `content` handle included.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: WiresDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "resize-node/reports-a-no-op-when-the-radius-already-matches: committed diff did not carry before to after");
}

/// 📐 `resize-node` is the only wires verb with a three-field optional payload, and its inverse is
/// MASKED BY THE PAYLOAD rather than copied from BASE: `payload.new_*.and(base value)` keeps
/// `width`/`height` at `None` even though the committed node really does carry `96`/`48`. Undoing
/// a radius-only resize must never silently restate an extent nobody touched.
#[semio_framework_async_macros::async_test]
async fn the_inverse_carries_back_only_the_extent_the_payload_touched() {
    let base = before();
    let node = find_board_node(&base, "node-nucleus").expect("the committed before-snapshot holds node-nucleus");
    assert_eq!(node.get("radius").and_then(|value| value.as_f64()), Some(24.0), "the guard fires because BASE's radius already equals the payload's newRadius");
    assert_eq!(node.get("width").and_then(|value| value.as_f64()), Some(96.0), "BASE carries a leftover rectangle width the payload never mentions");
    let inverse = <WiresMutation as protocol::Mutation<WiresSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "resize-node always undoes with exactly one resize-node, got {inverse:?}");
    let WiresMutation::ResizeNode(ResizeNode { node_id, new_radius, new_width, new_height }) = &inverse[0] else {
        panic!("resize-node's inverse must itself be a resize-node, got {:?}", inverse[0]);
    };
    assert_eq!(node_id, "node-nucleus", "the inverse addresses the same node the payload did");
    assert_eq!(*new_radius, Some(24.0), "the touched extent field comes back with BASE's own radius");
    assert_eq!((*new_width, *new_height), (None, None), "the untouched extent fields stay None — masked by the payload, not filled in from BASE's 96/48");
    let semantics = <WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("resize", "node", "resize-node", "ResizedNode"), "the fixture must be bound to resize-node's own descriptor");
    assert_eq!(<WiresMutation as protocol::SemanticMutation<WiresSnapshot>>::label(&mutation()), "Resize node \"node-nucleus\"", "resize-node's undo label names the node but never the extent");
}

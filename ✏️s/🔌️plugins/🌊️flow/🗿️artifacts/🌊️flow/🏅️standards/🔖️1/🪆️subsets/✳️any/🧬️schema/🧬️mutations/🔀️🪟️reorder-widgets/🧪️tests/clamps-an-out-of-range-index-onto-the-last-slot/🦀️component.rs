//! 🧪️ `reorder-widgets` fixture — `clamps-an-out-of-range-index-onto-the-last-slot`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✅️ This is an APPLIED case with an EMPTY diff. `reorder-widgets`' own `🔺️diff` leaf clamps the
//! requested position with `to_index.min(widgets.len() - 1)` and then, when that lands on the
//! widget's current index, returns `MutationOutcome::empty().warn("mutation.no-op", …)` without ever
//! reaching `diff_replace_content`. Because no new content handle is minted, `➡️after` equals
//! `⬅️before` and the committed diff is `FlowDiff`'s all-`null` `Default` — the only honestly
//! hand-authorable applied state for a flow verb, since every state-changing flow diff addresses its
//! composed `s.stdio.semio.flow` CHILD by a `DefaultHasher` digest of the child content.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::Widget;
use protocol::Identified;
use std::collections::BTreeMap;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> FlowMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FlowSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🔀️ The committed `⬅️before`, with its composed child resolved to an ORDERED pair of widgets;
/// `note-beta` sits in the last slot, index 1, which is where the payload's out-of-range index 9
/// clamps to.
fn before() -> FlowSnapshot {
    let snapshot: FlowSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let widgets = vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, Widget::InputNote { id: "note-beta".into(), text: "Beta".into() }];
    cache_flow_content(&snapshot.content.child_id, widgets, Vec::new(), BTreeMap::new());
    snapshot
}

/// ▶️ The clamped reorder carries `before` to exactly the committed `after` — the widget order is
/// left alone, and with it the content handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("reorder-widgets' no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a clamped-to-current reorder must leave the flow-content handle untouched");
    assert_eq!(flow_working_scene(&snapshot).widgets.iter().map(|widget| widget.id().clone()).collect::<Vec<_>>(), vec!["note-alpha".to_string(), "note-beta".to_string()], "the widget ORDER is what this verb owns, and it must be unchanged");
}

/// ↩️ `reorder-widgets` inverts BASE-first: it reads the widget's ORIGINAL index out of the scene
/// and asks for that index back. Here that is 1 — the same slot the payload's 9 clamped to — so the
/// undo is itself a no-op and the round trip lands exactly on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_flow_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "reorder-widgets always undoes with exactly one reorder, got {inverse:?}");
    let FlowMutation::ReorderWidgets(undo) = &inverse[0] else {
        panic!("reorder-widgets' inverse must be a reorder-widgets, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.to_index), ("note-beta", 1), "the inverse restores the widget's ORIGINAL index from base, never the payload's out-of-range 9");
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_flow_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` WITH a `warn`-level `mutation.no-op` — an out-of-range index
/// is clamped, never rejected as an invariant breach. (`🎯️outcome` spells the level `warn`;
/// `Severity` itself names that level `Warning`.)
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot declares an applied outcome");
    let mut snapshot = before();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: declared applied but the mutation was rejected");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its messages");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(declared.len(), messages.len(), "the declared message count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a clamped-to-current reorder is declared at warn level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "the emitted level for a clamped-to-current reorder is Warning");
    assert_eq!(messages[0].code.0, "mutation.no-op", "the emitted code is mutation.no-op — NOT the mutation.invariant an out-of-range index might suggest");
}

/// 🔺️ The sparse delta this clamp produces is exactly the committed diff — every field `null`,
/// `content` included: the proof that the guard returns before `diff_replace_content` mints a handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().content.is_none(), "a clamped-to-current reorder must not carry a content handle: {:?}", outcome.diff());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the flow artifact's own diff type —
/// `FlowDiff` carries `#[serde(default)]` with no per-field `skip_serializing_if`, so all eighteen
/// fields must be present and `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FlowDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(decoded, FlowDiff::default(), "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: the committed diff must be FlowDiff's Default");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — an empty
/// delta is still a complete description of "nothing moved".
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FlowDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-widgets/clamps-an-out-of-range-index-onto-the-last-slot: committed diff did not carry before to after");
}

/// 🔀️ `reorder-widgets` is ORDINAL, never spatial — it reorders `scene.widgets` and touches no
/// layout entry (that is `move-widgets`' job). The payload's index 9 is far past the end of a
/// 2-widget list; the clamp `to_index.min(len - 1)` is what turns it into 1, which is where
/// `note-beta` already sits.
#[semio_framework_async_macros::async_test]
async fn an_index_past_the_end_clamps_onto_the_widgets_last_slot() {
    let base = before();
    let scene = flow_working_scene(&base);
    assert_eq!(scene.widgets.len(), 2, "the clamp target `len - 1` is only meaningful against a known list length");
    assert!(scene.layout.is_empty(), "reorder-widgets is ordinal — this case deliberately carries no layout at all");
    let FlowMutation::ReorderWidgets(payload) = mutation() else {
        panic!("clamps-an-out-of-range-index-onto-the-last-slot's committed mutation must be a reorder-widgets");
    };
    assert_eq!(payload.id, "note-beta", "the payload addresses the widget that already occupies the clamped slot");
    assert!(payload.to_index > scene.widgets.len(), "the payload's index must genuinely be out of range, or this case would not exercise the clamp");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("reorder", "widget", "reorder-widgets", "ReorderedWidgets"),
        "the fixture must be bound to reorder-widgets' own descriptor — entity `widget`, unlike reorder-synapses' `synapse`"
    );
}

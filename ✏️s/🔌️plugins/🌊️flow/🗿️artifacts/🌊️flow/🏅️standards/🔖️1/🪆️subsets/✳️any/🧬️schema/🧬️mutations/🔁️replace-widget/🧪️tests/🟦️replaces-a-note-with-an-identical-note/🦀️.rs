//! 🧪️ `replace-widget` fixture — `🟦️replaces-a-note-with-an-identical-note`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✅️ This is an APPLIED case with an EMPTY diff. `replace-widget` is a WHOLE-VALUE swap — flow
//! widgets are heterogeneous enum variants, so its `🔺️diff` leaf compares the found widget against
//! the payload's with `PartialEq` and, on equality, returns
//! `MutationOutcome::empty().warn("mutation.no-op", …)` without reaching `diff_replace_content`.
//! Because no new content handle is minted, `➡️after` equals `⬅️before` and the committed diff is
//! `FlowDiff`'s all-`null` `Default` — the only honestly hand-authorable applied state for a flow
//! verb, since every state-changing flow diff addresses its composed `s.stdio.semio.flow` CHILD by a
//! domain-separated SHA-256 digest of the child content.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::Widget;
use flow::OrderedMap;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> FlowMutation {
    flow::os_pack::json::from_json_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FlowSnapshot {
    flow::os_pack::json::from_json_str(AFTER).expect("after snapshot decodes")
}

/// 🔁️ The committed `⬅️before`, with its composed child resolved to a scene holding exactly the
/// widget the committed payload offers as a "replacement" — the seeded widget IS the mutation JSON's
/// own `widget`, so the equality the no-op guard tests is structural, not asserted twice by hand.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = flow::os_pack::json::from_json_str(BEFORE).expect("before snapshot decodes");
    let FlowMutation::ReplaceWidget(payload) = mutation() else {
        panic!("replaces-a-note-with-an-identical-note's committed mutation must be a replace-widget");
    };
    cache_flow_content(&mut snapshot.content, vec![payload.widget.clone()], Vec::new(), OrderedMap::new());
    snapshot
}

/// ▶️ The identity replacement carries `before` to exactly the committed `after` — no widget value
/// changes, so no content handle is re-minted.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("replace-widget's no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-widget/replaces-a-note-with-an-identical-note: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "an identity replacement must leave the flow-content handle untouched");
}

/// ↩️ `replace-widget` inverts BASE-first: it captures the widget's PRIOR value out of the scene and
/// replaces it back. Here prior and requested values coincide, so the undo is itself a no-op and the
/// round trip lands exactly on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_flow_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "replace-widget always undoes with exactly one replace, got {inverse:?}");
    let FlowMutation::ReplaceWidget(undo) = &inverse[0] else {
        panic!("replace-widget's inverse must be a replace-widget, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "note-alpha", "the inverse addresses the same widget id");
    assert_eq!(undo.widget, flow_working_scene(&base).widgets[0], "the inverse carries the widget value read off BASE, never the payload's");
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_flow_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-widget/replaces-a-note-with-an-identical-note: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = flow::os_pack::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&decoded));
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-widget/replaces-a-note-with-an-identical-note: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&mutation()));
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-widget/replaces-a-note-with-an-identical-note: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` WITH a `warn`-level `mutation.no-op` — a value-equal
/// replacement is a warning on an empty diff, never a rejection. (`🎯️outcome` spells the level
/// `warn`; `Severity` itself names that level `Warning`.)
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-widget/replaces-a-note-with-an-identical-note declares an applied outcome");
    let mut snapshot = before();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("replace-widget/replaces-a-note-with-an-identical-note: declared applied but the mutation was rejected");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its messages");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(declared.len(), messages.len(), "the declared message count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a value-equal replacement is declared at warn level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "the emitted level for a value-equal replacement is Warning");
    assert_eq!(messages[0].code.0, "mutation.no-op", "the emitted code is mutation.no-op — NOT the target-missing this verb raises for an unknown id");
}

/// 🔺️ The sparse delta this identity replacement produces is exactly the committed diff — every
/// field `null`, `content` included: the proof that the equality guard returns before
/// `diff_replace_content` mints a handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().content.is_none(), "an identity replacement must not carry a content handle: {:?}", outcome.diff());
    let produced = serde_json::Value::from(dsl::ToValue::to_value(outcome.diff()));
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-widget/replaces-a-note-with-an-identical-note: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the flow artifact's own diff type —
/// `FlowDiff` carries `#[serde(default)]` with no per-field `skip_serializing_if`, so all eighteen
/// fields must be present and `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FlowDiff = flow::os_pack::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(decoded, FlowDiff::default(), "replace-widget/replaces-a-note-with-an-identical-note: the committed diff must be FlowDiff's Default");
    let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&decoded));
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-widget/replaces-a-note-with-an-identical-note: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — an empty
/// delta is still a complete description of "nothing was swapped".
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FlowDiff = flow::os_pack::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-widget/replaces-a-note-with-an-identical-note: committed diff did not carry before to after");
}

/// 🔁️ `replace-widget` swaps a WHOLE `Widget` value, so its no-op test is variant-and-field
/// equality, not a per-field patch comparison: the scene's `note-alpha` is an `InputNote` whose text
/// matches the payload's exactly. Change either the variant or the text and the guard would fall
/// through to `diff_replace_content`.
#[semio_framework_async_macros::async_test]
async fn the_guard_compares_the_whole_widget_value() {
    let base = before();
    let scene = flow_working_scene(&base);
    assert_eq!(scene.widgets.len(), 1, "this case's scene holds exactly the widget being replaced");
    assert_eq!(scene.widgets[0], Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, "the seeded widget must be the very InputNote the committed payload re-offers");
    let FlowMutation::ReplaceWidget(payload) = mutation() else {
        panic!("replaces-a-note-with-an-identical-note's committed mutation must be a replace-widget");
    };
    assert_eq!(payload.widget, scene.widgets[0], "payload and scene value must be equal — that equality IS the no-op guard");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("replace", "widget", "replace-widget", "ReplacedWidget"), "the fixture must be bound to replace-widget's own descriptor");
}

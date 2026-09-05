//! 🧪️ `move-widgets` fixture — `🟰️re-applies-the-current-layout-to-both-widgets`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✅️ This is an APPLIED case with an EMPTY diff. `move-widgets` is PLURAL by taxonomy design (one
//! op per real drag gesture), and its `🔺️diff` leaf's no-op guard is an `all`: every entry's
//! requested layout must already equal the scene's, and only then does it return
//! `MutationOutcome::empty().warn("mutation.no-op", …)` without reaching `diff_replace_content`.
//! Because no new content handle is minted, `➡️after` equals `⬅️before` and the committed diff is
//! `FlowDiff`'s all-`null` `Default` — the only honestly hand-authorable applied state for a flow
//! verb, since every state-changing flow diff addresses its composed `s.stdio.semio.flow` CHILD by a
//! domain-separated SHA-256 digest of the child content.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::{Widget, WidgetLayout};
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

/// 📍️ The committed `⬅️before`, with its composed child resolved to two widgets each already
/// carrying the layout the committed payload re-applies — the seeded layout entries ARE the mutation
/// JSON's own entries, so the equality the plural no-op guard tests is structural.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = flow::os_pack::json::from_json_str(BEFORE).expect("before snapshot decodes");
    let FlowMutation::MoveWidgets(payload) = mutation() else {
        panic!("re-applies-the-current-layout-to-both-widgets' committed mutation must be a move-widgets");
    };
    let widgets = vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, Widget::InputNote { id: "note-beta".into(), text: "Beta".into() }];
    let mut layout: OrderedMap<WidgetLayout> = OrderedMap::new();
    for entry in &payload.entries {
        layout.insert(entry.id.clone(), entry.layout.clone().expect("this case's committed entries all carry a layout"));
    }
    cache_flow_content(&mut snapshot.content, widgets, Vec::new(), layout);
    snapshot
}

/// ▶️ The re-applied layout carries `before` to exactly the committed `after` — neither widget
/// moves, so no content handle is re-minted.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("move-widgets' no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "move-widgets/re-applies-the-current-layout-to-both-widgets: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a layout re-application must leave the flow-content handle untouched");
}

/// ↩️ `move-widgets` inverts BASE-first and stays PLURAL: it folds every entry into ONE undo
/// `move-widgets` whose layouts are read off the scene. Here they coincide with the payload's, so
/// the undo is itself a no-op and the round trip lands exactly on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_flow_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "move-widgets folds its whole batch into exactly one undo step, never one per entry, got {inverse:?}");
    let FlowMutation::MoveWidgets(undo) = &inverse[0] else {
        panic!("move-widgets' inverse must be a move-widgets, got {:?}", inverse[0]);
    };
    let scene_layout = flow_working_scene(&base).layout;
    assert_eq!(undo.entries.len(), 2, "the undo must cover both moved widgets, got {:?}", undo.entries);
    for entry in &undo.entries {
        assert_eq!(entry.layout.as_ref(), scene_layout.get(&entry.id), "each undo entry carries the layout read off BASE for that id, never the payload's");
    }
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_flow_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "move-widgets/re-applies-the-current-layout-to-both-widgets: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = flow::os_pack::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&decoded));
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-widgets/re-applies-the-current-layout-to-both-widgets: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&mutation()));
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-widgets/re-applies-the-current-layout-to-both-widgets: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` WITH a `warn`-level `mutation.no-op` — a drag that ends
/// where it started is a warning on an empty diff, never a rejection. (`🎯️outcome` spells the level
/// `warn`; `Severity` itself names that level `Warning`.)
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-widgets/re-applies-the-current-layout-to-both-widgets declares an applied outcome");
    let mut snapshot = before();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("move-widgets/re-applies-the-current-layout-to-both-widgets: declared applied but the mutation was rejected");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its messages");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(declared.len(), messages.len(), "the declared message count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a layout re-application is declared at warn level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "the emitted level for a layout re-application is Warning — not the Fatal mutation.invariant a non-finite coordinate would raise");
    assert_eq!(messages[0].code.0, "mutation.no-op", "the emitted code is mutation.no-op");
}

/// 🔺️ The sparse delta this re-application produces is exactly the committed diff — every field
/// `null`, `content` included: the proof that the plural guard returns before `diff_replace_content`
/// mints a handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().content.is_none(), "a layout re-application must not carry a content handle: {:?}", outcome.diff());
    let produced = serde_json::Value::from(dsl::ToValue::to_value(outcome.diff()));
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-widgets/re-applies-the-current-layout-to-both-widgets: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the flow artifact's own diff type —
/// `FlowDiff` carries `#[serde(default)]` with no per-field `skip_serializing_if`, so all eighteen
/// fields must be present and `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FlowDiff = flow::os_pack::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(decoded, FlowDiff::default(), "move-widgets/re-applies-the-current-layout-to-both-widgets: the committed diff must be FlowDiff's Default");
    let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&decoded));
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-widgets/re-applies-the-current-layout-to-both-widgets: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — an empty
/// delta is still a complete description of "nothing moved".
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FlowDiff = flow::os_pack::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-widgets/re-applies-the-current-layout-to-both-widgets: committed diff did not carry before to after");
}

/// 📍️ `move-widgets` is SPATIAL and PLURAL: it writes `scene.layout`, never the widget ORDER (that
/// is `reorder-widgets`' job), it batches a whole drag gesture into one op, and its target list is
/// every entry id rather than a single one. This case carries TWO entries, so the `all`-shaped no-op
/// guard is genuinely exercised — one matching entry alone would not prove it.
#[semio_framework_async_macros::async_test]
async fn the_plural_guard_needs_every_entry_to_already_match() {
    let base = before();
    let scene = flow_working_scene(&base);
    let FlowMutation::MoveWidgets(payload) = mutation() else {
        panic!("re-applies-the-current-layout-to-both-widgets' committed mutation must be a move-widgets");
    };
    assert_eq!(payload.entries.len(), 2, "the committed payload must carry more than one entry, or the `all` guard is untested");
    for entry in &payload.entries {
        assert_eq!(scene.layout.get(&entry.id), entry.layout.as_ref(), "every entry's requested layout must already equal the scene's — that is the whole guard");
    }
    assert_eq!(scene.widgets.len(), 2, "both addressed widgets must exist, or a target-missing Error would fire first");
    assert_eq!(<FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::target(&mutation()), vec!["note-alpha".to_string(), "note-beta".to_string()], "move-widgets addresses a LIST of ids — the plural target every other flow verb reduces to one");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("move", "widgets", "move-widgets", "MovedWidgets"), "the fixture must be bound to move-widgets' own descriptor — the one entity in this vocabulary spelled PLURAL");
}

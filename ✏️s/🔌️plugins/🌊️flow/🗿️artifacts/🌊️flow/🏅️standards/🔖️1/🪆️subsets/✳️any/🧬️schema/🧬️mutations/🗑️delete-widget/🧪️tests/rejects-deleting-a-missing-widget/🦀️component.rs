//! 🧪️ `delete-widget` fixture — `rejects-deleting-a-missing-widget`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FlowSnapshot` persists its widgets/synapses/layout in
//! an opaque composed `s.stdio.semio.flow` CHILD (`🔖️ContentBridge`/`🔖️WorkingScene`), and every
//! APPLIED flow diff goes through `diff_replace_content`, which mints a fresh handle whose
//! `child_id` is a domain-separated SHA-256 digest of the child content. Hand-authoring such an `➡️after`
//! would mean hand-forging a value from `std`'s deliberately unspecified default hasher.
//! `delete-widget` has no no-op guard of its own, so its `mutation.target-missing` Error — the only
//! branch that mints no handle — is what this case pins.
//!
//! 🗑️ The scene is seeded with a POPULATED widget list (and a layout entry for it) so the rejection
//! is provably a lookup miss inside a non-empty `scene.widgets`, not the trivially empty scene an
//! unresolved handle would fail soft to.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::{Widget, WidgetLayout};
use flow::OrderedMap;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> FlowMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FlowSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🗑️ The committed `⬅️before`, with its composed child resolved to a scene holding one REAL widget
/// (`note-alpha`, with a layout entry) — deliberately not the `ghost-widget` the committed payload
/// addresses, so the lookup miss is a genuine miss inside a populated collection.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let mut layout = OrderedMap::new();
    layout.insert("note-alpha".to_string(), WidgetLayout { x: 12.0, y: 34.0 });
    cache_flow_content(&mut snapshot.content, vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }], Vec::new(), layout);
    snapshot
}

/// ▶️ A rejected `delete-widget` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-widget/rejects-deleting-a-missing-widget: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected delete-widget must not mint a new flow-content handle");
}

/// 🚫️ A missing widget is an Error-level `mutation.target-missing` — never the Fatal
/// `mutation.duplicate-id` its `create-widget` sibling raises. The guard runs BEFORE the cascade
/// scan, so the surviving widget `note-alpha` and its layout entry are left completely untouched
/// and no severed-synapse `mutation.cascade` Info is emitted.
#[semio_framework_async_macros::async_test]
async fn a_missing_widget_is_an_error_level_target_missing() {
    let base = before();
    let scene = flow_working_scene(&base);
    assert_eq!(scene.widgets.len(), 1, "rejects-deleting-a-missing-widget's seeded scene must hold exactly one real widget");
    assert!(scene.layout.contains_key("note-alpha"), "the seeded scene must carry the surviving widget's layout entry, so the untouched-cascade claim has teeth");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &FlowDiff::default(), "a rejecting delete-widget must carry an empty diff, never a partially-cascaded flow-content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected — no cascade Info may accompany a refused delete, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing widget is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "delete-widget's miss is Error, not the Fatal create-widget reserves for a duplicate identity");
    assert_eq!(messages[0].target, vec!["ghost-widget".to_string()], "the diagnostic names the WIDGET id that was asked for, not the one that survives");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "widget", "delete-widget", "DeletedWidget"), "the fixture must be bound to delete-widget's own descriptor");
}

/// ↩️ `delete-widget`'s inverse is BASE-derived — it rebuilds the widget, its layout entry and every
/// severed synapse out of the scene it found. With no such widget in base there is nothing to
/// reconstruct, so the inverse is empty (the exact opposite of `create-widget`'s payload-derived one).
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_widget_to_recreate() {
    let inverse = inverse_flow_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "delete-widget/rejects-deleting-a-missing-widget: a rejected delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-widget/rejects-deleting-a-missing-widget: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-widget/rejects-deleting-a-missing-widget: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-widget/rejects-deleting-a-missing-widget declares a rejected outcome");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

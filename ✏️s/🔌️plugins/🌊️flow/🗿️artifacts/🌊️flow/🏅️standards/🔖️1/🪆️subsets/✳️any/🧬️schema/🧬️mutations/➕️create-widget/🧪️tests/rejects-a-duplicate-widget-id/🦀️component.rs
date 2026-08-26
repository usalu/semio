//! 🧪️ `create-widget` fixture — `rejects-a-duplicate-widget-id`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FlowSnapshot` persists its widgets/synapses/layout in
//! an opaque composed `s.stdio.semio.flow` CHILD (`🔖️ContentBridge`/`🔖️WorkingScene`), and every
//! APPLIED flow diff goes through `diff_replace_content`, which mints a fresh handle whose
//! `child_id` is a `DefaultHasher` digest of the child content. Hand-authoring such an `➡️after`
//! would mean hand-forging a value from `std`'s deliberately unspecified default hasher.
//! `create-widget` is the one flow verb with NO no-op guard, so its `mutation.duplicate-id` Fatal —
//! the single branch that mints no handle at all — is what this tree pins until child resolution
//! lands.
//!
//! ➕️ Exactly as dag's `create-node` does, this case seeds the working-scene cache for the committed
//! content handle with the very widget the committed mutation payload carries: the id collision the
//! Fatal guards against. Nothing here is invented.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use std::collections::BTreeMap;

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

/// ➕️ The committed `⬅️before`, with its composed child resolved to a scene already holding the
/// widget the committed payload tries to create — the seeded widget IS the mutation JSON's own
/// `widget`, never a second hand-written copy of it.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let FlowMutation::CreateWidget(payload) = mutation() else {
        panic!("rejects-a-duplicate-widget-id's committed mutation must be a create-widget");
    };
    cache_flow_content(&mut snapshot.content, vec![payload.widget.clone()], Vec::new(), BTreeMap::new());
    snapshot
}

/// ▶️ A rejected `create-widget` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-widget/rejects-a-duplicate-widget-id: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected create-widget must not mint a new flow-content handle");
}

/// 🚨️ A colliding widget id is a FATAL `mutation.duplicate-id`, not the Error-level
/// `mutation.target-missing` the rest of this vocabulary reaches for: a duplicate widget identity is
/// an invariant breach, not a miss. `create-widget` searches `scene.widgets` — never `scene.synapses`
/// — and reports the id carried by the payload's own `widget`.
#[semio_framework_async_macros::async_test]
async fn a_colliding_widget_id_is_a_fatal_duplicate_id() {
    let base = before();
    assert_eq!(flow_working_scene(&base).widgets.len(), 1, "rejects-a-duplicate-widget-id's seeded scene must hold exactly the one colliding widget");
    assert!(flow_working_scene(&base).synapses.is_empty(), "create-widget's duplicate-id guard must be reachable with no synapse in the scene at all");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &FlowDiff::default(), "a rejecting create-widget must carry an empty diff, never a half-minted flow-content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "a widget id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb a duplicated widget identity");
    assert_eq!(messages[0].target, vec!["note-alpha".to_string()], "the diagnostic addresses the colliding WIDGET id");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "widget", "create-widget", "CreatedWidget"), "the fixture must be bound to create-widget's own descriptor");
}

/// ↩️ `create-widget`'s inverse is PAYLOAD-derived, not BASE-derived: it is a `delete-widget` of the
/// id it was asked to create, emitted even when the create itself was refused.
#[semio_framework_async_macros::async_test]
async fn inverse_is_always_a_delete_of_the_requested_widget_id() {
    let inverse = inverse_flow_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "create-widget always undoes with exactly one step, got {inverse:?}");
    let FlowMutation::DeleteWidget(undo) = &inverse[0] else {
        panic!("create-widget's inverse must be a delete-widget, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "note-alpha", "the inverse deletes exactly the widget id the payload carried");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-widget/rejects-a-duplicate-widget-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-widget/rejects-a-duplicate-widget-id: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-widget/rejects-a-duplicate-widget-id declares a rejected outcome");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

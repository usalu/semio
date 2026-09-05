//! 🧪️ `disconnect-widgets` fixture — `🚫️rejects-disconnecting-a-missing-synapse`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FlowSnapshot` persists its widgets/synapses/layout in
//! an opaque composed `s.stdio.semio.flow` CHILD (`🔖️ContentBridge`/`🔖️WorkingScene`), and every
//! APPLIED flow diff goes through `diff_replace_content`, which mints a fresh handle whose
//! `child_id` is a domain-separated SHA-256 digest of the child content. Hand-authoring such an `➡️after`
//! would mean hand-forging a value from `std`'s deliberately unspecified default hasher.
//! `disconnect-widgets` has no no-op guard, so its `mutation.target-missing` Error — the only branch
//! that mints no handle — is what this case pins.
//!
//! ✂️ The scene is seeded with two REAL widgets and NO synapse at all: `disconnect-widgets` is the
//! one flow verb addressed by SYNAPSE id, so a scene full of widgets must still reject, and the
//! diagnostic must name the synapse id rather than either endpoint widget.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::Widget;
use flow::OrderedMap;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> FlowMutation {
    flow::os_pack::json::from_json_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FlowSnapshot {
    flow::os_pack::json::from_json_str(AFTER).expect("after snapshot decodes")
}

/// ✂️ The committed `⬅️before`, with its composed child resolved to a scene of two connectable
/// widgets and an EMPTY synapse list — the state that makes "widgets present, edge absent" the only
/// possible reason for the rejection.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = flow::os_pack::json::from_json_str(BEFORE).expect("before snapshot decodes");
    let widgets = vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, Widget::InputNote { id: "note-beta".into(), text: "Beta".into() }];
    cache_flow_content(&mut snapshot.content, widgets, Vec::new(), OrderedMap::new());
    snapshot
}

/// ▶️ A rejected `disconnect-widgets` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "disconnect-widgets/rejects-disconnecting-a-missing-synapse: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a rejected disconnect-widgets must not mint a new flow-content handle");
}

/// ✂️ Despite its plural, widget-shaped name, `disconnect-widgets` is addressed by a single SYNAPSE
/// id: it searches `scene.synapses` only, so a scene holding both endpoint widgets still rejects,
/// and the reported target is the synapse id verbatim — never `note-alpha`/`note-beta`.
#[semio_framework_async_macros::async_test]
async fn a_missing_synapse_is_reported_by_its_synapse_id() {
    let base = before();
    let scene = flow_working_scene(&base);
    assert_eq!(scene.widgets.len(), 2, "rejects-disconnecting-a-missing-synapse's seeded scene must hold both endpoint widgets");
    assert!(scene.synapses.is_empty(), "the seeded scene must hold no synapse — that absence is what this case pins");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert_eq!(produced.diff(), &FlowDiff::default(), "a rejecting disconnect-widgets must carry an empty diff, never a re-minted flow-content handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing synapse is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "disconnect-widgets has no Fatal branch at all — a missing edge is always Error");
    assert_eq!(messages[0].target, vec!["synapse-ghost".to_string()], "the diagnostic names the SYNAPSE id, not either endpoint widget");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("disconnect", "synapse", "disconnect-widgets", "DisconnectedWidgets"),
        "the fixture must be bound to disconnect-widgets' own descriptor — entity `synapse`, despite the `widgets` in its kind"
    );
}

/// ↩️ `disconnect-widgets` inverts by reconstructing the removed edge out of BASE — its index, id,
/// both endpoint widgets and both ports. With no such synapse captured, the inverse is empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_synapse_to_reconnect() {
    let inverse = inverse_flow_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "disconnect-widgets/rejects-disconnecting-a-missing-synapse: a rejected disconnect must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = flow::os_pack::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&decoded));
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "disconnect-widgets/rejects-disconnecting-a-missing-synapse: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&mutation()));
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "disconnect-widgets/rejects-disconnecting-a-missing-synapse: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "disconnect-widgets/rejects-disconnecting-a-missing-synapse declares a rejected outcome");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

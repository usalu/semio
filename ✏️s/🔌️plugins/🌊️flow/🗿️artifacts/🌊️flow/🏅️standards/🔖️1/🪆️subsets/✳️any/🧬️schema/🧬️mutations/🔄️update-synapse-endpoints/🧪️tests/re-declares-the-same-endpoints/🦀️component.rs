//! 🧪️ `update-synapse-endpoints` fixture — `re-declares-the-same-endpoints`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✅️ This is an APPLIED case with an EMPTY diff. `update-synapse-endpoints` treats the four
//! endpoint fields as ONE inseparable facet, so its `🔺️diff` leaf compares all four at once and,
//! when every one already matches, returns `MutationOutcome::empty().warn("mutation.no-op", …)`
//! without reaching `diff_replace_content`. Because no new content handle is minted, `➡️after`
//! equals `⬅️before` and the committed diff is `FlowDiff`'s all-`null` `Default` — the only honestly
//! hand-authorable applied state for a flow verb, since every state-changing flow diff addresses its
//! composed `s.stdio.semio.flow` CHILD by a `DefaultHasher` digest of the child content.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::{SynapseSpec, Widget};
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

/// 🔄️ The committed `⬅️before`, with its composed child resolved to both endpoint widgets and the
/// synapse `synapse-1` already wired on exactly the four endpoint values the committed payload
/// re-declares.
fn before() -> FlowSnapshot {
    let snapshot: FlowSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let widgets = vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, Widget::InputNote { id: "note-beta".into(), text: "Beta".into() }];
    let synapses = vec![SynapseSpec { id: "synapse-1".into(), from: "note-alpha".into(), to: "note-beta".into(), from_port: "out".into(), to_port: "in".into() }];
    cache_flow_content(&snapshot.content.child_id, widgets, synapses, BTreeMap::new());
    snapshot
}

/// ▶️ The re-declaration carries `before` to exactly the committed `after` — no endpoint changes, so
/// no content handle is re-minted.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("update-synapse-endpoints' no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "update-synapse-endpoints/re-declares-the-same-endpoints: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "an endpoint re-declaration must leave the flow-content handle untouched");
}

/// ↩️ `update-synapse-endpoints` inverts BASE-first: it captures the synapse's PRIOR four endpoint
/// values out of the scene and re-declares those. Here prior and requested coincide, so the undo is
/// itself a no-op and the round trip lands exactly on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_flow_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "update-synapse-endpoints always undoes with exactly one update, got {inverse:?}");
    let FlowMutation::UpdateSynapseEndpoints(undo) = &inverse[0] else {
        panic!("update-synapse-endpoints' inverse must be an update-synapse-endpoints, got {:?}", inverse[0]);
    };
    let previous = &flow_working_scene(&base).synapses[0];
    assert_eq!(
        (undo.id.as_str(), undo.from.as_str(), undo.from_port.as_str(), undo.to.as_str(), undo.to_port.as_str()),
        (previous.id.as_str(), previous.from.as_str(), previous.from_port.as_str(), previous.to.as_str(), previous.to_port.as_str()),
        "the inverse carries all four endpoint values read off BASE — the whole facet, never a partial restore"
    );
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_flow_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-synapse-endpoints/re-declares-the-same-endpoints: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-synapse-endpoints/re-declares-the-same-endpoints: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-synapse-endpoints/re-declares-the-same-endpoints: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` WITH a `warn`-level `mutation.no-op` — re-declaring the
/// endpoints a synapse already has is a warning on an empty diff, never a rejection. (`🎯️outcome`
/// spells the level `warn`; `Severity` itself names that level `Warning`.)
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-synapse-endpoints/re-declares-the-same-endpoints declares an applied outcome");
    let mut snapshot = before();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("update-synapse-endpoints/re-declares-the-same-endpoints: declared applied but the mutation was rejected");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its messages");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(declared.len(), messages.len(), "the declared message count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "an endpoint re-declaration is declared at warn level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "the emitted level for an endpoint re-declaration is Warning");
    assert_eq!(messages[0].code.0, "mutation.no-op", "the emitted code is mutation.no-op");
}

/// 🔺️ The sparse delta this re-declaration produces is exactly the committed diff — every field
/// `null`, `content` included: the proof that the guard returns before `diff_replace_content` mints
/// a handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().content.is_none(), "an endpoint re-declaration must not carry a content handle: {:?}", outcome.diff());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-synapse-endpoints/re-declares-the-same-endpoints: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the flow artifact's own diff type —
/// `FlowDiff` carries `#[serde(default)]` with no per-field `skip_serializing_if`, so all eighteen
/// fields must be present and `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FlowDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(decoded, FlowDiff::default(), "update-synapse-endpoints/re-declares-the-same-endpoints: the committed diff must be FlowDiff's Default");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-synapse-endpoints/re-declares-the-same-endpoints: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — an empty
/// delta is still a complete description of "no endpoint moved".
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FlowDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-synapse-endpoints/re-declares-the-same-endpoints: committed diff did not carry before to after");
}

/// 🔄️ This verb is the only one that validates THREE ids before it looks at anything else: the
/// synapse it addresses, plus BOTH endpoint widgets. The scene therefore has to satisfy all three
/// lookups before the four-field equality guard is even reachable — and the guard compares the whole
/// `(from, from_port, to, to_port)` facet at once, never one field at a time.
#[semio_framework_async_macros::async_test]
async fn all_three_lookups_pass_before_the_four_field_equality_guard() {
    let base = before();
    let scene = flow_working_scene(&base);
    assert_eq!(scene.widgets.len(), 2, "both endpoint widgets must exist, or a target-missing Error would fire first");
    assert_eq!(scene.synapses.len(), 1, "the addressed synapse must exist, or a target-missing Error would fire first");
    let FlowMutation::UpdateSynapseEndpoints(payload) = mutation() else {
        panic!("re-declares-the-same-endpoints' committed mutation must be an update-synapse-endpoints");
    };
    let current = &scene.synapses[0];
    assert_eq!(payload.id, current.id, "the payload addresses the seeded synapse");
    assert_eq!(
        (payload.from.as_str(), payload.from_port.as_str(), payload.to.as_str(), payload.to_port.as_str()),
        (current.from.as_str(), current.from_port.as_str(), current.to.as_str(), current.to_port.as_str()),
        "all four endpoint fields must already match — that four-way equality IS the no-op guard"
    );
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("update", "synapse", "update-synapse-endpoints", "UpdatedSynapseEndpoints"),
        "the fixture must be bound to update-synapse-endpoints' own descriptor"
    );
}

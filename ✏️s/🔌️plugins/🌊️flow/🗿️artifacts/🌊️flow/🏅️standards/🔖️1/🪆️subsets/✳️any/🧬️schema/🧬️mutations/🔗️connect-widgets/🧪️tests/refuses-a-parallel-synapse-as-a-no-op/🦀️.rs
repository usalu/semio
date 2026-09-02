//! 🧪️ `connect-widgets` fixture — `refuses-a-parallel-synapse-as-a-no-op`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✅️ This is an APPLIED case with an EMPTY diff. `connect-widgets`' own `🔺️diff` leaf guards
//! parallel edges: when an identical `(from, from_port, to, to_port)` synapse already exists it
//! returns `MutationOutcome::empty().warn("mutation.no-op", …)` and never reaches
//! `diff_replace_content`. Because no new content handle is minted, `➡️after` equals `⬅️before` and
//! the committed diff is `FlowDiff`'s all-`null` `Default` — the only honestly hand-authorable
//! applied state for a flow verb, since every state-changing flow diff addresses its composed
//! `s.stdio.semio.flow` CHILD by a domain-separated SHA-256 digest of the child content.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::{SynapseSpec, Widget};
use flow::OrderedMap;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> FlowMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FlowSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 🔗️ The committed `⬅️before`, with its composed child resolved to two widgets already joined by
/// `synapse-1` on exactly the port pair the committed payload asks for again under a fresh id.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let widgets = vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, Widget::InputNote { id: "note-beta".into(), text: "Beta".into() }];
    let synapses = vec![SynapseSpec { id: "synapse-1".into(), from: "note-alpha".into(), to: "note-beta".into(), from_port: "out".into(), to_port: "in".into() }];
    cache_flow_content(&mut snapshot.content, widgets, synapses, OrderedMap::new());
    snapshot
}

/// ▶️ The refused parallel connect carries `before` to exactly the committed `after` — same handle,
/// same document.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("connect-widgets' no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a refused parallel connect must leave the flow-content handle untouched");
}

/// ↩️ `connect-widgets` inverts PAYLOAD-first — a `disconnect-widgets` of the id it was asked to
/// create, `synapse-2`, which was never created. That undo finds nothing to cut, so the round trip
/// still lands exactly on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_flow_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "connect-widgets always undoes with exactly one disconnect, got {inverse:?}");
    let FlowMutation::DisconnectWidgets(undo) = &inverse[0] else {
        panic!("connect-widgets' inverse must be a disconnect-widgets, got {:?}", inverse[0]);
    };
    assert_eq!(undo.id, "synapse-2", "the inverse cuts the id the payload asked for, not the pre-existing synapse-1");
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_flow_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` WITH a `warn`-level `mutation.no-op` — a refused parallel
/// edge is a warning on an empty diff, never a rejection. (`🎯️outcome` spells the level `warn`;
/// `Severity` itself names that level `Warning`.)
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "connect-widgets/refuses-a-parallel-synapse-as-a-no-op declares an applied outcome");
    let mut snapshot = before();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("connect-widgets/refuses-a-parallel-synapse-as-a-no-op: declared applied but the mutation was rejected");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its messages");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(declared.len(), messages.len(), "the declared message count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "a parallel-synapse refusal is declared at warn level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "the emitted level for a parallel-synapse refusal is Warning");
    assert_eq!(messages[0].code.0, "mutation.no-op", "the emitted code is mutation.no-op");
    assert!(messages[0].target.is_empty(), "connect-widgets' no-op warning is untargeted — unlike its duplicate-id Fatal and its two target-missing Errors");
}

/// 🔺️ The sparse delta this refusal produces is exactly the committed diff — for
/// `connect-widgets`' parallel-edge guard that means EVERY field is `null`, including `content`:
/// the proof that the guard returns before `diff_replace_content` ever mints a handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().content.is_none(), "a refused parallel connect must not carry a content handle: {:?}", outcome.diff());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the flow artifact's own diff type —
/// `FlowDiff` carries `#[serde(default)]` with no per-field `skip_serializing_if`, so all eighteen
/// fields must be present and `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FlowDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(decoded, FlowDiff::default(), "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: the committed diff must be FlowDiff's Default");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — an empty
/// delta is still a complete description of "nothing changed".
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FlowDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "connect-widgets/refuses-a-parallel-synapse-as-a-no-op: committed diff did not carry before to after");
}

/// 🔗️ The guard order this case walks: the payload's `synapse-2` is a FRESH id (so the duplicate-id
/// Fatal is skipped) and both endpoint widgets exist (so neither target-missing Error fires) — only
/// then does the parallel-edge check reach the pre-existing `synapse-1` on the same port pair.
#[semio_framework_async_macros::async_test]
async fn the_scene_reaches_the_parallel_guard_past_every_earlier_rejection() {
    let base = before();
    let scene = flow_working_scene(&base);
    assert_eq!(scene.widgets.len(), 2, "both endpoint widgets must exist, or a target-missing Error would fire first");
    assert_eq!(scene.synapses.len(), 1, "exactly one pre-existing synapse must occupy the port pair");
    assert_eq!(scene.synapses[0].id, "synapse-1", "the occupying synapse carries a DIFFERENT id from the payload's, or duplicate-id would fire first");
    let FlowMutation::ConnectWidgets(payload) = mutation() else {
        panic!("refuses-a-parallel-synapse-as-a-no-op's committed mutation must be a connect-widgets");
    };
    assert_eq!((payload.from.as_str(), payload.from_port.as_str(), payload.to.as_str(), payload.to_port.as_str()), ("note-alpha", "out", "note-beta", "in"), "the payload must re-request the exact port pair synapse-1 already occupies");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("connect", "synapse", "connect-widgets", "ConnectedWidgets"),
        "the fixture must be bound to connect-widgets' own descriptor — entity `synapse`, despite the `widgets` in its kind"
    );
}

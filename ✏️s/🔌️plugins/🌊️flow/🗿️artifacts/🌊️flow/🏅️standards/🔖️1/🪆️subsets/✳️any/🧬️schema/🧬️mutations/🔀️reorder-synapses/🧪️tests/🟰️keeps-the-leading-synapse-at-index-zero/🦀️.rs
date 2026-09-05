//! 🧪️ `reorder-synapses` fixture — `🟰️keeps-the-leading-synapse-at-index-zero`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`.
//!
//! ✅️ This is an APPLIED case with an EMPTY diff. `reorder-synapses`' own `🔺️diff` leaf returns
//! `MutationOutcome::empty().warn("mutation.no-op", …)` the moment the resolved destination equals
//! the synapse's current index, never reaching `diff_replace_content`. Because no new content handle
//! is minted, `➡️after` equals `⬅️before` and the committed diff is `FlowDiff`'s all-`null`
//! `Default` — the only honestly hand-authorable applied state for a flow verb, since every
//! state-changing flow diff addresses its composed `s.stdio.semio.flow` CHILD by a domain-separated SHA-256
//! digest of the child content.

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
    flow::os_pack::json::from_json_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FlowSnapshot {
    flow::os_pack::json::from_json_str(AFTER).expect("after snapshot decodes")
}

/// 🔀️ The committed `⬅️before`, with its composed child resolved to two widgets wired by an ORDERED
/// pair of synapses; `synapse-1` leads at index 0, which is exactly where the committed payload asks
/// to put it.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = flow::os_pack::json::from_json_str(BEFORE).expect("before snapshot decodes");
    let widgets = vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, Widget::InputNote { id: "note-beta".into(), text: "Beta".into() }];
    let synapses = vec![
        SynapseSpec { id: "synapse-1".into(), from: "note-alpha".into(), to: "note-beta".into(), from_port: "out".into(), to_port: "in".into() },
        SynapseSpec { id: "synapse-2".into(), from: "note-beta".into(), to: "note-alpha".into(), from_port: "out".into(), to_port: "in".into() },
    ];
    cache_flow_content(&mut snapshot.content, widgets, synapses, OrderedMap::new());
    snapshot
}

/// ▶️ The already-satisfied reorder carries `before` to exactly the committed `after` — the synapse
/// order is left alone, and with it the content handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("reorder-synapses' no-op diff applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "reorder-synapses/keeps-the-leading-synapse-at-index-zero: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "an already-satisfied reorder must leave the flow-content handle untouched");
    assert_eq!(flow_working_scene(&snapshot).synapses.iter().map(|synapse| synapse.id.clone()).collect::<Vec<_>>(), vec!["synapse-1".to_string(), "synapse-2".to_string()], "the synapse ORDER is what this verb owns, and it must be unchanged");
}

/// ↩️ `reorder-synapses` inverts BASE-first: it reads the synapse's ORIGINAL index out of the scene
/// and asks for that index back — 0 here, so the undo is itself a no-op and the round trip lands
/// exactly on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_flow_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "reorder-synapses always undoes with exactly one reorder, got {inverse:?}");
    let FlowMutation::ReorderSynapses(undo) = &inverse[0] else {
        panic!("reorder-synapses' inverse must be a reorder-synapses, got {:?}", inverse[0]);
    };
    assert_eq!((undo.id.as_str(), undo.to_index), ("synapse-1", 0), "the inverse restores the synapse's ORIGINAL index read off base");
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_flow_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-synapses/keeps-the-leading-synapse-at-index-zero: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = flow::os_pack::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&decoded));
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-synapses/keeps-the-leading-synapse-at-index-zero: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&mutation()));
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-synapses/keeps-the-leading-synapse-at-index-zero: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome is `applied` WITH a `warn`-level `mutation.no-op` — asking a synapse to
/// stay where it already is is a warning on an empty diff, never a rejection. (`🎯️outcome` spells
/// the level `warn`; `Severity` itself names that level `Warning`.)
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-synapses/keeps-the-leading-synapse-at-index-zero declares an applied outcome");
    let mut snapshot = before();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("reorder-synapses/keeps-the-leading-synapse-at-index-zero: declared applied but the mutation was rejected");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("a no-op outcome declares its messages");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(declared.len(), messages.len(), "the declared message count must match the emitted one, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(serde_json::Value::as_str), Some("warn"), "an already-satisfied synapse reorder is declared at warn level");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "the emitted level for an already-satisfied synapse reorder is Warning");
    assert_eq!(messages[0].code.0, "mutation.no-op", "the emitted code is mutation.no-op");
}

/// 🔺️ The sparse delta this no-op produces is exactly the committed diff — every field `null`,
/// `content` included: the proof that the guard returns before `diff_replace_content` mints a handle.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().content.is_none(), "an already-satisfied synapse reorder must not carry a content handle: {:?}", outcome.diff());
    let produced = serde_json::Value::from(dsl::ToValue::to_value(outcome.diff()));
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-synapses/keeps-the-leading-synapse-at-index-zero: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the flow artifact's own diff type —
/// `FlowDiff` carries `#[serde(default)]` with no per-field `skip_serializing_if`, so all eighteen
/// fields must be present and `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: FlowDiff = flow::os_pack::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(decoded, FlowDiff::default(), "reorder-synapses/keeps-the-leading-synapse-at-index-zero: the committed diff must be FlowDiff's Default");
    let reencoded = serde_json::Value::from(dsl::ToValue::to_value(&decoded));
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-synapses/keeps-the-leading-synapse-at-index-zero: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — an empty
/// delta is still a complete description of "nothing moved".
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: FlowDiff = flow::os_pack::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-synapses/keeps-the-leading-synapse-at-index-zero: committed diff did not carry before to after");
}

/// 🔀️ `reorder-synapses` is `reorder-widgets`' twin over the OTHER collection: it locates its target
/// by scanning `scene.synapses`, so a widget list of the same length is irrelevant to it, and its
/// descriptor's entity is `synapse`. This case pins a plain already-at-position no-op — no clamping
/// is involved, index 0 is genuinely in range for a 2-synapse list.
#[semio_framework_async_macros::async_test]
async fn the_target_is_found_by_scanning_the_synapse_list() {
    let base = before();
    let scene = flow_working_scene(&base);
    assert_eq!(scene.synapses.len(), 2, "the scene must hold more than one synapse, or 'already at index 0' would be vacuous");
    assert_eq!(scene.synapses[0].id, "synapse-1", "the payload's target must genuinely occupy index 0 of the SYNAPSE list");
    let FlowMutation::ReorderSynapses(payload) = mutation() else {
        panic!("keeps-the-leading-synapse-at-index-zero's committed mutation must be a reorder-synapses");
    };
    assert!(payload.to_index < scene.synapses.len(), "the requested index must be genuinely in range — this case is about equality, not the clamp");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("reorder", "synapse", "reorder-synapses", "ReorderedSynapses"),
        "the fixture must be bound to reorder-synapses' own descriptor — entity `synapse`, unlike reorder-widgets' `widget`"
    );
}

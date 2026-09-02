//! 🧪️ `duplicate-widget` fixture — `rejects-duplicating-onto-a-taken-id`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `FlowSnapshot` persists its widgets/synapses/layout in
//! an opaque composed `s.stdio.semio.flow` CHILD (`🔖️ContentBridge`/`🔖️WorkingScene`), and a
//! successful `duplicate-widget` folds its plan's `create-widget` + `connect-widgets` steps into a
//! `content` handle whose `child_id` is a domain-separated SHA-256 digest of the child content. Hand-authoring
//! such an `➡️after` would mean hand-forging a value from `std`'s deliberately unspecified default
//! hasher. A refused PLAN mints nothing at all, so that is what this case pins.
//!
//! 👯️ `duplicate-widget` is this vocabulary's only COMPOSITE: it owns no `🔺️diff`/`↩️inverse` leaf,
//! both fold from `🧩️plan` through `protocol::fold_plan_diff`/`fold_plan_inverse`. The scene is
//! seeded with BOTH `note-alpha` (a valid source) and `note-beta` (the id the payload wants for the
//! copy), so planning gets past the "source missing" precondition and dies on the third one —
//! `new_id` already taken — the branch a composite folds into a Fatal `mutation.invariant`.

use crate::artifacts::flow::schema::mutations::{apply_flow_mutation, inverse_flow_mutation, to_framework_mutation, FlowMutation};
use crate::artifacts::flow::{cache_flow_content, flow_working_scene, FlowDiff, FlowSnapshot};
use flow::Widget;
use flow::OrderedMap;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> FlowMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn expected_after() -> FlowSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// 👯️ The committed `⬅️before`, with its composed child resolved to a scene holding the payload's
/// `source_id` widget AND a second widget already occupying its `new_id` — the collision
/// `duplicate-widget`'s own `precondition` refuses.
fn before() -> FlowSnapshot {
    let mut snapshot: FlowSnapshot = serde_json::from_str(BEFORE).expect("before snapshot decodes");
    let widgets = vec![Widget::InputNote { id: "note-alpha".into(), text: "Alpha".into() }, Widget::InputNote { id: "note-beta".into(), text: "Beta".into() }];
    cache_flow_content(&mut snapshot.content, widgets, Vec::new(), OrderedMap::new());
    snapshot
}

/// ▶️ A refused `duplicate-widget` plan leaves the document byte-identical to the committed `after`
/// — neither of its two planned steps (`create-widget`, then `connect-widgets`) reaches the scene.
#[semio_framework_async_macros::async_test]
async fn a_refused_plan_leaves_the_document_at_the_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_flow_mutation(&mut snapshot, &mutation()).expect("an empty folded diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "duplicate-widget/rejects-duplicating-onto-a-taken-id: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.content, base.content, "a refused duplicate-widget plan must not mint a new flow-content handle");
    assert_eq!(flow_working_scene(&base).widgets.len(), 2, "the seeded scene must still hold exactly the source widget and the id-squatting widget");
}

/// 🚨️ A composite reports a planning refusal as ONE Fatal `mutation.invariant` with an EMPTY target
/// — `fold_plan_diff` stamps `PlanError`'s own text and never a per-entity address, which is what
/// makes this verb's diagnostic shape unlike every id-addressed leaf verb in this vocabulary. The
/// message text pins the third precondition branch (`new_id` already taken), not the first two.
#[semio_framework_async_macros::async_test]
async fn a_taken_new_id_folds_into_a_fatal_untargeted_invariant() {
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &FlowDiff::default(), "an all-or-nothing composite refusal must fold to an empty diff, never a half-planned create");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected — the plan dies before its first step is recorded, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.invariant", "a PlanError folds into mutation.invariant, not the duplicate-id a bare create-widget would raise");
    assert_eq!(messages[0].level, protocol::Severity::Fatal, "a refused plan is Fatal — no merge policy may absorb a half-applied composite");
    assert!(messages[0].target.is_empty(), "fold_plan_diff never addresses a PlanError to an entity, got {:?}", messages[0].target);
    assert_eq!(messages[0].message, "duplicate-widget: id \"note-beta\" already taken", "the refusal must come from duplicate-widget's own new_id-taken precondition");
    let semantics = <FlowMutation as protocol::SemanticMutation<FlowSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("duplicate", "widget", "duplicate-widget", "DuplicatedWidget"), "the fixture must be bound to duplicate-widget's own descriptor");
}

/// ↩️ A composite's inverse is the reversed per-step inverse of its PLAN, so a plan that never
/// produced a step has no inverse at all. `duplicate-widget` is also the one flow verb with no
/// framework-generic counterpart — it plans two ops, so `to_framework_mutation` returns `None`.
#[semio_framework_async_macros::async_test]
async fn a_refused_plan_has_neither_inverse_steps_nor_a_framework_counterpart() {
    let inverse = inverse_flow_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "duplicate-widget/rejects-duplicating-onto-a-taken-id: a refused plan must have no inverse steps, got {inverse:?}");
    assert!(to_framework_mutation(&mutation()).is_none(), "duplicate-widget is the only flow verb without a framework-generic op — it plans a create AND a connect");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point. Note that
/// `DuplicateWidget` carries NO `#[serde(rename_all)]`, so its payload keys stay snake_case
/// (`source_id`/`new_id`/`synapse_id`/`from_port`/`to_port`) — unlike every leaf verb's camelCase.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: FlowSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "duplicate-widget/rejects-duplicating-onto-a-taken-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "duplicate-widget/rejects-duplicating-onto-a-taken-id: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and (empty) path — is exactly what the fold emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "duplicate-widget/rejects-duplicating-onto-a-taken-id declares a rejected outcome");
    let produced = <FlowMutation as protocol::Mutation<FlowSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target — both empty, because a composite refusal has no entity address");
}

//! 🧪️ `change-node-label` fixture — `rejects-relabelling-a-node-that-is-not-in-the-graph`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `MathematicalSnapshot` keeps its graph and its point
//! cloud in three co-derived composed CHILDREN (`notation`/`results`/`computed`,
//! `🔖️WorkingScene`), and every APPLIED mathematical diff re-mints all three through
//! `mathematical_children_from_state`, whose `child_id` is a `DefaultHasher` digest of the child
//! content. Hand-authoring such an `➡️after` would mean hand-forging a value from `std`'s
//! deliberately unspecified default hasher. A committed snapshot therefore decodes to an
//! UNRESOLVED handle and `mathematical_scene` fails soft to a node-less graph — the state this
//! case pins, and the state in which `change-node-label`'s own `mutation.target-missing` fires.

use crate::artifacts::mathematical::{mathematical_graph, MathematicalDiff, MathematicalMutation, MathematicalSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> MathematicalSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> MathematicalSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> MathematicalMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<MathematicalDiff> {
    <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `change-node-label` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(mathematical_graph(&base).nodes.is_empty(), "rejects-relabelling-a-node-that-is-not-in-the-graph's before-snapshot must decode to an unresolved, node-less graph");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected relabel must not mint a fresh notation/results/computed triple");
}

/// 🏷️ `id` is this node's stable identity and is NEVER rewritten — which is exactly why the verb is
/// `change-node-label` and not `rename-node`. The diagnostic therefore addresses the untouched
/// `id`, and the requested `new_label` appears nowhere in it. The already-has-that-label
/// `mutation.no-op` guard sits BEHIND this lookup, so an absent node can never reach it.
#[semio_framework_async_macros::async_test]
async fn the_diagnostic_addresses_the_stable_id_never_the_new_label() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &MathematicalDiff::default(), "a rejecting change-node-label must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing, never as a no-op");
    assert_eq!(messages[0].level, protocol::Severity::Error, "change-node-label has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string()], "the diagnostic names the stable node id");
    assert!(!messages[0].target.contains(&"Alpha".to_string()), "the requested label must never leak into the diagnostic target");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "node", "change-node-label", "ChangedNodeLabel"), "the fixture must be bound to change-node-label's own descriptor — `change`, never `rename`");
}

/// ↩️ The undo carries BASE's own label for that id. With no such node the inverse is empty, so a
/// refused relabel can never be replayed as an accidental label wipe.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_prior_label_to_restore() {
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: a rejected relabel must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload field stays
/// snake_case (`new_label`) because `MathematicalMutation`'s payload structs carry no
/// `#[serde(rename_all)]` — unlike the nested document types, which do.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&(mutation()).to_value());
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/ChangeNodeLabel/new_label").and_then(pack::JsonValue::as_str), Some("Alpha"), "the payload's label field commits snake_case");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::JsonValue::as_str), Some("rejected"), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(pack::JsonValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(pack::JsonValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

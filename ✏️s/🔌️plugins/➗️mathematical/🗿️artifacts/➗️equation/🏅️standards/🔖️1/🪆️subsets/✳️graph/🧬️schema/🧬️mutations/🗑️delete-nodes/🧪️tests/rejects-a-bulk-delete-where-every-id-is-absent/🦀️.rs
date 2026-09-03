//! 🧪️ `delete-nodes` fixture — `rejects-a-bulk-delete-where-every-id-is-absent`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why this leaf pins a REJECTION branch: `EquationSnapshot` keeps its graph and its point
//! cloud in three co-derived composed CHILDREN (`notation`/`results`/`computed`,
//! `🔖️WorkingScene`), and every APPLIED equation diff re-mints all three through
//! `equation_children_from_state`, whose `child_id` is a `DefaultHasher` digest of the child
//! content. Hand-authoring such an `➡️after` would mean hand-forging a value from `std`'s
//! deliberately unspecified default hasher. A committed snapshot therefore decodes to an
//! UNRESOLVED handle and `equation_scene` fails soft to an empty graph — the state this case
//! pins, and the state in which `delete-nodes`' all-missing `mutation.target-missing` fires.

use crate::artifacts::equation::{equation_graph, EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> EquationSnapshot {
    pack::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    pack::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `delete-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(equation_graph(&base).nodes.is_empty(), "rejects-a-bulk-delete-where-every-id-is-absent's before-snapshot must decode to an unresolved, node-less graph");
    let applied = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected bulk delete must not mint a fresh notation/results/computed triple");
}

/// 🗑️ The PLURAL verb is all-or-nothing at its entry gate: only when NOT ONE requested id exists
/// does it refuse, and then it reports EVERY requested id in a single diagnostic — not one per id,
/// and not just the first. The partial-hit `mutation.partial` warning is unreachable here, because
/// that branch lives past the all-missing short circuit.
#[semio_framework_async_macros::async_test]
async fn every_requested_id_travels_in_one_all_missing_diagnostic() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting delete-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "the bulk refusal is a single diagnostic, never one per id, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "an all-missing bulk delete is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "delete-nodes has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string(), "n-beta".to_string()], "the diagnostic carries the WHOLE requested id list, in payload order");
    assert!(messages.iter().all(|message| message.code.0 != "mutation.partial" && message.code.0 != "mutation.cascade"), "neither the partial warning nor the cascade notice is reachable when nothing existed, got {messages:?}");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("delete", "nodes", "delete-nodes", "DeletedNodes"),
        "the fixture must be bound to delete-nodes' own PLURAL descriptor — the sibling singular verb is `delete-node`/`DeletedNode`"
    );
}

/// ↩️ `delete-nodes` inverts by re-creating every deleted node then re-`connect`ing every severed
/// edge, all captured from BASE. With no matching node the inverse is empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_nodes_to_recreate() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before());
    assert!(inverse.is_empty(), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: a rejected bulk delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The plural payload's one
/// field is `ids` — a bare `Vec<String>`, never a nested selection object.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&(mutation()).to_value());
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/DeleteNodes/ids").and_then(pack::JsonValue::as_array).map(|ids| ids.len()), Some(2), "the committed payload must request two ids for the bulk branch to be meaningful");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::JsonValue::as_str), Some("rejected"), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(pack::JsonValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(pack::JsonValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target — for the plural verb that is the whole id list");
}

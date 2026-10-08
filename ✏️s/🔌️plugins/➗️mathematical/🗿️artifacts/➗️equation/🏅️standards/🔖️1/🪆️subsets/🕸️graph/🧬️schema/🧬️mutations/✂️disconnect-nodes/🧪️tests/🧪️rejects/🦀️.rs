//! 🧪️ `disconnect-nodes` fixture — `🚫️rejects-severing-an-edge-that-is-not-in-the-graph`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE as parent-owned
//! fields, every leaf decides from them, and the `notation`/`results`/`computed` handles are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law. This vector pins the outcome its
//! `🎯️outcome` declares on that committed state.

use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/🎯️outcome/🔣️.json");

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `disconnect-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.edges.is_empty(), "rejects-severing-an-edge-that-is-not-in-the-graph's before-snapshot must hold a edge-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected disconnect must not mint a fresh notation/results/computed triple");
}

/// ✂️ `disconnect-nodes` is this vocabulary's only verb addressed by EDGE id: it searches
/// `graph.edges`, never `graph.nodes`, so the reported target is the edge id verbatim and neither
/// endpoint is ever named. It also has no duplicate/invariant branch — one Error and nothing else.
#[semio_framework_async_macros::async_test]
async fn a_missing_edge_is_reported_by_its_edge_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting disconnect-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing edge is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "disconnect-nodes has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["e-alpha-beta".to_string()], "the diagnostic names the EDGE id, not either endpoint node");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("disconnect", "node", "disconnect-nodes", "DisconnectedNodes"), "the fixture must be bound to disconnect-nodes' own descriptor");
}

/// ↩️ `disconnect-nodes` inverts by reconstructing the exact edge BASE showed — id plus both
/// endpoints. With no such edge captured the inverse is empty, the mirror image of its
/// `connect-nodes` twin, whose inverse is payload-derived and therefore never empty.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_edge_to_reconnect() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: a rejected disconnect must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload is a single
/// `id` — the edge's — with no source/target pair, which is what makes this verb edge-addressed.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/DisconnectNodes").and_then(semio_framework_pack_json::Value::as_object).map(|fields| fields.len()), Some(1), "the payload carries the edge id and nothing else");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(semio_framework_pack_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(semio_framework_pack_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

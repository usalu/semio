//! 🧪️ `move-node` fixture — `🚫️rejects-moving-a-node-that-is-not-in-the-graph`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE as parent-owned
//! fields, every leaf decides from them, and the `notation`/`results`/`computed` handles are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law. This vector pins the outcome its
//! `🎯️outcome` declares on that committed state.

use crate::standards::v1::subsets::graph::schema::mutations::move_node::MoveNode;
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/🎯️outcome/🔣️.json");

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

/// ▶️ A rejected `move-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-moving-a-node-that-is-not-in-the-graph's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected move must not mint a fresh notation/results/computed triple");
}

/// 🕹️ `move-node` is ID-keyed where its geometry twin `move-point` is index-keyed, and it stacks
/// three guards in a fixed order: id lookup, then the finiteness invariant, then the
/// already-there no-op. An absent id short-circuits all of them, so even a NaN destination is
/// refused as the Error `mutation.target-missing` rather than the Fatal `mutation.invariant`.
#[semio_framework_async_macros::async_test]
async fn the_id_lookup_precedes_the_finiteness_invariant() {
    let base = before();
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting move-node must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "a missing node is an Error, not the Fatal reserved for a non-finite position");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string()], "the diagnostic names the node id, never its requested coordinates");

    let non_finite = EquationMutation::MoveNode(MoveNode { id: "n-alpha".to_string(), x: f64::NAN, y: 0.0 });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&non_finite, &base);
    assert_eq!(outcome.messages()[0].code.0, "mutation.target-missing", "with no node under that id, even a NaN destination is refused as target-missing — the lookup guard wins");

    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("move", "node", "move-node", "MovedNode"), "the fixture must be bound to move-node's own descriptor");
}

/// ↩️ `move-node` inverts to another `move-node` carrying the coordinates BASE held for that id.
/// With no such node the inverse is empty — base-derived, never payload-derived.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_prior_position_to_restore() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: a rejected move must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The destination is
/// ABSOLUTE, not a delta — two `f64`s that always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/MoveNode/x").and_then(semio_framework_pack_json::Value::as_f64), Some(120.0), "the payload commits an absolute destination, not an offset");
    assert_eq!(BEFORE, AFTER, "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "move-node/rejects-moving-a-node-that-is-not-in-the-graph declares a rejected outcome");
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

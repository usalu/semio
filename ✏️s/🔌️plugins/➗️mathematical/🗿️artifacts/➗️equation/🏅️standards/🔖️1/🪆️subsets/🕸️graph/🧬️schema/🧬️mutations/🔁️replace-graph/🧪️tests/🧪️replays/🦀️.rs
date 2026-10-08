//! 🧪️ `replace-graph` fixture — `🕸️replays-the-identical-empty-graph`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE as parent-owned
//! fields, every leaf decides from them, and the `notation`/`results`/`computed` handles are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law. This vector pins the outcome its
//! `🎯️outcome` declares on that committed state.

use crate::standards::v1::subsets::graph::schema::mutations::replace_graph::ReplaceGraph;
use crate::{EquationDiff, EquationGraph, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-graph/🧪️replays/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-graph/🧪️replays/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-graph/🧪️replays/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-graph/🧪️replays/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔁️replace-graph/🧪️replays/🎯️outcome/🔣️.json");

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

/// ▶️ `replace-graph` compares the WHOLE payload graph against base — direction, node list, edge
/// list, algorithm and seed at once — so replaying an identical graph carries `before` to exactly
/// the committed `after`, i.e. leaves it untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let EquationMutation::ReplaceGraph(payload) = mutation() else {
        panic!("replays-the-identical-empty-graph's committed mutation must be a replace-graph");
    };
    assert_eq!(base.graph, payload.graph, "the committed payload must be byte-for-byte the graph BASE resolves to, or the no-op guard is never reached");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "replace-graph/replays-the-identical-empty-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a no-op replace-graph must not mint a fresh notation/results/computed triple");
}

/// ↩️ `replace-graph` inverts to another `replace-graph` carrying BASE's whole prior graph — it is
/// base-derived, never payload-derived.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![EquationMutation::ReplaceGraph(ReplaceGraph { graph: base.graph.clone() })], "replace-graph inverts to a replace-graph carrying BASE's whole prior graph, got {inverse:?}");
    let mut snapshot = protocol::apply_diff(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-graph/replays-the-identical-empty-graph: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload and the nested
/// `EquationGraph` both carry `#[value(rename_all = "camelCase")]`, so the seed slot is `algorithmSeed`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "replace-graph/replays-the-identical-empty-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "replace-graph/replays-the-identical-empty-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert!(original.pointer("/ReplaceGraph/graph/algorithmSeed").is_some(), "the nested graph renames algorithm_seed to algorithmSeed");
}

/// 🎯️ The declared outcome — `no-op`, with one `mutation.no-op` warning — is what the builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("no-op"), "replace-graph/replays-the-identical-empty-graph declares a no-op outcome");
    let emitted = produced();
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "replaying an identical graph is reported as no-op");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning, "a no-op is a Warning — never a refusal, it just changes nothing");
    let declared = outcome.get("messages").and_then(semio_framework_pack_json::Value::as_array).expect("the declared outcome carries its warning");
    assert_eq!(declared[0].get("code").and_then(semio_framework_pack_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
}

/// 🔺️ A no-op emits the artifact's `Default` diff — all six artifact slots `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &EquationDiff::default(), "a no-op replace-graph must carry the empty diff, never a re-minted child triple");
    let produced_value = semio_framework_pack_json::from_dsl_value(&(outcome.diff()).to_value());
    let committed = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&produced_value, &committed), "replace-graph/replays-the-identical-empty-graph: produced diff differs from the committed 🔺️diff/🔣️.json ({produced_value:?} vs {committed:?})");
}

/// 🔣️ The committed diff is canonical and decodes to `EquationDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    let original = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "replace-graph/replays-the-identical-empty-graph: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced_snapshot = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "replace-graph/replays-the-identical-empty-graph: committed diff did not carry before to after");
}

/// 🔁️ The guard is a WHOLE-VALUE comparison, not a field sample: a payload that differs in the
/// single `algorithm` field is a real replacement and re-derives the whole child triple.
#[semio_framework_async_macros::async_test]
async fn a_graph_differing_in_one_field_is_a_real_replacement() {
    let base = before();
    let retitled = EquationMutation::ReplaceGraph(ReplaceGraph { graph: EquationGraph { algorithm: "bfs".into(), ..base.graph.clone() } });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&retitled, &base);
    assert!(outcome.messages().is_empty(), "a graph that differs anywhere is a real replacement, not a no-op, got {:?}", outcome.messages());
    assert!(outcome.diff().notation.is_some() && outcome.diff().results.is_some() && outcome.diff().computed.is_some(), "replace-graph regenerates all three co-derived children at once");
    assert!(outcome.diff().equation.is_none(), "replace-graph never touches the inline equation slot");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("replace", "graph", "replace-graph", "ReplacedGraph"), "the fixture must be bound to replace-graph's own descriptor");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

//! 🧪️ `update-graph-algorithm` fixture — `🧮️restates-the-unset-algorithm-and-its-absent-seed`.
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

use crate::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::UpdateGraphAlgorithm;
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/🎯️outcome/🔣️.json");

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

/// ▶️ Restating both inseparable facets — algorithm AND seed — carries `before` to exactly the
/// committed `after`, i.e. leaves it untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let graph = base.graph.clone();
    assert_eq!((graph.algorithm.as_str(), graph.algorithm_seed.as_deref()), ("", None), "restates-the-unset-algorithm-and-its-absent-seed's base scene must carry the unset algorithm pair this payload restates");
    let applied = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: applied state differs from committed after-snapshot");
    assert_eq!(applied.results, base.results, "a no-op update-graph-algorithm must not mint a fresh composed results child");
}

/// ↩️ The undo captures BASE's algorithm and seed together — never one without the other, which is
/// precisely why this verb is `update-<facet-pair>` and not two `change-` scalars.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: String::new(), new_algorithm_seed: None })], "update-graph-algorithm inverts to BASE's own (algorithm, seed) pair, got {inverse:?}");
    let mut snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &snapshot);
        snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `newAlgorithmSeed`
/// carries no `skip_serializing_if`, so an absent seed must be committed as an explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert!(original.pointer("/UpdateGraphAlgorithm/newAlgorithmSeed").expect("the payload commits its seed slot").is_null(), "an absent seed is committed as an explicit null, never omitted");
}

/// 🎯️ The declared outcome — `no-op`, with one `mutation.no-op` warning — is what the builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("no-op"), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed declares a no-op outcome");
    let emitted = produced();
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "restating the current algorithm pair is reported as no-op");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning, "a no-op is a Warning — never a refusal, it just changes nothing");
    let declared = outcome.get("messages").and_then(semio_framework_pack_json::Value::as_array).expect("the declared outcome carries its warning");
    assert_eq!(declared[0].get("code").and_then(semio_framework_pack_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
}

/// 🔺️ A no-op emits the artifact's `Default` diff — every slot `null` — proving the guard fires before
/// `crate::equation_state_diff` is ever reached.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &EquationDiff::default(), "a no-op update-graph-algorithm must carry the empty diff, never a re-minted child triple");
    let produced_value = semio_framework_pack_json::from_dsl_value(&(outcome.diff()).to_value());
    let committed = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert!(
        semio_framework_pack_json::value_eq_ignoring_object_order(&produced_value, &committed),
        "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: produced diff differs from the committed 🔺️diff/🔣️.json ({produced_value:?} vs {committed:?})"
    );
}

/// 🔣️ The committed diff is canonical and decodes to `EquationDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    let original = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced_snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed diff did not carry before to after");
}

/// 🧮️ The no-op guard is a CONJUNCTION over both facets — this is the assertion that makes
/// `update-graph-algorithm` the vocabulary's one inseparable-facet `update`: keeping the algorithm
/// but attaching a seed is still a real change, so the guard must not fire on the algorithm alone.
#[semio_framework_async_macros::async_test]
async fn attaching_a_seed_alone_is_not_a_no_op() {
    let base = before();
    let seeded = EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: String::new(), new_algorithm_seed: Some("seed-1".into()) });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&seeded, &base);
    assert!(outcome.messages().is_empty(), "changing the seed alone is a real change, not a no-op, got {:?}", outcome.messages());
    assert!(outcome.diff().notation.is_some() && outcome.diff().results.is_some() && outcome.diff().computed.is_some(), "a graph-scoped equation mutation regenerates notation/results/computed together");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("update", "graph", "update-graph-algorithm", "UpdatedGraphAlgorithm"), "the fixture must be bound to update-graph-algorithm's own descriptor");
}

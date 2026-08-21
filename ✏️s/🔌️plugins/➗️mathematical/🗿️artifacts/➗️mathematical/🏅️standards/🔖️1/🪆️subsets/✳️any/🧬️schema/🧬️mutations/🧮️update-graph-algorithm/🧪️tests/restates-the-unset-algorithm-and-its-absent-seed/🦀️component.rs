//! 🧪️ `update-graph-algorithm` fixture — `restates-the-unset-algorithm-and-its-absent-seed`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Why this leaf pins the NO-OP branch: `MathematicalSnapshot` keeps its graph and its point
//! cloud in three co-derived composed CHILDREN (`notation`/`results`/`computed`,
//! `🔖️WorkingScene`), and every state-changing mathematical diff re-mints all three through
//! `mathematical_children_from_state`, whose `child_id` is a `DefaultHasher` digest of the child
//! content — a value `std` deliberately leaves unspecified, so it cannot honestly be hand-authored
//! into an `➡️after`. A committed snapshot therefore decodes to an UNRESOLVED handle and
//! `mathematical_scene` fails soft to a graph whose `algorithm` is `""` and whose `algorithm_seed`
//! is `None` — exactly the pair this committed payload restates, taking the verb's own
//! `mutation.no-op` guard.

use crate::artifacts::mathematical::mutations::update_graph_algorithm::mutation::UpdateGraphAlgorithm;
use crate::artifacts::mathematical::{mathematical_graph, MathematicalDiff, MathematicalMutation, MathematicalSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> MathematicalSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> MathematicalSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> MathematicalMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<MathematicalDiff> {
    <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Restating both inseparable facets — algorithm AND seed — carries `before` to exactly the
/// committed `after`, i.e. leaves it untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let graph = mathematical_graph(&base);
    assert_eq!((graph.algorithm.as_str(), graph.algorithm_seed.as_deref()), ("", None), "restates-the-unset-algorithm-and-its-absent-seed's base scene must carry the unset algorithm pair this payload restates");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: applied state differs from committed after-snapshot");
    assert_eq!(applied.results, base.results, "a no-op update-graph-algorithm must not mint a fresh composed results child");
}

/// ↩️ The undo captures BASE's algorithm and seed together — never one without the other, which is
/// precisely why this verb is `update-<facet-pair>` and not two `change-` scalars.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(
        inverse,
        vec![MathematicalMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: String::new(), new_algorithm_seed: None })],
        "update-graph-algorithm inverts to BASE's own (algorithm, seed) pair, got {inverse:?}"
    );
    let mut snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(step, &snapshot);
        snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `new_algorithm_seed`
/// carries no `skip_serializing_if`, so an absent seed must be committed as an explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed mutation JSON is not canonical");
    assert!(original.pointer("/UpdateGraphAlgorithm/new_algorithm_seed").expect("the payload commits its seed slot").is_null(), "an absent seed is committed as an explicit null, never omitted");
}

/// 🎯️ The declared outcome — applied, with one `mutation.no-op` warning — is what the builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed declares an applied outcome");
    let emitted = produced();
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "restating the current algorithm pair is reported as no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning — applied, but with nothing to change");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries its warning");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
}

/// 🔺️ A no-op emits the artifact's `Default` diff — all eight slots `null` — proving the guard
/// fires before `mathematical_children_from_state` is ever reached.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &MathematicalDiff::default(), "a no-op update-graph-algorithm must carry the empty diff, never a re-minted child triple");
    let produced_value = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced_value, committed, "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `MathematicalDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: MathematicalDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: MathematicalDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced_snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed diff did not carry before to after");
}

/// 🧮️ The no-op guard is a CONJUNCTION over both facets — this is the assertion that makes
/// `update-graph-algorithm` the vocabulary's one inseparable-facet `update`: keeping the algorithm
/// but attaching a seed is still a real change, so the guard must not fire on the algorithm alone.
#[semio_framework_async_macros::async_test]
async fn attaching_a_seed_alone_is_not_a_no_op() {
    let base = before();
    let seeded = MathematicalMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: String::new(), new_algorithm_seed: Some("seed-1".into()) });
    let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&seeded, &base);
    assert!(outcome.messages().is_empty(), "changing the seed alone is a real change, not a no-op, got {:?}", outcome.messages());
    assert!(outcome.diff().notation.is_some() && outcome.diff().results.is_some() && outcome.diff().computed.is_some(), "a graph-scoped mathematical mutation regenerates notation/results/computed together");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("update", "graph", "update-graph-algorithm", "UpdatedGraphAlgorithm"), "the fixture must be bound to update-graph-algorithm's own descriptor");
}

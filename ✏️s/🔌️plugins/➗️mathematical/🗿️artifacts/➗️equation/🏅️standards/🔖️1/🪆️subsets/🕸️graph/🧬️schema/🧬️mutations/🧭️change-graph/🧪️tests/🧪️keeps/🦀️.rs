//! 🧪️ `change-graph-directed` fixture — `➡️keeps-an-already-directed-graph-directed`.
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

use crate::standards::v1::subsets::graph::schema::mutations::change_graph_directed::ChangeGraphDirected;
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/🎯️outcome/🔣️.json");

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

/// ▶️ Re-asserting the direction the graph already has carries `before` to exactly the committed
/// `after` — which for a no-op means "unchanged, and with the composed child triple untouched".
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    assert!(base.graph.directed, "change-graph-directed/keeps-an-already-directed-graph-directed: the base graph must already be directed for this fixture to reach the no-op guard");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "change-graph-directed/keeps-an-already-directed-graph-directed: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a no-op change-graph-directed must not mint a fresh notation/results/computed triple");
}

/// ↩️ This verb inverts from BASE state, so undoing a no-op re-asserts the very same flag and the
/// snapshot is restored trivially.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&forward, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: true })], "change-graph-directed inverts to the flag BASE carried, got {inverse:?}");
    let mut snapshot = protocol::apply_diff(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-graph-directed/keeps-an-already-directed-graph-directed: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point. The payload field is `newDirected` — every `EquationMutation` leaf carries
/// `#[value(rename_all = "camelCase")]`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🎯️ The declared outcome — `no-op`, with one `mutation.no-op` warning — is exactly what the diff
/// builder emits. A warn no-op is a NO-OP with an empty diff, never a rejection.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("no-op"), "change-graph-directed/keeps-an-already-directed-graph-directed declares a no-op outcome");
    let emitted = produced();
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "restating the current direction is reported as no-op");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning, "a no-op is a Warning — never a refusal, the document is just unchanged");
    let declared = outcome.get("messages").and_then(semio_framework_pack_json::Value::as_array).expect("the declared outcome carries its warning");
    assert_eq!(declared[0].get("code").and_then(semio_framework_pack_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
}

/// 🔺️ A no-op emits the artifact's `Default` diff — every sparse slot `null`.
/// This is the assertion that proves the guard fires BEFORE `crate::equation_state_diff`:
/// a wrong guard would leave a freshly minted triple behind even though nothing changed.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &EquationDiff::default(), "a no-op change-graph-directed must carry the empty diff, never a re-minted child triple");
    let produced_value = semio_framework_pack_json::from_dsl_value(&(outcome.diff()).to_value());
    let committed = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&produced_value, &committed), "change-graph-directed/keeps-an-already-directed-graph-directed: produced diff differs from the committed 🔺️diff/🔣️.json ({produced_value:?} vs {committed:?})");
}

/// 🔣️ The committed diff is itself canonical and decodes to `EquationDiff`, whose container
/// `#[serde(default)]` carries NO per-field `skip_serializing_if` — so all eight fields must be
/// present as explicit `null`s.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    let original = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 6, "EquationDiff emits all six artifact slots, `null` for the untouched ones");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced_snapshot = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "change-graph-directed/keeps-an-already-directed-graph-directed: committed diff did not carry before to after");
}

/// 🔀️ The guard is value-sensitive, not unconditional: flipping the flag the other way DOES build
/// a diff, and — this verb being graph-scoped — regenerates all three co-derived children at once
/// while leaving the inline `equation` slot alone.
#[semio_framework_async_macros::async_test]
async fn flipping_the_flag_the_other_way_regenerates_the_whole_child_triple() {
    let base = before();
    let flip = EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: false });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&flip, &base);
    assert!(outcome.messages().is_empty(), "flipping to undirected is a real change, not a no-op, got {:?}", outcome.messages());
    let diff = outcome.diff();
    assert!(diff.notation.is_some() && diff.results.is_some() && diff.computed.is_some(), "a graph-scoped equation mutation regenerates notation/results/computed together");
    assert!(diff.equation.is_none(), "change-graph-directed never touches the inline equation slot");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "graph", "change-graph-directed", "ChangedGraphDirected"), "the fixture must be bound to change-graph-directed's own descriptor");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

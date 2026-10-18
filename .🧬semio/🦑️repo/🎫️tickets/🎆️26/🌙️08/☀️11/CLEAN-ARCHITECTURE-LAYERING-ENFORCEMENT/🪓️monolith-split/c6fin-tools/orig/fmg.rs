//! 🧪️ Aggregated unit tests of the `mutations` leaves, one module per leaf.

mod change_graph_directed {
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧭️change-graph/🧪️keeps/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
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
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🎯️ The declared outcome — `no-op`, with one `mutation.no-op` warning — is exactly what the diff
/// builder emits. A warn no-op is a NO-OP with an empty diff, never a rejection.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
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
/// This is the assertion that proves the guard fires BEFORE `EquationDiff::apply`:
/// a wrong guard would leave a freshly minted triple behind even though nothing changed.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &EquationDiff::default(), "a no-op change-graph-directed must carry the empty diff, never a re-minted child triple");
    let produced_value = semio_framework_pack_json::from_dsl_value(&(outcome.diff()).to_value());
    let committed = semio_framework_pack_json::parse((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&produced_value, &committed), "change-graph-directed/keeps-an-already-directed-graph-directed: produced diff differs from the committed 🔺️diff/🔣️.json ({produced_value:?} vs {committed:?})");
}

/// 🔣️ The committed diff is itself canonical and decodes to `EquationDiff`, whose container
/// `#[serde(default)]` carries NO per-field `skip_serializing_if` — so all eight fields must be
/// present as explicit `null`s.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    let original = semio_framework_pack_json::parse((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 6, "EquationDiff emits all six artifact slots, `null` for the untouched ones");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod update_graph_algorithm {
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🧮️update-graph/🧪️restates/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
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
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
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
    let mut snapshot = protocol::apply_diff(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `newAlgorithmSeed`
/// carries no `skip_serializing_if`, so an absent seed must be committed as an explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert!(original.pointer("/UpdateGraphAlgorithm/newAlgorithmSeed").expect("the payload commits its seed slot").is_null(), "an absent seed is committed as an explicit null, never omitted");
}

/// 🎯️ The declared outcome — `no-op`, with one `mutation.no-op` warning — is what the builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
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
/// `EquationDiff::apply` is ever reached.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &EquationDiff::default(), "a no-op update-graph-algorithm must carry the empty diff, never a re-minted child triple");
    let produced_value = semio_framework_pack_json::from_dsl_value(&(outcome.diff()).to_value());
    let committed = semio_framework_pack_json::parse((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert!(
        semio_framework_pack_json::value_eq_ignoring_object_order(&produced_value, &committed),
        "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: produced diff differs from the committed 🔺️diff/🔣️.json ({produced_value:?} vs {committed:?})"
    );
}

/// 🔣️ The committed diff is canonical and decodes to `EquationDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    let original = semio_framework_pack_json::parse((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "update-graph-algorithm/restates-the-unset-algorithm-and-its-absent-seed: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced_snapshot = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
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

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod create_node {
//! 🧪️ `create-node` fixture — `🚫️rejects-a-duplicate-node-id`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE, and the
//! `notation`/`results`/`computed` handles are the content addresses of their derivation (`crate::equation_children`), kept
//! exact by the fixture writer law.
//!
//! 🌱 `create-node` refuses only a colliding id, so — following dag's `🚫️rejects-a-duplicate-node-id` precedent — the committed
//! graph already holds exactly the node the committed payload asks to create: its `id`/`label`/`x`/`y` ARE the mutation JSON's.

use crate::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode;
use crate::{EquationDiff, EquationGraph, EquationMutation, EquationNode, EquationSnapshot};
use semio_framework_value::ToValue;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/➕️create-node/🧪️rejects/🎯️outcome/🔣️.json"));

fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// 🟢️ The committed payload, unwrapped — the colliding node is built from it and nothing else.
fn payload() -> CreateNode {
    let EquationMutation::CreateNode(payload) = mutation() else {
        panic!("rejects-a-duplicate-node-id's committed mutation must be a create-node");
    };
    payload
}

/// 🌱 The graph both committed snapshots hold: it already contains the very node the payload tries to create — the
/// collision `mutation.duplicate-id` guards against.
fn colliding_graph() -> EquationGraph {
    let payload = payload();
    EquationGraph { directed: true, nodes: vec![EquationNode { id: payload.id.clone(), label: payload.label.clone(), x: payload.x, y: payload.y }], edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }
}

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `create-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert_eq!(base.graph, colliding_graph(), "rejects-a-duplicate-node-id's before-snapshot holds exactly the payload's node");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "create-node/rejects-a-duplicate-node-id: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected create must not mint a fresh notation/results/computed triple");
}

/// 🚨️ A colliding node id is a FATAL `mutation.duplicate-id`, not the Error-level `target-missing`
/// the rest of this vocabulary uses — a duplicate identity is an invariant breach, not a miss. The
/// diagnostic names the NODE id the payload carried, never its label or coordinates.
#[semio_framework_async_macros::async_test]
async fn a_colliding_id_is_a_fatal_duplicate_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting create-node must carry an empty diff, never a half-built child triple");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec![payload().id.clone()], "the diagnostic addresses the colliding node id");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "node", "create-node", "CreatedNode"), "the fixture must be bound to create-node's own descriptor");
}

/// ↩️ Equation's `create-node` inverse is BASE-GUARDED, not unconditionally payload-derived:
/// because `base` already carries this id, the create is treated as having changed nothing and the
/// undo is empty — a deliberate divergence from dag's create-node, whose inverse is always a
/// delete of the requested id.
#[semio_framework_async_macros::async_test]
async fn an_already_present_id_leaves_nothing_to_undo() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "create-node/rejects-a-duplicate-node-id: a create whose id BASE already holds must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed
/// point. `x`/`y` are `f64`, so they always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "create-node/rejects-a-duplicate-node-id: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "create-node/rejects-a-duplicate-node-id: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "create-node/rejects-a-duplicate-node-id declares a rejected outcome");
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod delete_node {
//! 🧪️ `delete-node` fixture — `🚫️rejects-deleting-a-node-that-is-not-in-the-graph`.
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/❌️delete-node/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `delete-node` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-deleting-a-node-that-is-not-in-the-graph's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected delete must not mint a fresh notation/results/computed triple");
}

/// ❌️ `delete-node` is the SINGULAR, id-keyed delete: it names exactly one node id, and — this is
/// the branch that separates it from a successful delete — it never reaches its edge cascade, so
/// no `mutation.cascade` Info message accompanies the refusal.
#[semio_framework_async_macros::async_test]
async fn a_missing_node_is_reported_alone_with_no_cascade_notice() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting delete-node must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "a refused delete emits the miss and nothing else, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "delete-node has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string()], "the diagnostic names the single node id the payload carried");
    assert!(messages.iter().all(|message| message.code.0 != "mutation.cascade"), "the edge cascade is unreachable when the node was never found, got {messages:?}");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "node", "delete-node", "DeletedNode"), "the fixture must be bound to delete-node's own SINGULAR descriptor");
}

/// ↩️ `delete-node` inverts by re-creating the node and re-`connect`ing every edge it severed, all
/// captured from BASE. With no such node the inverse is empty — never a speculative create.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_node_to_recreate() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: a rejected delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed
/// point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "delete-node/rejects-deleting-a-node-that-is-not-in-the-graph declares a rejected outcome");
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod delete_nodes {
//! 🧪️ `delete-nodes` fixture — `🚫️rejects-a-bulk-delete-where-every-id-is-absent`.
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-nodes/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `delete-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-a-bulk-delete-where-every-id-is-absent's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
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
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "delete-nodes has no Fatal branch at all");
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
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: a rejected bulk delete must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The plural payload's one
/// field is `ids` — a bare `Vec<String>`, never a nested selection object.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/DeleteNodes/ids").and_then(semio_framework_pack_json::Value::as_array).map(|ids| ids.len()), Some(2), "the committed payload must request two ids for the bulk branch to be meaningful");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "delete-nodes/rejects-a-bulk-delete-where-every-id-is-absent declares a rejected outcome");
    let emitted = produced();
    let message = emitted.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(semio_framework_pack_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(semio_framework_pack_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target — for the plural verb that is the whole id list");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod change_node_label {
//! 🧪️ `change-node-label` fixture — `🚫️rejects-relabelling-a-node-that-is-not-in-the-graph`.
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🏷️change-node/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `change-node-label` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty(), "rejects-relabelling-a-node-that-is-not-in-the-graph's before-snapshot must hold a node-less graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
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
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting change-node-label must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing node is reported as target-missing, never as a no-op");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "change-node-label has no Fatal branch at all");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string()], "the diagnostic names the stable node id");
    assert!(!messages[0].target.contains(&"Alpha".to_string()), "the requested label must never leak into the diagnostic target");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "node", "change-node-label", "ChangedNodeLabel"), "the fixture must be bound to change-node-label's own descriptor — `change`, never `rename`");
}

/// ↩️ The undo carries BASE's own label for that id. With no such node the inverse is empty, so a
/// refused relabel can never be replayed as an accidental label wipe.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_prior_label_to_restore() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: a rejected relabel must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload field is
/// camelCase (`newLabel`): `EquationMutation`'s leaf structs carry `#[value(rename_all = "camelCase")]`
/// like the nested document types.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/ChangeNodeLabel/newLabel").and_then(semio_framework_pack_json::Value::as_str), Some("Alpha"), "the payload's label field commits camelCase");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "change-node-label/rejects-relabelling-a-node-that-is-not-in-the-graph declares a rejected outcome");
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod move_node {
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🕹️move-node/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
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
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "move-node/rejects-moving-a-node-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/MoveNode/x").and_then(semio_framework_pack_json::Value::as_f64), Some(120.0), "the payload commits an absolute destination, not an offset");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod connect_nodes {
//! 🧪️ `connect-nodes` fixture — `🚫️rejects-an-edge-between-two-absent-endpoints`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and a `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Model (a) (design §20.15): the committed snapshot carries its graph and its point cloud INLINE as parent-owned
//! fields, every leaf decides from them, and the `notation`/`results`/`computed` handles are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law. This vector pins the outcome its
//! `🎯️outcome` declares on that committed state.

use crate::standards::v1::subsets::graph::schema::mutations::disconnect_nodes::DisconnectNodes;
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/🔗️connect-nodes/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ A rejected `connect-nodes` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.graph.nodes.is_empty() && base.graph.edges.is_empty(), "rejects-an-edge-between-two-absent-endpoints' before-snapshot must hold a empty graph");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "connect-nodes/rejects-an-edge-between-two-absent-endpoints: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected connect must not mint a fresh notation/results/computed triple");
}

/// 🔗️ `connect-nodes` is the only verb in this vocabulary whose diagnostic names something OTHER
/// than the entity it was asked to create: the edge id `e-alpha-beta` never appears — the target is
/// the list of missing ENDPOINT node ids, both of them, in source-then-target order. (Its own id is
/// only ever reported by the earlier Fatal `mutation.duplicate-id` branch, which an edge-less graph
/// cannot reach.)
#[semio_framework_async_macros::async_test]
async fn the_missing_endpoints_are_named_not_the_new_edge_id() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting connect-nodes must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "absent endpoints are reported as target-missing, not duplicate-id");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "a missing endpoint is an Error — the Fatal is reserved for a colliding edge id");
    assert_eq!(messages[0].target, vec!["n-alpha".to_string(), "n-beta".to_string()], "the diagnostic names the endpoint nodes, in source-then-target order");
    assert!(!messages[0].target.contains(&"e-alpha-beta".to_string()), "the edge id must not appear in a target-missing diagnostic");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("connect", "node", "connect-nodes", "ConnectedNodes"), "the fixture must be bound to connect-nodes' own descriptor — a PLURAL kind over a SINGULAR entity");
}

/// ↩️ `connect-nodes`' inverse is PAYLOAD-derived, unlike every other refusing verb in this
/// vocabulary: because BASE holds no edge under that id, the undo is a `disconnect-nodes` of the id
/// it was asked to create, even though the connect itself was refused.
#[semio_framework_async_macros::async_test]
async fn inverse_is_a_disconnect_of_the_requested_edge_id() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![EquationMutation::DisconnectNodes(DisconnectNodes { id: "e-alpha-beta".to_string() })], "connect-nodes undoes with exactly one disconnect of the requested edge id, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The payload carries three
/// flat id strings — `id`, `source`, `target` — never a nested endpoint object.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "connect-nodes/rejects-an-edge-between-two-absent-endpoints: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "connect-nodes/rejects-an-edge-between-two-absent-endpoints: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/ConnectNodes/source").and_then(semio_framework_pack_json::Value::as_str), Some("n-alpha"), "the payload addresses its endpoints by bare node id");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "connect-nodes/rejects-an-edge-between-two-absent-endpoints declares a rejected outcome");
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod disconnect_nodes {
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../🕸️graph/🧪️tests/🧫️fixtures/🧬️mutations/✂️disconnect-nodes/🧪️rejects/🎯️outcome/🔣️.json"));

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
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
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "disconnect-nodes/rejects-severing-an-edge-that-is-not-in-the-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/DisconnectNodes").and_then(semio_framework_pack_json::Value::as_object).map(|fields| fields.len()), Some(1), "the payload carries the edge id and nothing else");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

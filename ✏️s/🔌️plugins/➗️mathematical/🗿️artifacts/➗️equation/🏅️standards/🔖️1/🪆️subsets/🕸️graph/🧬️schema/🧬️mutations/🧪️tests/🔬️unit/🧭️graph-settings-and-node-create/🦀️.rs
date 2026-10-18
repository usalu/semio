/// 🧭️ Graph settings and node create: the unit tests of the change graph directed, update graph algorithm and create node leaves, one module per leaf.

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

//! 🧪️ `replace-graph` fixture — `replays-the-identical-empty-graph`.
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
//! `mathematical_scene` fails soft to the empty graph this committed payload replays verbatim,
//! taking `replace-graph`'s own whole-value `mutation.no-op` guard.

use crate::artifacts::mathematical::standards::v1::subsets::graph::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::mathematical::{mathematical_graph, MathematicalDiff, MathematicalGraph, MathematicalMutation, MathematicalSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

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

/// ▶️ `replace-graph` compares the WHOLE payload graph against base — direction, node list, edge
/// list, algorithm and seed at once — so replaying an identical graph carries `before` to exactly
/// the committed `after`, i.e. leaves it untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let MathematicalMutation::ReplaceGraph(payload) = mutation() else {
        panic!("replays-the-identical-empty-graph's committed mutation must be a replace-graph");
    };
    assert_eq!(mathematical_graph(&base), payload.graph, "the committed payload must be byte-for-byte the graph BASE resolves to, or the no-op guard is never reached");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "replace-graph/replays-the-identical-empty-graph: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a no-op replace-graph must not mint a fresh notation/results/computed triple");
}

/// ↩️ `replace-graph` inverts to another `replace-graph` carrying BASE's whole prior graph — it is
/// base-derived, never payload-derived.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse, vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: mathematical_graph(&base) })], "replace-graph inverts to a replace-graph carrying BASE's whole prior graph, got {inverse:?}");
    let mut snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(step, &snapshot);
        snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-graph/replays-the-identical-empty-graph: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. The nested
/// `MathematicalGraph` DOES carry `#[serde(rename_all = "camelCase")]`, so its seed slot is
/// `algorithmSeed` even though the payload's own fields stay snake_case.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "replace-graph/replays-the-identical-empty-graph: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&(mutation()).to_value());
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "replace-graph/replays-the-identical-empty-graph: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert!(original.pointer("/ReplaceGraph/graph/algorithmSeed").is_some(), "the nested graph renames algorithm_seed to algorithmSeed");
}

/// 🎯️ The declared outcome — applied, with one `mutation.no-op` warning — is what the builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::JsonValue::as_str), Some("applied"), "replace-graph/replays-the-identical-empty-graph declares an applied outcome");
    let emitted = produced();
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "replaying an identical graph is reported as no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning — applied, but with nothing to change");
    let declared = outcome.get("messages").and_then(pack::JsonValue::as_array).expect("the declared outcome carries its warning");
    assert_eq!(declared[0].get("code").and_then(pack::JsonValue::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
}

/// 🔺️ A no-op emits the artifact's `Default` diff — all eight slots `null`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &MathematicalDiff::default(), "a no-op replace-graph must carry the empty diff, never a re-minted child triple");
    let produced_value = pack::json_from_dsl_value(&(outcome.diff()).to_value());
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert!(pack::json::value_eq_ignoring_object_order(&produced_value, &committed), "replace-graph/replays-the-identical-empty-graph: produced diff differs from the committed 🔺️diff/🔣️.json ({produced_value:?} vs {committed:?})");
}

/// 🔣️ The committed diff is canonical and decodes to `MathematicalDiff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: MathematicalDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = pack::json_from_dsl_value(&decoded.to_value());
    let original = pack::parse_json(DIFF).expect("committed diff reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "replace-graph/replays-the-identical-empty-graph: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: MathematicalDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced_snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "replace-graph/replays-the-identical-empty-graph: committed diff did not carry before to after");
}

/// 🔁️ The guard is a WHOLE-VALUE comparison, not a field sample: a payload that differs in the
/// single `algorithm` field is a real replacement and re-derives the whole child triple.
#[semio_framework_async_macros::async_test]
async fn a_graph_differing_in_one_field_is_a_real_replacement() {
    let base = before();
    let retitled = MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: MathematicalGraph { algorithm: "bfs".into(), ..mathematical_graph(&base) } });
    let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&retitled, &base);
    assert!(outcome.messages().is_empty(), "a graph that differs anywhere is a real replacement, not a no-op, got {:?}", outcome.messages());
    assert!(outcome.diff().notation.is_some() && outcome.diff().results.is_some() && outcome.diff().computed.is_some(), "replace-graph regenerates all three co-derived children at once");
    assert!(outcome.diff().equation.is_none(), "replace-graph never touches the inline equation slot");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("replace", "graph", "replace-graph", "ReplacedGraph"), "the fixture must be bound to replace-graph's own descriptor");
}

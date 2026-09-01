//! 🧪️ `change-graph-directed` fixture — `keeps-an-already-directed-graph-directed`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ Why this leaf pins the NO-OP branch rather than a state change: `MathematicalSnapshot` keeps
//! its graph and its point cloud in three co-derived composed CHILDREN (`notation`/`results`/
//! `computed`, `🔖️WorkingScene`), and every state-changing mathematical diff re-mints all three
//! through `mathematical_children_from_state`, whose `child_id` is a `DefaultHasher` digest of the
//! child content — a value `std` deliberately leaves unspecified, so it cannot honestly be
//! hand-authored into an `➡️after`. A committed snapshot therefore decodes to an UNRESOLVED handle
//! and `mathematical_scene` fails soft to an empty — but DIRECTED — graph. Re-asserting
//! `directed = true` against it takes this verb's own `mutation.no-op` guard: an empty diff, no
//! re-minting at all, and an `➡️after` byte-identical to `⬅️before`.

use crate::artifacts::mathematical::mutations::change_graph_directed::mutation::ChangeGraphDirected;
use crate::artifacts::mathematical::{mathematical_graph, MathematicalDiff, MathematicalMutation, MathematicalSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

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

/// ▶️ Re-asserting the direction the graph already has carries `before` to exactly the committed
/// `after` — which for a no-op means "unchanged, and with the composed child triple untouched".
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    assert!(mathematical_graph(&base).directed, "change-graph-directed/keeps-an-already-directed-graph-directed: the unresolved base scene must already be directed for this fixture to reach the no-op guard");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "change-graph-directed/keeps-an-already-directed-graph-directed: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a no-op change-graph-directed must not mint a fresh notation/results/computed triple");
}

/// ↩️ This verb inverts from BASE state, so undoing a no-op re-asserts the very same flag and the
/// snapshot is restored trivially.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&forward, &base);
    assert_eq!(inverse, vec![MathematicalMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: true })], "change-graph-directed inverts to the flag BASE carried, got {inverse:?}");
    let mut snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(step, &snapshot);
        snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-graph-directed/keeps-an-already-directed-graph-directed: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point. Note the payload field is `new_directed` — this enum carries no
/// `#[serde(rename_all)]`, so mutation payloads stay snake_case on the wire.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&(mutation()).to_value());
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🎯️ The declared outcome — applied, with one `mutation.no-op` warning — is exactly what the diff
/// builder emits. A warn no-op is APPLIED with an empty diff, never a rejection.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::JsonValue::as_str), Some("applied"), "change-graph-directed/keeps-an-already-directed-graph-directed declares an applied outcome");
    let emitted = produced();
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "restating the current direction is reported as no-op");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "a no-op is a Warning — the document is still applied, just unchanged");
    let declared = outcome.get("messages").and_then(pack::JsonValue::as_array).expect("the declared outcome carries its warning");
    assert_eq!(declared[0].get("code").and_then(pack::JsonValue::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
}

/// 🔺️ A no-op emits the artifact's `Default` diff — every one of the eight sparse slots `null`.
/// This is the assertion that proves the guard fires BEFORE `mathematical_children_from_state`:
/// a wrong guard would leave a freshly minted triple behind even though nothing changed.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &MathematicalDiff::default(), "a no-op change-graph-directed must carry the empty diff, never a re-minted child triple");
    let produced_value = pack::json_from_dsl_value(&(outcome.diff()).to_value());
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert!(pack::json::value_eq_ignoring_object_order(&produced_value, &committed), "change-graph-directed/keeps-an-already-directed-graph-directed: produced diff differs from the committed 🔺️diff/🔣️component.json ({produced_value:?} vs {committed:?})");
}

/// 🔣️ The committed diff is itself canonical and decodes to `MathematicalDiff`, whose container
/// `#[serde(default)]` carries NO per-field `skip_serializing_if` — so all eight fields must be
/// present as explicit `null`s.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: MathematicalDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = pack::json_from_dsl_value(&decoded.to_value());
    let original = pack::parse_json(DIFF).expect("committed diff reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "change-graph-directed/keeps-an-already-directed-graph-directed: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 8, "MathematicalDiff emits all eight slots, `null` for the untouched ones");
}

/// 🩹 Applying the committed (empty) diff to `before` yields the committed `after` unchanged.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: MathematicalDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let produced_snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "change-graph-directed/keeps-an-already-directed-graph-directed: committed diff did not carry before to after");
}

/// 🔀️ The guard is value-sensitive, not unconditional: flipping the flag the other way DOES build
/// a diff, and — this verb being graph-scoped — regenerates all three co-derived children at once
/// while leaving the inline `equation` slot alone.
#[semio_framework_async_macros::async_test]
async fn flipping_the_flag_the_other_way_regenerates_the_whole_child_triple() {
    let base = before();
    let flip = MathematicalMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: false });
    let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&flip, &base);
    assert!(outcome.messages().is_empty(), "flipping to undirected is a real change, not a no-op, got {:?}", outcome.messages());
    let diff = outcome.diff();
    assert!(diff.notation.is_some() && diff.results.is_some() && diff.computed.is_some(), "a graph-scoped mathematical mutation regenerates notation/results/computed together");
    assert!(diff.equation.is_none(), "change-graph-directed never touches the inline equation slot");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("change", "graph", "change-graph-directed", "ChangedGraphDirected"), "the fixture must be bound to change-graph-directed's own descriptor");
}

//! 🧪️ `insert-point` fixture — `📍️seeds-the-empty-cloud-with-its-first-point`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ `insert-point` has neither a rejection branch nor a no-op guard (an out-of-range index is CLAMPED, a Warning), so every
//! reachable outcome is APPLIED. Model (a) (design §20.15): the committed snapshots carry the graph and the point cloud inline,
//! and the `notation`/`results`/`computed` handles in `⬅️before`, `➡️after` and `🔺️diff` are the content addresses of their
//! derivation (`crate::equation_children`), kept exact by the fixture writer law — everything here is the committed bytes.

use crate::standards::v1::subsets::geometry::schema::mutations::insert_point::InsertPoint;
use crate::standards::v1::subsets::geometry::schema::mutations::remove_point::RemovePoint;
use crate::{EquationDiff, EquationGeometry, EquationMutation, EquationPoint, EquationSnapshot};
use semio_framework_value::ToValue;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/🎯️outcome/🔣️.json");

fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ➕️ The committed payload, unwrapped — every state below is derived from it, never invented.
fn payload() -> InsertPoint {
    let EquationMutation::InsertPoint(payload) = mutation() else {
        panic!("seeds-the-empty-cloud-with-its-first-point's committed mutation must be an insert-point");
    };
    payload
}

fn after_geometry() -> EquationGeometry {
    EquationGeometry { points: vec![EquationPoint { x: payload().x, y: payload().y }] }
}

fn before() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn expected_diff() -> EquationDiff {
    semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes")
}

fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Inserting at index 0 of an empty cloud carries `before` to exactly the committed `after`: one
/// point at `(5, 6)`, the graph untouched, and the inline `equation` untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    assert!(base.geometry.points.is_empty(), "seeds-the-empty-cloud-with-its-first-point's base cloud must be empty for `index: 0` to be the exact end of the cloud");
    let applied = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("insert-point applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "insert-point/seeds-the-empty-cloud-with-its-first-point: applied state differs from committed after-snapshot");
    assert_eq!(applied.geometry.points, after_geometry().points, "the inserted point must land verbatim at the payload's coordinates");
    assert_eq!(applied.equation, base.equation, "insert-point is geometry-scoped — it never touches the inline equation slot");
}

/// ↩️ `insert-point`'s `index` is FINAL-state, so its undo is a `remove-point` at the SAME index —
/// clamped against BASE's length, which here is 0.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![EquationMutation::RemovePoint(RemovePoint { index: 0 })], "insert-point inverts to a remove-point at the same index, got {inverse:?}");
    let mut snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &snapshot);
        snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "insert-point/seeds-the-empty-cloud-with-its-first-point: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `index` is a `usize`, so
/// it commits as a bare integer, while `x`/`y` are `f64` and always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/InsertPoint/index").and_then(semio_framework_pack_json::Value::as_u64), Some(0), "an index-keyed geometry verb commits its address as a bare integer");
}

/// 🎯️ The declared outcome is a clean `applied` — index 0 is within range of an empty cloud, so
/// the clamp warning must NOT fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("applied"), "insert-point/seeds-the-empty-cloud-with-its-first-point declares an applied outcome");
    let emitted = produced();
    assert!(emitted.messages().is_empty(), "an in-range insert raises no diagnostic at all, got {:?}", emitted.messages());
    assert!(outcome.get("messages").is_none(), "a clean applied outcome commits no messages array");
}

/// 🔺️ The produced delta is exactly the committed one: the new `graph`/`geometry` with all three derived handles re-minted
/// together, and the equation slot left null. Its owned encoding contains exactly the six artifact fields.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &expected_diff(), "insert-point/seeds-the-empty-cloud-with-its-first-point: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(outcome.diff().equation.is_none(), "insert-point fills only the state slots and their derived handles");
    let encoded: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("third-party diff decoder");
    assert_eq!(encoded.as_object().unwrap().keys().map(String::as_str).collect::<std::collections::BTreeSet<_>>(), std::collections::BTreeSet::from(["graph", "geometry", "notation", "results", "computed", "equation"]));
}

/// 🔣️ The committed diff is itself canonical and decodes to `EquationDiff`, whose owned codec emits all six artifact slots,
/// including null values, and its handles are the content addresses of the inserted state.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    let original = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 6, "EquationDiff emits all six artifact slots, `null` for the untouched ones");
    let (notation, results, computed) = crate::equation_children(decoded.graph.as_ref().expect("an applied diff carries the graph"), decoded.geometry.as_ref().expect("an applied diff carries the point cloud"));
    assert_eq!((decoded.notation, decoded.results, decoded.computed), (Some(notation), Some(results), Some(computed)), "the committed handles are the content addresses of the inserted state");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the insert, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let produced_snapshot = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(&expected_diff(), &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed diff did not carry before to after");
}

/// ➕️ The branch this verb owns and no other geometry verb has: an out-of-range index is CLAMPED to
/// the end of the cloud and reported as a Warning `mutation.clamped` — it still applies, unlike
/// `remove-point`/`move-points`, which reject an out-of-range index outright.
#[semio_framework_async_macros::async_test]
async fn an_out_of_range_index_clamps_instead_of_rejecting() {
    let base = before();
    let past_the_end = EquationMutation::InsertPoint(InsertPoint { index: 9, x: payload().x, y: payload().y });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&past_the_end, &base);
    let messages = outcome.messages();
    assert_eq!(messages.len(), 1, "a clamped insert raises exactly one diagnostic, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.clamped", "an out-of-range insert index is clamped, never reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning, "clamping is a Warning — insert-point has no Error or Fatal branch at all");
    let clamped = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &base).expect("a clamped insert still applies");
    assert_eq!(clamped.geometry.points, after_geometry().points, "clamping lands the point at the end of the cloud — here the same single slot index 0 names");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("insert", "point", "insert-point", "InsertedPoint"), "the fixture must be bound to insert-point's own descriptor");
}

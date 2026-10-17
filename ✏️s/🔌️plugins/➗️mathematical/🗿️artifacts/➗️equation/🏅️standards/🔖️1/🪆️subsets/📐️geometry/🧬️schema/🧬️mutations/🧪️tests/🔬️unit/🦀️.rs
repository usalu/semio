//! 🧪️ Aggregated unit tests of the `mutations` leaves, one module per leaf.

mod insert_point {
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➕️insert-point/🧪️seeds/🎯️outcome/🔣️.json"));

fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
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
    semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> EquationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn expected_diff() -> EquationDiff {
    semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes")
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
    let applied = protocol::apply_diff(produced().diff(), &base).expect("insert-point applies to its committed before-snapshot");
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
    let mut snapshot = protocol::apply_diff(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "insert-point/seeds-the-empty-cloud-with-its-first-point: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `index` is a `usize`, so
/// it commits as a bare integer, while `x`/`y` are `f64` and always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/InsertPoint/index").and_then(semio_framework_pack_json::Value::as_u64), Some(0), "an index-keyed geometry verb commits its address as a bare integer");
}

/// 🎯️ The declared outcome is a clean `applied` — index 0 is within range of an empty cloud, so
/// the clamp warning must NOT fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
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
    let decoded: EquationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    let original = semio_framework_pack_json::parse((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 6, "EquationDiff emits all six artifact slots, `null` for the untouched ones");
    let (notation, results, computed) = crate::equation_children(decoded.graph.as_ref().expect("an applied diff carries the graph"), decoded.geometry.as_ref().expect("an applied diff carries the point cloud"));
    assert_eq!((decoded.notation, decoded.results, decoded.computed), (Some(notation), Some(results), Some(computed)), "the committed handles are the content addresses of the inserted state");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the insert, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let produced_snapshot = protocol::apply_diff(&expected_diff(), &before()).expect("committed diff applies to the before-snapshot");
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
    let clamped = protocol::apply_diff(outcome.diff(), &base).expect("a clamped insert still applies");
    assert_eq!(clamped.geometry.points, after_geometry().points, "clamping lands the point at the end of the cloud — here the same single slot index 0 names");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("insert", "point", "insert-point", "InsertedPoint"), "the fixture must be bound to insert-point's own descriptor");
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

mod remove_point {
//! 🧪️ `remove-point` fixture — `🚫️rejects-removing-a-point-from-an-empty-cloud`.
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

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-point/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-point/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-point/🧪️rejects/🦠️mutation/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/➖️remove-point/🧪️rejects/🎯️outcome/🔣️.json"));

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

/// ▶️ A rejected `remove-point` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    assert!(base.geometry.points.is_empty(), "rejects-removing-a-point-from-an-empty-cloud's before-snapshot must hold a point-less cloud");
    let applied = protocol::apply_diff(produced().diff(), &base).expect("an empty diff still applies cleanly");
    assert_eq!(applied, expected_after(), "remove-point/rejects-removing-a-point-from-an-empty-cloud: applied state differs from committed after-snapshot");
    assert_eq!((applied.notation, applied.results, applied.computed), (base.notation, base.results, base.computed), "a rejected remove must not mint a fresh notation/results/computed triple");
}

/// ➖️ `remove-point` addresses an ANONYMOUS, index-keyed collection by its BASE-state index, so the
/// diagnostic target is that index rendered decimal — not the `["geometry", "points", "0"]` path
/// `MutationKind::target` renders for the undo stack. `>= len` is the whole test, which is why an
/// empty cloud rejects index 0.
#[semio_framework_async_macros::async_test]
async fn a_base_state_index_past_the_end_is_reported_as_a_decimal_string() {
    let emitted = produced();
    assert_eq!(emitted.diff(), &EquationDiff::default(), "a rejecting remove-point must carry an empty diff");
    let messages = emitted.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "an out-of-range index is reported as target-missing");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Error, "remove-point has no Fatal branch at all — an absent point is a miss, not an invariant breach");
    assert_eq!(messages[0].target, vec!["0".to_string()], "the diagnostic names the bare index, not a geometry/points path");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("remove", "point", "remove-point", "RemovedPoint"),
        "the fixture must be bound to remove-point's own descriptor — `remove`, the index-keyed counterpart of `delete`"
    );
}

/// ↩️ `remove-point` inverts by re-`insert`ing the exact point it captured from BASE. With no point
/// at that index there is nothing to capture, so the inverse is empty — base-derived, never
/// payload-derived.
#[semio_framework_async_macros::async_test]
async fn inverse_has_no_point_to_reinsert() {
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &before()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "remove-point/rejects-removing-a-point-from-an-empty-cloud: a rejected remove must have no inverse steps, got {inverse:?}");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed
/// point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: EquationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "remove-point/rejects-removing-a-point-from-an-empty-cloud: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = semio_framework_pack_json::from_dsl_value(&(mutation()).to_value());
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &original), "remove-point/rejects-removing-a-point-from-an-empty-cloud: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!((*BEFORE), (*AFTER), "a rejected case commits an after-snapshot byte-identical to its before-snapshot");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("rejected"), "remove-point/rejects-removing-a-point-from-an-empty-cloud declares a rejected outcome");
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

mod move_points {
//! 🧪️ `move-points` fixture — `🧪️translates-two-points-by-the-drag-offset`.
//!
//! Source of truth is the committed JSON beside this file. Model (a) (design §20.15): the snapshots carry the graph and the
//! point cloud inline, and the `notation`/`results`/`computed` handles are the content addresses of their derivation
//! (`crate::equation_children`), kept exact by the fixture writer law. The drag moves points 0 and 2 by `(5, -2.5)` from their
//! BASE positions and leaves point 1 where it is.

use crate::standards::v1::subsets::geometry::schema::mutations::set_point_positions::{EquationPointPosition, SetPointPositions};
use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/🎯️outcome/🔣️.json"));

fn snapshot(text: &str) -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn committed_diff() -> EquationDiff {
    semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &snapshot((*BEFORE)))
}

/// ▶️ The drag carries `before` to exactly the committed `after` through exactly the committed diff, with a clean `applied`.
#[semio_framework_async_macros::async_test]
async fn applies_the_committed_diff_to_the_committed_after() {
    let outcome = produced();
    assert!(outcome.messages().is_empty(), "an in-range drag raises no diagnostic, got {:?}", outcome.messages());
    assert_eq!(outcome.diff(), &committed_diff(), "move-points/translates: produced diff differs from the committed 🔺️diff");
    let applied = protocol::apply_diff(outcome.diff(), &snapshot((*BEFORE))).expect("the drag applies");
    assert_eq!(applied, snapshot((*AFTER)), "move-points/translates: applied state differs from the committed after-snapshot");
    let declared = semio_framework_pack_json::parse((*OUTCOME), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("applied"));
}

/// ↩️ The undo is ONE absolute `set-point-positions` row carrying every moved point's BASE position, and it restores `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_is_one_absolute_row_that_restores_before() {
    let base = snapshot((*BEFORE));
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    let expected = SetPointPositions { positions: [0, 2].map(|index| EquationPointPosition { index, x: base.geometry.points[index].x, y: base.geometry.points[index].y }).to_vec() };
    assert_eq!(inverse, vec![EquationMutation::SetPointPositions(expected)]);
    let mut state = snapshot((*AFTER));
    for step in &inverse {
        let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(step, &state);
        state = protocol::apply_diff(outcome.diff(), &state).expect("the inverse applies");
    }
    assert_eq!(state, base, "move-points/translates: the inverse did not restore the before-snapshot");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation()).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &base).await;
}

/// 🔣️ Every committed document is canonical, and its handles are the content addresses of its own state.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical_and_content_addressed() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded = snapshot(text);
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&decoded.to_value()), &original), "move-points/translates: committed {label} JSON is not canonical");
        assert_eq!((decoded.notation.clone(), decoded.results.clone(), decoded.computed.clone()), crate::equation_children(&decoded.graph, &decoded.geometry), "move-points/translates: committed {label} handles are not the content addresses of its state");
    }
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&mutation().to_value()), &original), "move-points/translates: committed mutation JSON is not canonical");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

mod set_point_positions {
//! 🧪️ `set-point-positions` fixture — `🧪️restores-two-points-to-their-base-positions`.
//!
//! Source of truth is the committed JSON beside this file. Model (a) (design §20.15): the snapshots carry the graph and the
//! point cloud inline, and the `notation`/`results`/`computed` handles are the content addresses of their derivation
//! (`crate::equation_children`), kept exact by the fixture writer law. The row is the exact undo of the `move-points`
//! `🧪️translates` drag: its `before` is that drag's `after` and its `after` that drag's `before`.

use crate::{EquationDiff, EquationMutation, EquationSnapshot};
use semio_framework_value::ToValue;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/📌️set-point-positions/🧪️restores/🔺️diff/🔣️.json"));
static DRAG_BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/⬅️before/🔣️.json"));
static DRAG_AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| runtime_fixture_text("../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🎯️move-points/🧪️translates/📸️snapshot/➡️after/🔣️.json"));

fn snapshot(text: &str) -> EquationSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes")
}
fn mutation() -> EquationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &snapshot((*BEFORE)))
}

/// ▶️ The row carries `before` to exactly the committed `after` through exactly the committed diff, with a clean `applied`.
#[semio_framework_async_macros::async_test]
async fn applies_the_committed_diff_to_the_committed_after() {
    let outcome = produced();
    assert!(outcome.messages().is_empty(), "an in-range placement raises no diagnostic, got {:?}", outcome.messages());
    let committed: EquationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(outcome.diff(), &committed, "set-point-positions/restores: produced diff differs from the committed 🔺️diff");
    let applied = protocol::apply_diff(outcome.diff(), &snapshot((*BEFORE))).expect("the placement applies");
    assert_eq!(applied, snapshot((*AFTER)), "set-point-positions/restores: applied state differs from the committed after-snapshot");
}

/// 🔁️ The vector is the exact mirror of the drag it undoes, and its own inverse is one row that re-applies the drag.
#[semio_framework_async_macros::async_test]
async fn it_mirrors_the_drag_and_is_point_invertible() {
    assert_eq!((snapshot((*BEFORE)), snapshot((*AFTER))), (snapshot((*DRAG_AFTER)), snapshot((*DRAG_BEFORE))), "set-point-positions/restores undoes exactly move-points/translates");
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &snapshot((*BEFORE))).expect("valid retained mutation inverse fixture");
    assert!(matches!(inverse.as_slice(), [EquationMutation::SetPointPositions(_)]), "the inverse is ONE absolute row, got {inverse:?}");
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&snapshot((*BEFORE)), &mutation()).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &snapshot((*BEFORE))).await;
}

/// 🔣️ Every committed document is canonical, and its handles are the content addresses of its own state.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical_and_content_addressed() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded = snapshot(text);
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&decoded.to_value()), &original), "set-point-positions/restores: committed {label} JSON is not canonical");
        assert_eq!((decoded.notation.clone(), decoded.results.clone(), decoded.computed.clone()), crate::equation_children(&decoded.graph, &decoded.geometry), "set-point-positions/restores: committed {label} handles are not the content addresses of its state");
    }
    let original = semio_framework_pack_json::parse((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&semio_framework_pack_json::from_dsl_value(&mutation().to_value()), &original), "set-point-positions/restores: committed mutation JSON is not canonical");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
}

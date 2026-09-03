//! 🧪️ `insert-point` fixture — `seeds-the-empty-cloud-with-its-first-point`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ `insert-point` is the ONE equation verb with neither a rejection branch nor a no-op
//! guard: read its `🔺️diff/🦀️.rs` — an out-of-range index is CLAMPED (a Warning), never
//! refused, so every reachable outcome is an APPLIED one that re-mints the three co-derived
//! composed children through `equation_children_from_state`. That function derives each
//! `child_id` from a `std::collections::hash_map::DefaultHasher` digest of the child content, a
//! value `std` deliberately leaves unspecified — hand-forging it into the committed JSON is
//! forbidden. So the three `childId`s in `⬅️before`, `➡️after` and `🔺️diff` are DOCUMENTED
//! PLACEHOLDERS, and the fixture asks the plugin's own minting function for the real digests,
//! feeding it the hand-authored `(graph, geometry)` pair each side is claimed to hold. Everything
//! else — the handle targets, the dialects, the inline `equation`, which diff slots are filled, and
//! both geometry states — remains hand-authored and asserted verbatim.

use crate::artifacts::equation::standards::v1::subsets::geometry::schema::mutations::insert_point::mutation::InsertPoint;
use crate::artifacts::equation::standards::v1::subsets::geometry::schema::mutations::remove_point::mutation::RemovePoint;
use crate::artifacts::equation::{equation_children_from_state, equation_geometry, EquationDiff, EquationGeometry, EquationGraph, EquationMutation, EquationPoint, EquationSnapshot};
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> EquationMutation {
    pack::from_json_str(MUTATION).expect("mutation decodes")
}

/// ➕️ The committed payload, unwrapped — every state below is derived from it, never invented.
fn payload() -> InsertPoint {
    let EquationMutation::InsertPoint(payload) = mutation() else {
        panic!("seeds-the-empty-cloud-with-its-first-point's committed mutation must be an insert-point");
    };
    payload
}

/// 🕳️ Exactly the graph `equation_scene` fails soft to on a cache miss (`🔖️WorkingScene`): a
/// directed, node-less, edge-less graph with no algorithm and no seed. This fixture never changes
/// it — `insert-point` is geometry-scoped — but the child digests are derived from the PAIR, so it
/// has to be named.
fn unresolved_graph() -> EquationGraph {
    EquationGraph { directed: true, nodes: Vec::new(), edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }
}
fn base_geometry() -> EquationGeometry {
    EquationGeometry { points: Vec::new() }
}
fn after_geometry() -> EquationGeometry {
    EquationGeometry { points: vec![EquationPoint { x: payload().x, y: payload().y }] }
}

/// 🧩️ Swaps the committed placeholder handles for the ones this plugin mints for `geometry` — and,
/// as a side effect of `equation_children_from_state`, caches that scene so the snapshot
/// resolves instead of failing soft.
fn resolved(text: &str, geometry: &EquationGeometry) -> EquationSnapshot {
    let mut snapshot: EquationSnapshot = pack::from_json_str(text).expect("snapshot decodes");
    let (notation, results, computed) = equation_children_from_state(&unresolved_graph(), geometry);
    snapshot.notation = notation;
    snapshot.results = results;
    snapshot.computed = computed;
    snapshot
}

fn before() -> EquationSnapshot {
    resolved(BEFORE, &base_geometry())
}
fn expected_after() -> EquationSnapshot {
    resolved(AFTER, &after_geometry())
}

/// 🔺️ The committed diff with the same placeholder-for-digest substitution. Which slots are `Some`
/// and which are `None` comes from the committed JSON alone.
fn expected_diff() -> EquationDiff {
    let mut diff: EquationDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let (notation, results, computed) = equation_children_from_state(&unresolved_graph(), &after_geometry());
    diff.notation = Some(notation);
    diff.results = Some(results);
    diff.computed = Some(computed);
    diff
}

fn produced() -> protocol::MutationOutcome<EquationDiff> {
    <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Inserting at index 0 of an empty cloud carries `before` to exactly the committed `after`: one
/// point at `(5, 6)`, the graph untouched, and the inline `equation` untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    assert!(equation_geometry(&base).points.is_empty(), "seeds-the-empty-cloud-with-its-first-point's base cloud must be empty for `index: 0` to be the exact end of the cloud");
    let applied = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(produced().diff(), &base).expect("insert-point applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "insert-point/seeds-the-empty-cloud-with-its-first-point: applied state differs from committed after-snapshot");
    assert_eq!(equation_geometry(&applied).points, after_geometry().points, "the inserted point must land verbatim at the payload's coordinates");
    assert_eq!(applied.equation, base.equation, "insert-point is geometry-scoped — it never touches the inline equation slot");
}

/// ↩️ `insert-point`'s `index` is FINAL-state, so its undo is a `remove-point` at the SAME index —
/// clamped against BASE's length, which here is 0.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <EquationMutation as protocol::Mutation<EquationSnapshot>>::inverse(&mutation(), &base);
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
        let decoded: EquationSnapshot = pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("snapshot reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed {label} JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&(mutation()).to_value());
    let original = pack::parse_json(MUTATION).expect("mutation reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed mutation JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.pointer("/InsertPoint/index").and_then(pack::JsonValue::as_u64), Some(0), "an index-keyed geometry verb commits its address as a bare integer");
}

/// 🎯️ The declared outcome is a clean `applied` — index 0 is within range of an empty cloud, so
/// the clamp warning must NOT fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(pack::JsonValue::as_str), Some("applied"), "insert-point/seeds-the-empty-cloud-with-its-first-point declares an applied outcome");
    let emitted = produced();
    assert!(emitted.messages().is_empty(), "an in-range insert raises no diagnostic at all, got {:?}", emitted.messages());
    assert!(outcome.get("messages").is_none(), "a clean applied outcome commits no messages array");
}

/// 🔺️ The produced delta is exactly the committed one: all three co-derived children replaced
/// together, and `equation` plus every config-lane slot left `null`. This is what pins that a
/// geometry insert may not smuggle an equation or camera edit alongside it.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = produced();
    assert_eq!(outcome.diff(), &expected_diff(), "insert-point/seeds-the-empty-cloud-with-its-first-point: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert!(outcome.diff().equation.is_none() && outcome.diff().camera_x.is_none() && outcome.diff().locale.is_none(), "insert-point fills only the three composed-child slots");
}

/// 🔣️ The committed diff is itself canonical and decodes to `EquationDiff`, whose container
/// `#[serde(default)]` carries no per-field `skip_serializing_if` — all eight slots are present.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EquationDiff = pack::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = pack::json_from_dsl_value(&decoded.to_value());
    let original = pack::parse_json(DIFF).expect("committed diff reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 8, "EquationDiff emits all eight slots, `null` for the untouched ones");
    assert_eq!(original.pointer("/results/target/artifactId").and_then(pack::JsonValue::as_str), Some("equation-table"), "the results slot always targets this plugin's `table` child, whatever its digest");
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
/// `remove-point`/`move-point`, which reject an out-of-range index outright.
#[semio_framework_async_macros::async_test]
async fn an_out_of_range_index_clamps_instead_of_rejecting() {
    let base = before();
    let past_the_end = EquationMutation::InsertPoint(InsertPoint { index: 9, x: payload().x, y: payload().y });
    let outcome = <EquationMutation as protocol::Mutation<EquationSnapshot>>::diff(&past_the_end, &base);
    let messages = outcome.messages();
    assert_eq!(messages.len(), 1, "a clamped insert raises exactly one diagnostic, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.clamped", "an out-of-range insert index is clamped, never reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "clamping is a Warning — insert-point has no Error or Fatal branch at all");
    let clamped = <EquationDiff as protocol::MutationDiff<EquationSnapshot>>::apply(outcome.diff(), &base).expect("a clamped insert still applies");
    assert_eq!(equation_geometry(&clamped).points, after_geometry().points, "clamping lands the point at the end of the cloud — here the same single slot index 0 names");
    let semantics = <EquationMutation as protocol::SemanticMutation<EquationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("insert", "point", "insert-point", "InsertedPoint"), "the fixture must be bound to insert-point's own descriptor");
}

//! 🧪️ `insert-point` fixture — `seeds-the-empty-cloud-with-its-first-point`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! ⚠️ `insert-point` is the ONE mathematical verb with neither a rejection branch nor a no-op
//! guard: read its `🔺️diff/🦀️component.rs` — an out-of-range index is CLAMPED (a Warning), never
//! refused, so every reachable outcome is an APPLIED one that re-mints the three co-derived
//! composed children through `mathematical_children_from_state`. That function derives each
//! `child_id` from a `std::collections::hash_map::DefaultHasher` digest of the child content, a
//! value `std` deliberately leaves unspecified — hand-forging it into the committed JSON is
//! forbidden. So the three `childId`s in `⬅️before`, `➡️after` and `🔺️diff` are DOCUMENTED
//! PLACEHOLDERS, and the fixture asks the plugin's own minting function for the real digests,
//! feeding it the hand-authored `(graph, geometry)` pair each side is claimed to hold. Everything
//! else — the handle targets, the dialects, the inline `equation`, which diff slots are filled, and
//! both geometry states — remains hand-authored and asserted verbatim.

use crate::artifacts::mathematical::mutations::insert_point::mutation::InsertPoint;
use crate::artifacts::mathematical::mutations::remove_point::mutation::RemovePoint;
use crate::artifacts::mathematical::{
    mathematical_children_from_state, mathematical_geometry, MathematicalDiff, MathematicalGeometry, MathematicalGraph, MathematicalMutation, MathematicalPoint, MathematicalSnapshot,
};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> MathematicalMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ➕️ The committed payload, unwrapped — every state below is derived from it, never invented.
fn payload() -> InsertPoint {
    let MathematicalMutation::InsertPoint(payload) = mutation() else {
        panic!("seeds-the-empty-cloud-with-its-first-point's committed mutation must be an insert-point");
    };
    payload
}

/// 🕳️ Exactly the graph `mathematical_scene` fails soft to on a cache miss (`🔖️WorkingScene`): a
/// directed, node-less, edge-less graph with no algorithm and no seed. This fixture never changes
/// it — `insert-point` is geometry-scoped — but the child digests are derived from the PAIR, so it
/// has to be named.
fn unresolved_graph() -> MathematicalGraph {
    MathematicalGraph { directed: true, nodes: Vec::new(), edges: Vec::new(), algorithm: String::new(), algorithm_seed: None }
}
fn base_geometry() -> MathematicalGeometry {
    MathematicalGeometry { points: Vec::new() }
}
fn after_geometry() -> MathematicalGeometry {
    MathematicalGeometry { points: vec![MathematicalPoint { x: payload().x, y: payload().y }] }
}

/// 🧩️ Swaps the committed placeholder handles for the ones this plugin mints for `geometry` — and,
/// as a side effect of `mathematical_children_from_state`, caches that scene so the snapshot
/// resolves instead of failing soft.
fn resolved(text: &str, geometry: &MathematicalGeometry) -> MathematicalSnapshot {
    let mut snapshot: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
    let (notation, results, computed) = mathematical_children_from_state(&unresolved_graph(), geometry);
    snapshot.notation = notation;
    snapshot.results = results;
    snapshot.computed = computed;
    snapshot
}

fn before() -> MathematicalSnapshot {
    resolved(BEFORE, &base_geometry())
}
fn expected_after() -> MathematicalSnapshot {
    resolved(AFTER, &after_geometry())
}

/// 🔺️ The committed diff with the same placeholder-for-digest substitution. Which slots are `Some`
/// and which are `None` comes from the committed JSON alone.
fn expected_diff() -> MathematicalDiff {
    let mut diff: MathematicalDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let (notation, results, computed) = mathematical_children_from_state(&unresolved_graph(), &after_geometry());
    diff.notation = Some(notation);
    diff.results = Some(results);
    diff.computed = Some(computed);
    diff
}

fn produced() -> protocol::MutationOutcome<MathematicalDiff> {
    <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Inserting at index 0 of an empty cloud carries `before` to exactly the committed `after`: one
/// point at `(5, 6)`, the graph untouched, and the inline `equation` untouched.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    assert!(mathematical_geometry(&base).points.is_empty(), "seeds-the-empty-cloud-with-its-first-point's base cloud must be empty for `index: 0` to be the exact end of the cloud");
    let applied = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("insert-point applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "insert-point/seeds-the-empty-cloud-with-its-first-point: applied state differs from committed after-snapshot");
    assert_eq!(mathematical_geometry(&applied).points, after_geometry().points, "the inserted point must land verbatim at the payload's coordinates");
    assert_eq!(applied.equation, base.equation, "insert-point is geometry-scoped — it never touches the inline equation slot");
}

/// ↩️ `insert-point`'s `index` is FINAL-state, so its undo is a `remove-point` at the SAME index —
/// clamped against BASE's length, which here is 0.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let inverse = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse, vec![MathematicalMutation::RemovePoint(RemovePoint { index: 0 })], "insert-point inverts to a remove-point at the same index, got {inverse:?}");
    let mut snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(produced().diff(), &base).expect("forward applies");
    for step in &inverse {
        let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(step, &snapshot);
        snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(outcome.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "insert-point/seeds-the-empty-cloud-with-its-first-point: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical. `index` is a `usize`, so
/// it commits as a bare integer, while `x`/`y` are `f64` and always re-encode with a `.0`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: MathematicalSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-point/seeds-the-empty-cloud-with-its-first-point: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "insert-point/seeds-the-empty-cloud-with-its-first-point: committed mutation JSON is not canonical");
    assert_eq!(original.pointer("/InsertPoint/index").and_then(serde_json::Value::as_u64), Some(0), "an index-keyed geometry verb commits its address as a bare integer");
}

/// 🎯️ The declared outcome is a clean `applied` — index 0 is within range of an empty cloud, so
/// the clamp warning must NOT fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "insert-point/seeds-the-empty-cloud-with-its-first-point declares an applied outcome");
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
    assert_eq!(outcome.diff(), &expected_diff(), "insert-point/seeds-the-empty-cloud-with-its-first-point: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert!(outcome.diff().equation.is_none() && outcome.diff().camera_x.is_none() && outcome.diff().locale.is_none(), "insert-point fills only the three composed-child slots");
}

/// 🔣️ The committed diff is itself canonical and decodes to `MathematicalDiff`, whose container
/// `#[serde(default)]` carries no per-field `skip_serializing_if` — all eight slots are present.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: MathematicalDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "insert-point/seeds-the-empty-cloud-with-its-first-point: committed diff JSON is not canonical");
    assert_eq!(original.as_object().expect("the diff is a JSON object").len(), 8, "MathematicalDiff emits all eight slots, `null` for the untouched ones");
    assert_eq!(original.pointer("/results/target/artifactId").and_then(serde_json::Value::as_str), Some("mathematical-table"), "the results slot always targets this plugin's `table` child, whatever its digest");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of the insert, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let produced_snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(&expected_diff(), &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced_snapshot, expected_after(), "insert-point/seeds-the-empty-cloud-with-its-first-point: committed diff did not carry before to after");
}

/// ➕️ The branch this verb owns and no other geometry verb has: an out-of-range index is CLAMPED to
/// the end of the cloud and reported as a Warning `mutation.clamped` — it still applies, unlike
/// `remove-point`/`move-point`, which reject an out-of-range index outright.
#[semio_framework_async_macros::async_test]
async fn an_out_of_range_index_clamps_instead_of_rejecting() {
    let base = before();
    let past_the_end = MathematicalMutation::InsertPoint(InsertPoint { index: 9, x: payload().x, y: payload().y });
    let outcome = <MathematicalMutation as protocol::Mutation<MathematicalSnapshot>>::diff(&past_the_end, &base);
    let messages = outcome.messages();
    assert_eq!(messages.len(), 1, "a clamped insert raises exactly one diagnostic, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.clamped", "an out-of-range insert index is clamped, never reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "clamping is a Warning — insert-point has no Error or Fatal branch at all");
    let clamped = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(outcome.diff(), &base).expect("a clamped insert still applies");
    assert_eq!(mathematical_geometry(&clamped).points, after_geometry().points, "clamping lands the point at the end of the cloud — here the same single slot index 0 names");
    let semantics = <MathematicalMutation as protocol::SemanticMutation<MathematicalSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("insert", "point", "insert-point", "InsertedPoint"), "the fixture must be bound to insert-point's own descriptor");
}

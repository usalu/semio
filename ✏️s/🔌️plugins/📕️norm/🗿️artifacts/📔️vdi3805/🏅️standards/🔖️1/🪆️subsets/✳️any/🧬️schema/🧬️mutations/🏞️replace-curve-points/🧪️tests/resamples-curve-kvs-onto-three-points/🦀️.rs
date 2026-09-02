//! 🧪️ `replace-curve-points` fixture — `resamples-curve-kvs-onto-three-points`.
//!
//! `curve.kvs`'s two-point ramp is resampled onto three points by inserting the 50 % midpoint at 2.25 m3/h. Both units are left as-is; only `points` inside the republished curves map changes.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Vdi3805Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Vdi3805Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Vdi3805Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn applied() -> Vdi3805Snapshot {
    let base = before();
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &base);
    <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(raised.diff(), &base).expect("replace-curve-points applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let snapshot = applied();
    let points = &snapshot.curves["curve.kvs"].points;
    assert_eq!(points.len(), 3, "replace-curve-points/resamples-curve-kvs-onto-three-points: the curve must carry three points afterwards");
    assert_eq!(points[1].x, 50.0, "replace-curve-points/resamples-curve-kvs-onto-three-points: the inserted midpoint must sit at 50 % travel");
    assert_eq!(points[1].y, 2.25, "replace-curve-points/resamples-curve-kvs-onto-three-points: the inserted midpoint must read 2.25 m3/h");
    assert_eq!(snapshot.curves["curve.kvs"].x_unit, before().curves["curve.kvs"].x_unit, "replace-curve-points/resamples-curve-kvs-onto-three-points: resampling must not rewrite the abscissa unit");
    assert_eq!(snapshot, expected_after(), "replace-curve-points/resamples-curve-kvs-onto-three-points: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "replace-curve-points/resamples-curve-kvs-onto-three-points: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(step, &snapshot);
        snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-curve-points/resamples-curve-kvs-onto-three-points: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Vdi3805Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-curve-points/resamples-curve-kvs-onto-three-points: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-curve-points/resamples-curve-kvs-onto-three-points: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `replace-curve-points`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "replace-curve-points/resamples-curve-kvs-onto-three-points: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "replace-curve-points/resamples-curve-kvs-onto-three-points: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "replace-curve-points/resamples-curve-kvs-onto-three-points: a rejected mutation must leave the snapshot untouched"),
        other => panic!("replace-curve-points/resamples-curve-kvs-onto-three-points: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `replace-curve-points` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert_eq!(raised_diff.curves.as_ref().map(|map| map["curve.kvs"].points.len()), Some(3), "replace-curve-points/resamples-curve-kvs-onto-three-points: the diff must publish the curves map with the three-point list");
    assert!(raised_diff.manufacturer_file.is_none(), "replace-curve-points/resamples-curve-kvs-onto-three-points: a curve resample is not a header edit");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-curve-points/resamples-curve-kvs-onto-three-points: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `Vdi3805Diff` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Vdi3805Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-curve-points/resamples-curve-kvs-onto-three-points: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `replace-curve-points` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Vdi3805Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-curve-points/resamples-curve-kvs-onto-three-points: committed diff did not carry before to after");
}

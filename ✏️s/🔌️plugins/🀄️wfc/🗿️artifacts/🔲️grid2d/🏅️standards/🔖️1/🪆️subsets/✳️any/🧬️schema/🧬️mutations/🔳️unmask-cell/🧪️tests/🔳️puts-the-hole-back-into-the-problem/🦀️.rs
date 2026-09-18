//! 🧪️ `unmask-cell` fixture — `🔳️puts-the-hole-back-into-the-problem`.
//!
//! Unmasking returns the cell to the problem; it comes back unpinned, because the mask cascaded its pin away.
//!
//! Source of truth is the committed JSON quintet beside this file; this module only replays it.

use crate::diff::Grid2dDiff;
use crate::mutations::{apply_grid2d_mutation, inverse_grid2d_mutation, Grid2dMutation};
use crate::schema::snapshot::Grid2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔳️unmask-cell/🔳️puts-the-hole-back-into-the-problem/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔳️unmask-cell/🔳️puts-the-hole-back-into-the-problem/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔳️unmask-cell/🔳️puts-the-hole-back-into-the-problem/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔳️unmask-cell/🔳️puts-the-hole-back-into-the-problem/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔳️unmask-cell/🔳️puts-the-hole-back-into-the-problem/🎯️outcome/🔣️.json");

fn before() -> Grid2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Grid2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Grid2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_grid2d_mutation(&mut snapshot, &mutation()).expect("unmask-cell applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "unmask-cell/🔳️puts-the-hole-back-into-the-problem: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — VALUE and POSITION.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_grid2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_grid2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_grid2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "unmask-cell/🔳️puts-the-hole-back-into-the-problem: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Grid2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "unmask-cell/🔳️puts-the-hole-back-into-the-problem: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "unmask-cell/🔳️puts-the-hole-back-into-the-problem: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <Grid2dMutation as protocol::Mutation<Grid2dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "unmask-cell/🔳️puts-the-hole-back-into-the-problem: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = apply_grid2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "unmask-cell/🔳️puts-the-hole-back-into-the-problem: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "unmask-cell/🔳️puts-the-hole-back-into-the-problem: declared applied but the snapshot came back unchanged");
        }
        "rejected" => assert_eq!(snapshot, before(), "unmask-cell/🔳️puts-the-hole-back-into-the-problem: a rejected mutation must leave the snapshot untouched"),
        other => panic!("unmask-cell/🔳️puts-the-hole-back-into-the-problem: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins WHICH lanes
/// `unmask-cell` is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Grid2dMutation as protocol::Mutation<Grid2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "unmask-cell/🔳️puts-the-hole-back-into-the-problem: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Grid2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "unmask-cell/🔳️puts-the-hole-back-into-the-problem: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Grid2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <Grid2dDiff as protocol::MutationDiff<Grid2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "unmask-cell/🔳️puts-the-hole-back-into-the-problem: committed diff did not carry before to after");
}

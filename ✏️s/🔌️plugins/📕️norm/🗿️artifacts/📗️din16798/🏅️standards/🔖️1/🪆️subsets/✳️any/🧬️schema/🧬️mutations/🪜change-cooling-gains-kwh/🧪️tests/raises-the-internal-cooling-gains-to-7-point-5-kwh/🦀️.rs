//! 🧪️ `change-cooling-gains-kwh` fixture — `raises-the-internal-cooling-gains-to-7-point-5-kwh`.
//!
//! Raises the internal and solar gains entering the cooling balance. The utilization factor applied to them is not re-derived. Guards read off this leaf's own diff builder: `is_finite` then `==` no-op.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Din16798Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Din16798Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Din16798Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-cooling-gains-kwh` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("change-cooling-gains-kwh applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.cooling_gains_kwh, 7.5, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: coolingGainsKwh did not land on 7.5");
    assert_eq!(
        snapshot.cooling_utilization_factor,
        before().cooling_utilization_factor,
        "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: coolingUtilizationFactor must stay exactly as the before-snapshot had it — change-cooling-gains-kwh owns coolingGainsKwh and nothing else"
    );
}

/// ↩️ Applying `change-cooling-gains-kwh` and then its own inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: undoing one coolingGainsKwh edit is exactly one step");
    assert_eq!(snapshot.cooling_gains_kwh, 5.0, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: the undo step must put coolingGainsKwh back to 5.0");
}

/// 🔣️ Both committed snapshots and this leaf's committed mutation payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din16798Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `change-cooling-gains-kwh`'s own diff builder
/// raises for this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: raised diagnostics differ from the committed 🎯️outcome messages");
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `change-cooling-gains-kwh` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields of `Din16798Snapshot` this leaf is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(raised.diff().cooling_gains_kwh, Some(7.5), "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: the sparse delta must carry coolingGainsKwh = 7.5");
    assert!(
        raised.diff().cooling_utilization_factor.is_none(),
        "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: the sparse delta must leave coolingUtilizationFactor unset — a delta that rewrote it would be a bug this assertion exists to catch"
    );
}

/// 🔣️ The committed diff is itself canonical and decodes to `Din16798Diff`. Its
/// `selectedCheckIndex` is an `Option<Option<u32>>` and so cannot distinguish `None` from
/// `Some(None)` across a JSON round trip — `change-cooling-gains-kwh` never writes it, so the committed
/// `null` is unambiguously `None` here and the fixed point holds.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Din16798Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.selected_check_index.is_none(), "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: change-cooling-gains-kwh is an artifact-lane edit and must never carry the presence-lane selectedCheckIndex");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-cooling-gains-kwh` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Din16798Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Din16798Diff as protocol::MutationDiff<Din16798Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-cooling-gains-kwh/raises-the-internal-cooling-gains-to-7-point-5-kwh: committed diff did not carry before to after");
}

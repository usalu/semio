//! 🧪️ `change-renewable-kwh` fixture — `raises-the-on-site-renewable-yield-to-2250-kwh`.
//!
//! The on-site renewable yield credited against primary energy goes 1500 → 2250 kWh (a larger PV array). Neither the system losses it offsets nor the annual limit it is judged against move.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::din18599::{Din18599Diff, Din18599Mutation, Din18599Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Din18599Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Din18599Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Din18599Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn applied() -> Din18599Snapshot {
    let base = before();
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &base);
    <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &base).expect("change-renewable-kwh applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.renewable_kwh, 2250.0, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: the renewable yield must be 2250 kWh");
    assert_eq!(snapshot.annual_limit_kwh, before().annual_limit_kwh, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: the compliance limit is not lowered by generating more");
    assert_eq!(snapshot, expected_after(), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(step, &snapshot);
        snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din18599Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `change-renewable-kwh`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: a rejected mutation must leave the snapshot untouched"),
        other => panic!("change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `change-renewable-kwh` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert_eq!(raised_diff.renewable_kwh, Some(2250.0), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: the diff must publish renewableKwh = 2250");
    assert!(raised_diff.system_losses_kwh.is_none(), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: systemLossesKwh must stay null");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `Din18599Diff` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Din18599Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-renewable-kwh` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Din18599Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-renewable-kwh/raises-the-on-site-renewable-yield-to-2250-kwh: committed diff did not carry before to after");
}

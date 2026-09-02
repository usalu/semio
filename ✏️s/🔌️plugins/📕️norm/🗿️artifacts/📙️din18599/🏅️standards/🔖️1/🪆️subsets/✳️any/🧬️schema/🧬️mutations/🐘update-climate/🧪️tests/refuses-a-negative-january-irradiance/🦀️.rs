//! 🧪️ `update-climate` fixture — `refuses-a-negative-january-irradiance`.
//!
//! `update-climate` is the only DIN V 18599 mutation that writes the composed `climate` child slot, and the only one whose payload carries real data (a literal twelve-month `MonthlyClimate`) rather than a handle. Its `mutation.invariant` guard runs BEFORE the no-op check and refuses any non-finite value or any negative global irradiance: here January's `g_h_w_m2` is -30 W/m2, so the builder returns a FATAL outcome with the default all-null diff and the snapshot — climate handle included — is left untouched.
//!
//! Source of truth is the committed JSON quartet beside this file plus contract D6's empty
//! `🔺️diff/🚫️.absent` marker (ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The
//! `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio` encodings are derived from it by
//! `fixtures generate` and are asserted by the shared codec-matrix harness, not here.

use crate::artifacts::din18599::{Din18599Diff, Din18599Mutation, Din18599Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
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
    <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &base).expect("an empty rejection diff still applies cleanly")
}

/// ▶️ The rejected mutation carries `before` to exactly the committed `after` — which, for a
/// rejection, is `before` verbatim.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.climate.child_id, before().climate.child_id, "update-climate/refuses-a-negative-january-irradiance: the composed climate child handle must not be re-minted by a refused payload");
    assert_eq!(snapshot.climate.target.artifact_id, "din18599-climate", "update-climate/refuses-a-negative-january-irradiance: the child slot must still point at the same table artifact");
    assert_eq!(snapshot.solar_gains_kwh, before().solar_gains_kwh, "update-climate/refuses-a-negative-january-irradiance: no climate-derived scalar may be recomputed from a refused payload");
    assert_eq!(snapshot, expected_after(), "update-climate/refuses-a-negative-january-irradiance: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "update-climate/refuses-a-negative-january-irradiance: a rejected case's after-snapshot must be its before-snapshot verbatim");
}

/// ↩️ A rejection changes nothing, and replaying its inverse on top still lands on `before`.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let mut snapshot = applied();
    for step in &<Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::inverse(&mutation, &base) {
        let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(step, &snapshot);
        snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-climate/refuses-a-negative-january-irradiance: replaying the inverse of a rejection must still leave the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din18599Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-climate/refuses-a-negative-january-irradiance: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-climate/refuses-a-negative-january-irradiance: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — the `rejected` status AND the exact diagnostic `update-climate`'s own
/// diff builder raises — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "rejected", "update-climate/refuses-a-negative-january-irradiance: this case exists to pin a rejection");
    let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("a rejected outcome carries a machine-readable code");
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
    assert_eq!(produced, declared, "update-climate/refuses-a-negative-january-irradiance: raised diagnostics differ from the committed 🎯️outcome messages");
    assert!(produced.iter().any(|(_, raised_code)| raised_code == code), "update-climate/refuses-a-negative-january-irradiance: the outcome's declared code must be one the builder actually raised");
    assert_eq!(applied(), before(), "update-climate/refuses-a-negative-january-irradiance: a rejected mutation must leave the snapshot untouched");
}

/// 🔺️ A rejection publishes NO delta: the raised diff must be the diff type's own `default()`, and
/// the case must carry contract D6's empty `🚫️.absent` marker instead of an invented
/// empty patch.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation(), &before());
    assert_eq!(raised.diff(), &Din18599Diff::default(), "update-climate/refuses-a-negative-january-irradiance: a fatal rejection must publish the default (all-null) diff");
    assert!(DIFF_ABSENT.is_empty(), "update-climate/refuses-a-negative-january-irradiance: 🔺️diff/🚫️.absent must be a zero-byte marker");
}

/// 🔣️ There is no committed diff JSON to be canonical — the absent marker is the committed form,
/// and it must stay empty.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    assert_eq!(DIFF_ABSENT.len(), 0, "update-climate/refuses-a-negative-january-irradiance: the absence marker must carry no bytes at all");
    let produced = serde_json::to_value(Din18599Diff::default()).expect("default diff encodes");
    assert!(produced.as_object().is_some_and(|fields| fields.values().all(serde_json::Value::is_null)), "update-climate/refuses-a-negative-january-irradiance: the default diff must serialize as an all-null object, never as an omitted-field one");
}

/// 🩹 Applying the rejection's own (default) diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let produced = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&Din18599Diff::default(), &before()).expect("the default diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-climate/refuses-a-negative-january-irradiance: the rejection's empty diff must leave before exactly as committed in after");
}

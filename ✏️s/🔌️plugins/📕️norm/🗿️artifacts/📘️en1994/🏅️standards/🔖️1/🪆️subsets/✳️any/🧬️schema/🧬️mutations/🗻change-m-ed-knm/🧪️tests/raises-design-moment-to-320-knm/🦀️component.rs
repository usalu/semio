//! 🧪️ `change-m-ed-knm` fixture — `raises-design-moment-to-320-knm` (EN 1994 composite).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1994Snapshot {
    serde_json::from_str(BEFORE).expect("change-m-ed-knm/raises-design-moment-to-320-knm: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-m-ed-knm/raises-design-moment-to-320-knm: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-m-ed-knm/raises-design-moment-to-320-knm: mutation decodes")
}

/// ▶️ `change-m-ed-knm` carries `m_ed_knm` from 240.0 to 320.0 and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-m-ed-knm/raises-design-moment-to-320-knm: mutation applies to its committed before-snapshot");
    assert_eq!(produced.m_ed_knm, 320.0, "change-m-ed-knm/raises-design-moment-to-320-knm: `m_ed_knm` must read 320.0 after the mutation");
    assert_eq!(produced.v_ed_kn, base.v_ed_kn, "change-m-ed-knm/raises-design-moment-to-320-knm: `v_ed_kn` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-m-ed-knm/raises-design-moment-to-320-knm: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `m_ed_knm` (240.0) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-m-ed-knm/raises-design-moment-to-320-knm: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-m-ed-knm/raises-design-moment-to-320-knm: inverse step applies");
    }
    assert_eq!(snapshot.m_ed_knm, base.m_ed_knm, "change-m-ed-knm/raises-design-moment-to-320-knm: inverse must put `m_ed_knm` back to 240.0");
    assert_eq!(snapshot, base, "change-m-ed-knm/raises-design-moment-to-320-knm: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-m-ed-knm/raises-design-moment-to-320-knm: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-m-ed-knm/raises-design-moment-to-320-knm: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-m-ed-knm/raises-design-moment-to-320-knm: snapshot reparses");
        assert_eq!(reencoded, original, "change-m-ed-knm/raises-design-moment-to-320-knm: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-m-ed-knm/raises-design-moment-to-320-knm: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-m-ed-knm/raises-design-moment-to-320-knm: mutation reparses");
    assert_eq!(reencoded, original, "change-m-ed-knm/raises-design-moment-to-320-knm: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean 240.0→320.0 edit of `m_ed_knm` raises no diagnostic.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-m-ed-knm/raises-design-moment-to-320-knm: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-m-ed-knm/raises-design-moment-to-320-knm: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-m-ed-knm/raises-design-moment-to-320-knm: changing `m_ed_knm` away from 240.0 must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-m-ed-knm/raises-design-moment-to-320-knm: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `mEdKnm` set, every other field left null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().m_ed_knm, Some(320.0), "change-m-ed-knm/raises-design-moment-to-320-knm: the diff must carry `m_ed_knm` = 320.0");
    assert!(outcome.diff().v_ed_kn.is_none(), "change-m-ed-knm/raises-design-moment-to-320-knm: the diff must leave `v_ed_kn` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-m-ed-knm/raises-design-moment-to-320-knm: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-m-ed-knm/raises-design-moment-to-320-knm: committed diff decodes");
    assert_eq!(produced, committed, "change-m-ed-knm/raises-design-moment-to-320-knm: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-m-ed-knm/raises-design-moment-to-320-knm: committed diff decodes");
    assert_eq!(decoded.m_ed_knm, Some(320.0), "change-m-ed-knm/raises-design-moment-to-320-knm: the committed diff must name `m_ed_knm` = 320.0");
    let reencoded = serde_json::to_value(&decoded).expect("change-m-ed-knm/raises-design-moment-to-320-knm: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-m-ed-knm/raises-design-moment-to-320-knm: committed diff reparses");
    assert_eq!(reencoded, original, "change-m-ed-knm/raises-design-moment-to-320-knm: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the 320.0 `m_ed_knm` edit is complete on its own.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-m-ed-knm/raises-design-moment-to-320-knm: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-m-ed-knm/raises-design-moment-to-320-knm: committed diff applies to the before-snapshot");
    assert_eq!(produced.m_ed_knm, 320.0, "change-m-ed-knm/raises-design-moment-to-320-knm: the committed diff must set `m_ed_knm` to 320.0");
    assert_eq!(produced, expected_after(), "change-m-ed-knm/raises-design-moment-to-320-knm: committed diff did not carry before to after");
}

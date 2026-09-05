//! 🧪️ `change-annex` fixture — `🌐️switches-national-annex-to-en` (EN 1994 composite).
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1994Snapshot {
    serde_json::from_str(BEFORE).expect("change-annex/switches-national-annex-to-en: before snapshot decodes")
}
fn expected_after() -> En1994Snapshot {
    serde_json::from_str(AFTER).expect("change-annex/switches-national-annex-to-en: after snapshot decodes")
}
fn mutation() -> En1994Mutation {
    serde_json::from_str(MUTATION).expect("change-annex/switches-national-annex-to-en: mutation decodes")
}

/// ▶️ `change-annex` carries `annex` from AnnexChoice::De to AnnexChoice::En and lands on the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = outcome.diff().apply(&base).expect("change-annex/switches-national-annex-to-en: mutation applies to its committed before-snapshot");
    assert_eq!(produced.annex, crate::document::AnnexChoice::En, "change-annex/switches-national-annex-to-en: `annex` must read AnnexChoice::En after the mutation");
    assert_eq!(produced.m_ed_knm, base.m_ed_knm, "change-annex/switches-national-annex-to-en: `m_ed_knm` is not addressed by this mutation and must survive untouched");
    assert_eq!(produced, expected_after(), "change-annex/switches-national-annex-to-en: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse re-states the pre-edit `annex` (AnnexChoice::De) and restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let outcome = forward.diff(&base);
    let mut snapshot = outcome.diff().apply(&base).expect("change-annex/switches-national-annex-to-en: forward applies");
    for step in &forward.inverse(&base) {
        let step_outcome = step.diff(&snapshot);
        snapshot = step_outcome.diff().apply(&snapshot).expect("change-annex/switches-national-annex-to-en: inverse step applies");
    }
    assert_eq!(snapshot.annex, base.annex, "change-annex/switches-national-annex-to-en: inverse must put `annex` back to AnnexChoice::De");
    assert_eq!(snapshot, base, "change-annex/switches-national-annex-to-en: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1994Snapshot = serde_json::from_str(text).expect("change-annex/switches-national-annex-to-en: snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("change-annex/switches-national-annex-to-en: snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("change-annex/switches-national-annex-to-en: snapshot reparses");
        assert_eq!(reencoded, original, "change-annex/switches-national-annex-to-en: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("change-annex/switches-national-annex-to-en: mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("change-annex/switches-national-annex-to-en: mutation reparses");
    assert_eq!(reencoded, original, "change-annex/switches-national-annex-to-en: committed mutation JSON is not canonical");
}

/// 🎯️ The declared `applied` outcome holds — a clean AnnexChoice::De→AnnexChoice::En edit of `annex` raises no diagnostic.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("change-annex/switches-national-annex-to-en: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-annex/switches-national-annex-to-en: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "change-annex/switches-national-annex-to-en: changing `annex` away from AnnexChoice::De must not warn `mutation.no-op` nor fail `mutation.invariant`");
    assert!(outcome.diff().apply(&base).is_ok(), "change-annex/switches-national-annex-to-en: declared applied but the diff was rejected");
}

/// 🔺️ The sparse delta is exactly the committed diff: `annex` set, every other field left null.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    assert_eq!(outcome.diff().annex, Some(crate::document::AnnexChoice::En), "change-annex/switches-national-annex-to-en: the diff must carry `annex` = AnnexChoice::En");
    assert!(outcome.diff().m_ed_knm.is_none(), "change-annex/switches-national-annex-to-en: the diff must leave `m_ed_knm` unset");
    let produced = serde_json::to_value(outcome.diff()).expect("change-annex/switches-national-annex-to-en: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("change-annex/switches-national-annex-to-en: committed diff decodes");
    assert_eq!(produced, committed, "change-annex/switches-national-annex-to-en: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is canonical and decodes to `En1994Diff`.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-annex/switches-national-annex-to-en: committed diff decodes");
    assert_eq!(decoded.annex, Some(crate::document::AnnexChoice::En), "change-annex/switches-national-annex-to-en: the committed diff must name `annex` = AnnexChoice::En");
    let reencoded = serde_json::to_value(&decoded).expect("change-annex/switches-national-annex-to-en: diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("change-annex/switches-national-annex-to-en: committed diff reparses");
    assert_eq!(reencoded, original, "change-annex/switches-national-annex-to-en: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff to `before` yields `after` — the AnnexChoice::En `annex` edit is complete on its own.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let base = before();
    let decoded: En1994Diff = serde_json::from_str(DIFF).expect("change-annex/switches-national-annex-to-en: committed diff decodes");
    let produced = decoded.apply(&base).expect("change-annex/switches-national-annex-to-en: committed diff applies to the before-snapshot");
    assert_eq!(produced.annex, crate::document::AnnexChoice::En, "change-annex/switches-national-annex-to-en: the committed diff must set `annex` to AnnexChoice::En");
    assert_eq!(produced, expected_after(), "change-annex/switches-national-annex-to-en: committed diff did not carry before to after");
}

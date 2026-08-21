//! 🧪️ `change-z-mm3` fixture — `raises-the-section-modulus-to-9500000-mm3`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-z-mm3` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1996Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1996Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1996Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-z-mm3` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising Z from 8000000.0 mm³ to 9500000.0 mm³ rewrites `z_mm3` alone — the design moment it resists is
/// untouched.
#[semio_framework_async_macros::async_test]
async fn raises_the_section_modulus_to_9500000_mm3() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-z-mm3 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.z_mm3, 9500000.0, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: z_mm3 must read 9500000.0 mm³ once the change lands");
    assert_eq!(applied.m_ed_knm, before().m_ed_knm, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the design moment is the action side of the flexure check and must not follow the section geometry");
}

/// ↩️ `change-z-mm3`'s inverse reads the OLD 8000000.0 mm³ out of BASE, so replaying it puts the 8000000.0 mm³
/// back on `z_mm3`.
#[semio_framework_async_macros::async_test]
async fn restoring_8000000_mm3_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-z-mm3 applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the inverse of one change-z-mm3 is exactly one change-z-mm3 back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-z-mm3 inverse step applies");
    }
    assert_eq!(snapshot.z_mm3, base.z_mm3, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the inverse must put the 8000000.0 mm³ back on `z_mm3`");
    assert_eq!(snapshot, base, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-z-mm3` payload are already canonical: decode → encode
/// is a fixed point, so `newZMm3` (serde camelCase over `new_z_mm3`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-z-mm3 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-z-mm3 payload reparses");
    assert_eq!(reencoded, original, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the committed change-z-mm3 JSON is not canonical");
}

/// 🎯️ 9500000.0 mm³ is finite and differs from the committed 8000000.0 mm³, so both of
/// `change-z-mm3`'s early returns are bypassed.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 9500000.0 differs from the committed 8000000.0, so the `mutation.no-op` warning guard stays shut too");
    assert!(produced.messages().is_empty(), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: an accepted change-z-mm3 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-z-mm3` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `zMm3` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-z-mm3 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the section modulus and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-z-mm3 diff decodes");
    assert_eq!(decoded.z_mm3, Some(9500000.0), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the committed diff must carry zMm3 = 9500000.0 mm³");
    assert!(decoded.m_ed_knm.is_none(), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: change-z-mm3 writes zMm3 and must leave `m_ed_knm` untouched");
    assert!(decoded.area_mm2.is_none(), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: change-z-mm3 writes zMm3 and must leave `area_mm2` untouched");
    assert!(decoded.artifact.is_none(), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the section-modulus change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-z-mm3 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: the committed diff did not carry before to after");
    assert_eq!(produced.z_mm3, 9500000.0, "change-z-mm3/raises-the-section-modulus-to-9500000-mm3: applying the committed diff must land z_mm3 on 9500000.0 mm³");
}

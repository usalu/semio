//! 🧪️ `change-f-vk-mpa` fixture — `🧭️raises-the-characteristic-shear-strength-to-0-375-mpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-f-vk-mpa` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1996Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1996Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1996Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-f-vk-mpa` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising f_vk from 0.25 MPa to 0.375 MPa rewrites `f_vk_mpa` alone — the characteristic compressive
/// strength is a separate material input.
#[semio_framework_async_macros::async_test]
fn raises_the_characteristic_shear_strength_to_0_375_mpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-f-vk-mpa applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.f_vk_mpa, 0.375, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: f_vk_mpa must read 0.375 MPa once the change lands");
    assert_eq!(applied.f_k_mpa, before().f_k_mpa, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: f_k and f_vk are independent declared material strengths, not two views of one value");
}

/// ↩️ `change-f-vk-mpa`'s inverse reads the OLD 0.25 MPa out of BASE, so replaying it puts the 0.25 MPa back on
/// `f_vk_mpa`.
#[semio_framework_async_macros::async_test]
fn restoring_0_25_mpa_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-f-vk-mpa applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the inverse of one change-f-vk-mpa is exactly one change-f-vk-mpa back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-f-vk-mpa inverse step applies");
    }
    assert_eq!(snapshot.f_vk_mpa, base.f_vk_mpa, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the inverse must put the 0.25 MPa back on `f_vk_mpa`");
    assert_eq!(snapshot, base, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-f-vk-mpa` payload are already canonical: decode →
/// encode is a fixed point, so `newFVkMpa` (serde camelCase over `new_f_vk_mpa` — `Vk` is one segment, so
/// only its `V` is capitalised) is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-f-vk-mpa payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-f-vk-mpa payload reparses");
    assert_eq!(reencoded, original, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the committed change-f-vk-mpa JSON is not canonical");
}

/// 🎯️ 0.375 MPa is finite and differs from the committed 0.25 MPa, so `change-f-vk-mpa` produces
/// a message-free outcome.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 0.375 differs from the committed 0.25, so the `mutation.no-op` warning guard stays shut too");
    assert!(produced.messages().is_empty(), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: an accepted change-f-vk-mpa emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-f-vk-mpa` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fVkMpa` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-f-vk-mpa diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the characteristic shear
/// strength and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-f-vk-mpa diff decodes");
    assert_eq!(decoded.f_vk_mpa, Some(0.375), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the committed diff must carry fVkMpa = 0.375 MPa");
    assert!(decoded.f_k_mpa.is_none(), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: change-f-vk-mpa writes fVkMpa and must leave `f_k_mpa` untouched");
    assert!(decoded.mu.is_none(), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: change-f-vk-mpa writes fVkMpa and must leave `mu` untouched");
    assert!(decoded.artifact.is_none(), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the shear-strength change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-f-vk-mpa diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: the committed diff did not carry before to after");
    assert_eq!(produced.f_vk_mpa, 0.375, "change-f-vk-mpa/raises-the-characteristic-shear-strength-to-0-375-mpa: applying the committed diff must land f_vk_mpa on 0.375 MPa");
}

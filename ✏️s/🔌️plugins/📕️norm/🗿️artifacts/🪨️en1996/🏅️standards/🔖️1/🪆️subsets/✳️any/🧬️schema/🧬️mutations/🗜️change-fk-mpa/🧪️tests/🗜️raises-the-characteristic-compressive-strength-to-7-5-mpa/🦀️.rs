//! 🧪️ `change-fk-mpa` fixture — `🗜️raises-the-characteristic-compressive-strength-to-7-5-mpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-fk-mpa` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-fk-mpa` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising f_k from 5.0 MPa to 7.5 MPa rewrites `f_k_mpa` alone — γ_M is derived from the annex and masonry
/// class, never stored, so no partial factor moves with it.
#[semio_framework_async_macros::async_test]
fn raises_the_characteristic_compressive_strength_to_7_5_mpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-fk-mpa applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.f_k_mpa, 7.5, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: f_k_mpa must read 7.5 MPa once the change lands");
    assert_eq!(applied.masonry_class, before().masonry_class, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the manufacturing-control class governs γ_M and must not be inferred from a strength edit");
}

/// ↩️ `change-fk-mpa`'s inverse reads the OLD 5.0 MPa out of BASE, so replaying it puts the 5.0 MPa back on
/// `f_k_mpa`.
#[semio_framework_async_macros::async_test]
fn restoring_5_mpa_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-fk-mpa applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the inverse of one change-fk-mpa is exactly one change-fk-mpa back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-fk-mpa inverse step applies");
    }
    assert_eq!(snapshot.f_k_mpa, base.f_k_mpa, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the inverse must put the 5.0 MPa back on `f_k_mpa`");
    assert_eq!(snapshot, base, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-fk-mpa` payload are already canonical: decode → encode
/// is a fixed point, so `newFKMpa` — serde camelCase over `new_f_k_mpa` capitalises each underscore-separated
/// segment, so the `K` stays upper-case is spelled here exactly as this artifact's own serde attributes
/// render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-fk-mpa payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-fk-mpa payload reparses");
    assert_eq!(reencoded, original, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the committed change-fk-mpa JSON is not canonical");
}

/// 🎯️ 7.5 MPa is finite and differs from the committed 5.0 MPa, so `change-fk-mpa` (whose guard
/// message reads "Fk mpa", not "F k mpa") emits nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 7.5 differs from the committed 5.0, so the `mutation.no-op` warning guard stays shut too");
    assert!(produced.messages().is_empty(), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: an accepted change-fk-mpa emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-fk-mpa` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fKMpa` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-fk-mpa diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the characteristic
/// compressive strength and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-fk-mpa diff decodes");
    assert_eq!(decoded.f_k_mpa, Some(7.5), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the committed diff must carry fKMpa = 7.5 MPa");
    assert!(decoded.f_vk_mpa.is_none(), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: change-fk-mpa writes fKMpa and must leave `f_vk_mpa` untouched");
    assert!(decoded.masonry_class.is_none(), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: change-fk-mpa writes fKMpa and must leave `masonry_class` untouched");
    assert!(decoded.artifact.is_none(), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the compressive-strength change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-fk-mpa diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: the committed diff did not carry before to after");
    assert_eq!(produced.f_k_mpa, 7.5, "change-fk-mpa/raises-the-characteristic-compressive-strength-to-7-5-mpa: applying the committed diff must land f_k_mpa on 7.5 MPa");
}

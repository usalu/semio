//! 🧪️ `change-fvk` fixture — `lowers-the-characteristic-shear-strength-to-3-5-mpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-fvk` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1995Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1995Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1995Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-fvk` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Lowering f_v,k from 4.0 MPa to 3.5 MPa rewrites `f_v_k` alone — the design shear force it resists is an
/// action and must not be trimmed to keep the §6.1.7 check passing.
#[semio_framework_async_macros::async_test]
async fn lowers_the_characteristic_shear_strength_to_3_5_mpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-fvk applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.f_v_k, 3.5, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: f_v_k must read 3.5 MPa once the change lands");
    assert_eq!(applied.v_ed_kn, before().v_ed_kn, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the design shear force is the action side of the same check and must not fall with the strength");
}

/// ↩️ `change-fvk`'s inverse reads the OLD 4.0 MPa out of BASE, so replaying it puts the 4.0 MPa shear strength
/// back on `f_v_k`.
#[semio_framework_async_macros::async_test]
async fn restoring_the_4_mpa_shear_strength_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-fvk applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the inverse of one change-fvk is exactly one change-fvk back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-fvk inverse step applies");
    }
    assert_eq!(snapshot.f_v_k, base.f_v_k, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the inverse must put the 4.0 MPa shear strength back on `f_v_k`");
    assert_eq!(snapshot, base, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-fvk` payload are already canonical: decode → encode is
/// a fixed point, so `{"ChangeFVK": {"newFVK": 3.5}}` — the variant is `ChangeFVK`, and serde camelCase over
/// `new_f_v_k` gives `newFVK` is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-fvk payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-fvk payload reparses");
    assert_eq!(reencoded, original, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the committed change-fvk JSON is not canonical");
}

/// 🎯️ 3.5 MPa is finite and differs from the committed 4.0 MPa — a DOWNWARD move, which the
/// equality guard treats no differently — so `change-fvk` (guard message "Fvk") stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the payload is finite, so `change-fvk`'s `mutation.invariant` fatal cannot fire, and 3.5 differs from the committed 4.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: an accepted change-fvk emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-fvk` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fVK` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-fvk diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the characteristic shear
/// strength and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fvk diff decodes");
    assert_eq!(decoded.f_v_k, Some(3.5), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the committed diff must carry fVK = 3.5 MPa");
    assert!(decoded.v_ed_kn.is_none(), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: change-fvk writes fVK and must leave `v_ed_kn` untouched");
    assert!(decoded.f_m_k.is_none(), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: change-fvk writes fVK and must leave `f_m_k` untouched");
    assert!(decoded.artifact.is_none(), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the shear-strength change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fvk diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: the committed diff did not carry before to after");
    assert_eq!(produced.f_v_k, 3.5, "change-fvk/lowers-the-characteristic-shear-strength-to-3-5-mpa: applying the committed diff must land f_v_k on 3.5 MPa");
}

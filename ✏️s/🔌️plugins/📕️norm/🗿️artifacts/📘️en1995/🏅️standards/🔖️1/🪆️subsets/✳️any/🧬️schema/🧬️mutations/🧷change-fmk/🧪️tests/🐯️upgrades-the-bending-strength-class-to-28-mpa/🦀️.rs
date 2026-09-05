//! 🧪️ `change-fmk` fixture — `🐯️upgrades-the-bending-strength-class-to-28-mpa`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-fmk` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1995Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1995Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1995Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-fmk` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising f_m,k from 24.0 MPa (GL24h) to 28.0 MPa (GL28h) rewrites `f_m_k` alone. A real strength-class
/// change would move f_c,0,k and f_v,k too, but each is its own `change-<field>` mutation — this one must not
/// touch them.
#[semio_framework_async_macros::async_test]
fn upgrades_the_bending_strength_class_to_28_mpa() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-fmk applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.f_m_k, 28.0, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: f_m_k must read 28.0 MPa once the change lands");
    assert_eq!(applied.f_c_0_k, before().f_c_0_k, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the compressive strength is a separate declared material property, not a function of the bending strength");
}

/// ↩️ `change-fmk`'s inverse reads the OLD 24.0 MPa out of BASE, so replaying it puts the 24.0 MPa bending
/// strength back on `f_m_k`.
#[semio_framework_async_macros::async_test]
fn restoring_the_24_mpa_bending_strength_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-fmk applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the inverse of one change-fmk is exactly one change-fmk back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-fmk inverse step applies");
    }
    assert_eq!(snapshot.f_m_k, base.f_m_k, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the inverse must put the 24.0 MPa bending strength back on `f_m_k`");
    assert_eq!(snapshot, base, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-fmk` payload are already canonical: decode → encode is
/// a fixed point, so `{"ChangeFMK": {"newFMK": 28.0}}` — the variant is `ChangeFMK`, and serde camelCase over
/// `new_f_m_k` gives `newFMK` is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-fmk payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-fmk payload reparses");
    assert_eq!(reencoded, original, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the committed change-fmk JSON is not canonical");
}

/// 🎯️ 28.0 MPa is finite and differs from the committed 24.0 MPa, so `change-fmk` (whose guard
/// message reads "Fmk") emits nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the payload is finite, so `change-fmk`'s `mutation.invariant` fatal cannot fire, and 28.0 differs from the committed 24.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: an accepted change-fmk emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-fmk` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fMK` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-fmk diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the characteristic bending
/// strength and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fmk diff decodes");
    assert_eq!(decoded.f_m_k, Some(28.0), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the committed diff must carry fMK = 28.0 MPa");
    assert!(decoded.f_c_0_k.is_none(), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: change-fmk writes fMK and must leave `f_c_0_k` untouched");
    assert!(decoded.f_v_k.is_none(), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: change-fmk writes fMK and must leave `f_v_k` untouched");
    assert!(decoded.artifact.is_none(), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the bending-strength change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-fmk diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: the committed diff did not carry before to after");
    assert_eq!(produced.f_m_k, 28.0, "change-fmk/upgrades-the-bending-strength-class-to-28-mpa: applying the committed diff must land f_m_k on 28.0 MPa");
}

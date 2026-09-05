//! 🧪️ `change-pile-lm` fixture — `📏️lengthens-the-pile-to-15-m`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-pile-lm` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1997Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1997Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1997Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-pile-lm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Lengthening the pile from 12.0 m to 15.0 m rewrites `pile_l_m` alone. The shaft area πDL grows with it,
/// but the INVESTIGATED depth — how far the ground investigation actually reached — is a separate EN 1997-2
/// fact and must not be extended to match.
#[semio_framework_async_macros::async_test]
fn lengthens_the_pile_to_15_m() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-pile-lm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-pile-lm/lengthens-the-pile-to-15-m: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.pile_l_m, 15.0, "change-pile-lm/lengthens-the-pile-to-15-m: pile_l_m must read 15.0 m once the change lands");
    assert_eq!(applied.z_investigated_m, before().z_investigated_m, "change-pile-lm/lengthens-the-pile-to-15-m: the investigated depth records what was actually explored and must never follow the design geometry");
}

/// ↩️ `change-pile-lm`'s inverse reads the OLD 12.0 m out of BASE, so replaying it puts the 12.0 m pile length
/// back on `pile_l_m`.
#[semio_framework_async_macros::async_test]
fn restoring_the_12_m_pile_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-pile-lm applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-pile-lm/lengthens-the-pile-to-15-m: the inverse of one change-pile-lm is exactly one change-pile-lm back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-pile-lm inverse step applies");
    }
    assert_eq!(snapshot.pile_l_m, base.pile_l_m, "change-pile-lm/lengthens-the-pile-to-15-m: the inverse must put the 12.0 m pile length back on `pile_l_m`");
    assert_eq!(snapshot, base, "change-pile-lm/lengthens-the-pile-to-15-m: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-pile-lm` payload are already canonical: decode → encode
/// is a fixed point, so `newPileLM` (serde camelCase over `new_pile_l_m`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-pile-lm/lengthens-the-pile-to-15-m: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-pile-lm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-pile-lm payload reparses");
    assert_eq!(reencoded, original, "change-pile-lm/lengthens-the-pile-to-15-m: the committed change-pile-lm JSON is not canonical");
}

/// 🎯️ 15.0 m is finite and differs from the committed 12.0 m, so `change-pile-lm` (whose guard
/// message reads "Pile length [m]") emits nothing.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-pile-lm/lengthens-the-pile-to-15-m: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-pile-lm/lengthens-the-pile-to-15-m: the payload is finite, so `change-pile-lm`'s `mutation.invariant` fatal cannot fire, and 15.0 differs from the committed 12.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-pile-lm/lengthens-the-pile-to-15-m: an accepted change-pile-lm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-pile-lm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `pileLM` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-pile-lm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-pile-lm/lengthens-the-pile-to-15-m: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the pile length and nothing
/// else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-lm diff decodes");
    assert_eq!(decoded.pile_l_m, Some(15.0), "change-pile-lm/lengthens-the-pile-to-15-m: the committed diff must carry pileLM = 15.0 m");
    assert!(decoded.z_investigated_m.is_none(), "change-pile-lm/lengthens-the-pile-to-15-m: change-pile-lm writes pileLM and must leave `z_investigated_m` untouched");
    assert!(decoded.pile_d_m.is_none(), "change-pile-lm/lengthens-the-pile-to-15-m: change-pile-lm writes pileLM and must leave `pile_d_m` untouched");
    assert!(decoded.artifact.is_none(), "change-pile-lm/lengthens-the-pile-to-15-m: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-pile-lm/lengthens-the-pile-to-15-m: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the pile-length change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-lm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-pile-lm/lengthens-the-pile-to-15-m: the committed diff did not carry before to after");
    assert_eq!(produced.pile_l_m, 15.0, "change-pile-lm/lengthens-the-pile-to-15-m: applying the committed diff must land pile_l_m on 15.0 m");
}

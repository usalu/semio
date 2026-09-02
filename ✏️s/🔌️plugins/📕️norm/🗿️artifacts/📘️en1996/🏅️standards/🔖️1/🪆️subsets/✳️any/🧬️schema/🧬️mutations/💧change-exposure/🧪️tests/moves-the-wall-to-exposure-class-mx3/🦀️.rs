//! 🧪️ `change-exposure` fixture — `moves-the-wall-to-exposure-class-mx3`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-exposure` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-exposure` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Moving the wall from MX1 to MX3 rewrites `exposure` alone. The EN 1996-2 Annex B minimum mortar strength
/// jumps from 1.0 MPa to 10.0 MPa for clay units, which the committed M5 no longer satisfies — and that is
/// exactly right: the check must be allowed to FAIL rather than the mutation quietly upgrading the mortar.
#[semio_framework_async_macros::async_test]
fn moves_the_wall_to_exposure_class_mx3() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-exposure applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-exposure/moves-the-wall-to-exposure-class-mx3: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.exposure, crate::artifacts::en1996::part_2::ExposureClass::Mx3, "change-exposure/moves-the-wall-to-exposure-class-mx3: exposure must read `ExposureClass::Mx3` once the change lands");
    assert_eq!(applied.mortar, before().mortar, "change-exposure/moves-the-wall-to-exposure-class-mx3: M5 must survive the move to MX3 so the durability check can report the failure instead of the mutation hiding it");
}

/// ↩️ `change-exposure`'s inverse reads the OLD `ExposureClass::Mx1` out of BASE, so replaying it puts the MX1
/// exposure class back on `exposure`.
#[semio_framework_async_macros::async_test]
fn returning_to_exposure_class_mx1_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-exposure applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-exposure/moves-the-wall-to-exposure-class-mx3: the inverse of one change-exposure is exactly one change-exposure back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-exposure inverse step applies");
    }
    assert_eq!(snapshot.exposure, base.exposure, "change-exposure/moves-the-wall-to-exposure-class-mx3: the inverse must put the MX1 exposure class back on `exposure`");
    assert_eq!(snapshot, base, "change-exposure/moves-the-wall-to-exposure-class-mx3: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-exposure` payload are already canonical: decode →
/// encode is a fixed point, so `"Mx3"` — `ExposureClass` carries no serde rename, so MX3 is spelled `Mx3` on
/// the wire is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-exposure/moves-the-wall-to-exposure-class-mx3: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-exposure payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-exposure payload reparses");
    assert_eq!(reencoded, original, "change-exposure/moves-the-wall-to-exposure-class-mx3: the committed change-exposure JSON is not canonical");
}

/// 🎯️ `Mx3` differs from the committed `Mx1`, so `change-exposure`'s equality guard stays shut —
/// and it has no admissibility guard, by design: durability is a CHECK, not a mutation invariant.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-exposure/moves-the-wall-to-exposure-class-mx3: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-exposure/moves-the-wall-to-exposure-class-mx3: `change-exposure` has no numeric-finiteness guard at all — only the equality guard — and `ExposureClass::Mx3` differs from the committed committed `Mx1`, so `mutation.no-op` must not fire");
    assert!(produced.messages().is_empty(), "change-exposure/moves-the-wall-to-exposure-class-mx3: an accepted change-exposure emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-exposure` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `exposure` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-exposure diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-exposure/moves-the-wall-to-exposure-class-mx3: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the exposure class and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-exposure diff decodes");
    assert_eq!(decoded.exposure, Some(crate::artifacts::en1996::part_2::ExposureClass::Mx3), "change-exposure/moves-the-wall-to-exposure-class-mx3: the committed diff must carry exposure = `ExposureClass::Mx3`");
    assert!(decoded.mortar.is_none(), "change-exposure/moves-the-wall-to-exposure-class-mx3: change-exposure writes exposure and must leave `mortar` untouched");
    assert!(decoded.unit.is_none(), "change-exposure/moves-the-wall-to-exposure-class-mx3: change-exposure writes exposure and must leave `unit` untouched");
    assert!(decoded.artifact.is_none(), "change-exposure/moves-the-wall-to-exposure-class-mx3: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-exposure/moves-the-wall-to-exposure-class-mx3: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the exposure-class change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-exposure diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-exposure/moves-the-wall-to-exposure-class-mx3: the committed diff did not carry before to after");
    assert_eq!(produced.exposure, crate::artifacts::en1996::part_2::ExposureClass::Mx3, "change-exposure/moves-the-wall-to-exposure-class-mx3: applying the committed diff must land exposure on `ExposureClass::Mx3`");
}

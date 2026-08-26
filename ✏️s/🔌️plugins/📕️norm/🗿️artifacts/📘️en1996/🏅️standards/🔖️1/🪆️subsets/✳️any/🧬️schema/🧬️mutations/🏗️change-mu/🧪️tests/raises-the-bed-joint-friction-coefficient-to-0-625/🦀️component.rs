//! 🧪️ `change-mu` fixture — `raises-the-bed-joint-friction-coefficient-to-0-625`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-mu` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-mu` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising µ from 0.5 to 0.625 rewrites `mu` alone — the axial force it multiplies in the §6.2.4 sliding
/// resistance is left as committed.
#[semio_framework_async_macros::async_test]
fn raises_the_bed_joint_friction_coefficient_to_0_625() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-mu applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.mu, 0.625, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: mu must read 0.625 once the change lands");
    assert_eq!(applied.n_ed_kn, before().n_ed_kn, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: N_Ed is the axial force µ is multiplied by and must not be edited by a friction change");
}

/// ↩️ `change-mu`'s inverse reads the OLD 0.5 out of BASE, so replaying it puts the 0.5 friction coefficient
/// back on `mu`.
#[semio_framework_async_macros::async_test]
fn restoring_0_5_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-mu applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the inverse of one change-mu is exactly one change-mu back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-mu inverse step applies");
    }
    assert_eq!(snapshot.mu, base.mu, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the inverse must put the 0.5 friction coefficient back on `mu`");
    assert_eq!(snapshot, base, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-mu` payload are already canonical: decode → encode is a
/// fixed point, so `newMu` (serde camelCase over `new_mu`) is spelled here exactly as this artifact's own
/// serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-mu payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-mu payload reparses");
    assert_eq!(reencoded, original, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the committed change-mu JSON is not canonical");
}

/// 🎯️ 0.625 is finite and differs from the committed 0.5, so `change-mu` returns a bare
/// `MutationOutcome::new` with no messages.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the payload is a finite number, so the `is_finite` fatal guard stays shut, and 0.625 differs from the committed 0.5, so the `mutation.no-op` warning guard stays shut too"
    );
    assert!(produced.messages().is_empty(), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: an accepted change-mu emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-mu` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `mu` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-mu diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the friction coefficient and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-mu diff decodes");
    assert_eq!(decoded.mu, Some(0.625), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the committed diff must carry mu = 0.625");
    assert!(decoded.n_ed_kn.is_none(), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: change-mu writes mu and must leave `n_ed_kn` untouched");
    assert!(decoded.h_ed_kn.is_none(), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: change-mu writes mu and must leave `h_ed_kn` untouched");
    assert!(decoded.artifact.is_none(), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the friction-coefficient change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-mu diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: the committed diff did not carry before to after");
    assert_eq!(produced.mu, 0.625, "change-mu/raises-the-bed-joint-friction-coefficient-to-0-625: applying the committed diff must land mu on 0.625");
}

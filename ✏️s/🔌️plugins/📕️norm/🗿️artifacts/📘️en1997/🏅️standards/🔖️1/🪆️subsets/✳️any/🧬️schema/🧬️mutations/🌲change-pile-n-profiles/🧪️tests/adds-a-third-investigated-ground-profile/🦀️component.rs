//! 🧪️ `change-pile-n-profiles` fixture — `adds-a-third-investigated-ground-profile`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-pile-n-profiles` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1997Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1997Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1997Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-pile-n-profiles` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Going from 2 to 3 investigated ground profiles rewrites `pile_n_profiles` alone. `n` drives the EN 1997-1
/// §7.6.2.3 correlation factors ξ₃/ξ₄, which fall as more profiles are available — but the investigated DEPTH
/// is a different fact about the same campaign and does not change with the profile count.
#[semio_framework_async_macros::async_test]
async fn adds_a_third_investigated_ground_profile() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-pile-n-profiles applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.pile_n_profiles, 3, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: pile_n_profiles must read 3 profiles once the change lands");
    assert_eq!(applied.z_investigated_m, before().z_investigated_m, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: how DEEP the investigation went and how MANY profiles it produced are independent facts about the campaign");
}

/// ↩️ `change-pile-n-profiles`'s inverse reads the OLD 2 out of BASE, so replaying it puts the two-profile count
/// back on `pile_n_profiles`.
#[semio_framework_async_macros::async_test]
async fn returning_to_two_profiles_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-pile-n-profiles applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the inverse of one change-pile-n-profiles is exactly one change-pile-n-profiles back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-pile-n-profiles inverse step applies");
    }
    assert_eq!(snapshot.pile_n_profiles, base.pile_n_profiles, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the inverse must put the two-profile count back on `pile_n_profiles`");
    assert_eq!(snapshot, base, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-pile-n-profiles` payload are already canonical: decode
/// → encode is a fixed point, so `newPileNProfiles`, carrying a bare JSON integer because the field is a
/// `u32` is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-pile-n-profiles payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-pile-n-profiles payload reparses");
    assert_eq!(reencoded, original, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the committed change-pile-n-profiles JSON is not canonical");
}

/// 🎯️ `change-pile-n-profiles` is en1997's only integer-valued leaf, so it carries no
/// `mutation.invariant` guard; 3 differs from the committed 2, so `mutation.no-op` stays shut too.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-pile-n-profiles/adds-a-third-investigated-ground-profile: `pile_n_profiles` is a `u32`, so `change-pile-n-profiles` has no finiteness guard at all — only the equality one — and 3 differs from the committed 2"
    );
    assert!(produced.messages().is_empty(), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: an accepted change-pile-n-profiles emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-pile-n-profiles` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `pileNProfiles` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-pile-n-profiles diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the investigated-profile
/// count and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-n-profiles diff decodes");
    assert_eq!(decoded.pile_n_profiles, Some(3), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the committed diff must carry pileNProfiles = 3 profiles");
    assert!(decoded.z_investigated_m.is_none(), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: change-pile-n-profiles writes pileNProfiles and must leave `z_investigated_m` untouched");
    assert!(decoded.pile_l_m.is_none(), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: change-pile-n-profiles writes pileNProfiles and must leave `pile_l_m` untouched");
    assert!(decoded.artifact.is_none(), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the profile-count change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-pile-n-profiles diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-pile-n-profiles/adds-a-third-investigated-ground-profile: the committed diff did not carry before to after");
    assert_eq!(produced.pile_n_profiles, 3, "change-pile-n-profiles/adds-a-third-investigated-ground-profile: applying the committed diff must land pile_n_profiles on 3 profiles");
}

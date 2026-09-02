//! 🧪️ `change-nu` fixture — `raises-poissons-ratio-to-0-375`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-nu` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-nu` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising ν from 0.25 to 0.375 rewrites `nu` alone — the soil modulus it pairs with in the elastic
/// settlement expression is a separate declared parameter.
#[semio_framework_async_macros::async_test]
fn raises_poissons_ratio_to_0_375() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-nu applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-nu/raises-poissons-ratio-to-0-375: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.nu, 0.375, "change-nu/raises-poissons-ratio-to-0-375: nu must read 0.375 once the change lands");
    assert_eq!(applied.e_s_mpa, before().e_s_mpa, "change-nu/raises-poissons-ratio-to-0-375: E_s and ν are two independently entered elastic constants, not one derived from the other");
}

/// ↩️ `change-nu`'s inverse reads the OLD 0.25 out of BASE, so replaying it puts the 0.25 Poisson's ratio back
/// on `nu`.
#[semio_framework_async_macros::async_test]
fn restoring_0_25_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-nu applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-nu/raises-poissons-ratio-to-0-375: the inverse of one change-nu is exactly one change-nu back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-nu inverse step applies");
    }
    assert_eq!(snapshot.nu, base.nu, "change-nu/raises-poissons-ratio-to-0-375: the inverse must put the 0.25 Poisson's ratio back on `nu`");
    assert_eq!(snapshot, base, "change-nu/raises-poissons-ratio-to-0-375: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-nu` payload are already canonical: decode → encode is a
/// fixed point, so `newNu` (serde camelCase over `new_nu`) is spelled here exactly as this artifact's own
/// serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-nu/raises-poissons-ratio-to-0-375: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-nu payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-nu payload reparses");
    assert_eq!(reencoded, original, "change-nu/raises-poissons-ratio-to-0-375: the committed change-nu JSON is not canonical");
}

/// 🎯️ 0.375 is finite and differs from the committed 0.25, so `change-nu` emits nothing. The
/// thermodynamic ν < 0.5 bound is not a mutation invariant here.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-nu/raises-poissons-ratio-to-0-375: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-nu/raises-poissons-ratio-to-0-375: the payload is finite, so `change-nu`'s `mutation.invariant` fatal cannot fire, and 0.375 differs from the committed 0.25, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-nu/raises-poissons-ratio-to-0-375: an accepted change-nu emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-nu` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `nu` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-nu diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-nu/raises-poissons-ratio-to-0-375: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries Poisson's ratio and nothing
/// else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-nu diff decodes");
    assert_eq!(decoded.nu, Some(0.375), "change-nu/raises-poissons-ratio-to-0-375: the committed diff must carry nu = 0.375");
    assert!(decoded.e_s_mpa.is_none(), "change-nu/raises-poissons-ratio-to-0-375: change-nu writes nu and must leave `e_s_mpa` untouched");
    assert!(decoded.settlement_limit_mm.is_none(), "change-nu/raises-poissons-ratio-to-0-375: change-nu writes nu and must leave `settlement_limit_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-nu/raises-poissons-ratio-to-0-375: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-nu/raises-poissons-ratio-to-0-375: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the Poisson's-ratio change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-nu diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-nu/raises-poissons-ratio-to-0-375: the committed diff did not carry before to after");
    assert_eq!(produced.nu, 0.375, "change-nu/raises-poissons-ratio-to-0-375: applying the committed diff must land nu on 0.375");
}

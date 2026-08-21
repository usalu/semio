//! 🧪️ `change-n-pile-ed-kn` fixture — `raises-the-design-pile-axial-load-to-1200-kn`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-n-pile-ed-kn` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-n-pile-ed-kn` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising the pile's N_Ed from 800.0 kN to 1200.0 kN rewrites `n_pile_ed_kn` alone — it is the PILE action
/// and is entirely independent of the shallow footing's V_Ed, which the same document also carries.
#[semio_framework_async_macros::async_test]
async fn raises_the_design_pile_axial_load_to_1200_kn() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-n-pile-ed-kn applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.n_pile_ed_kn, 1200.0, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: n_pile_ed_kn must read 1200.0 kN once the change lands");
    assert_eq!(applied.v_ed_kn, before().v_ed_kn, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the shallow-foundation vertical load lives in the same flat document and must not be touched by a pile-load edit");
}

/// ↩️ `change-n-pile-ed-kn`'s inverse reads the OLD 800.0 kN out of BASE, so replaying it puts the 800.0 kN back
/// on `n_pile_ed_kn`.
#[semio_framework_async_macros::async_test]
async fn restoring_800_kn_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-n-pile-ed-kn applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the inverse of one change-n-pile-ed-kn is exactly one change-n-pile-ed-kn back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-n-pile-ed-kn inverse step applies");
    }
    assert_eq!(snapshot.n_pile_ed_kn, base.n_pile_ed_kn, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the inverse must put the 800.0 kN back on `n_pile_ed_kn`");
    assert_eq!(snapshot, base, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-n-pile-ed-kn` payload are already canonical: decode →
/// encode is a fixed point, so `newNPileEdKn` (serde camelCase over `new_n_pile_ed_kn`) is spelled here
/// exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-n-pile-ed-kn payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-n-pile-ed-kn payload reparses");
    assert_eq!(reencoded, original, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the committed change-n-pile-ed-kn JSON is not canonical");
}

/// 🎯️ 1200.0 kN is finite and differs from the committed 800.0 kN, so `change-n-pile-ed-kn` emits
/// nothing.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the payload is finite, so `change-n-pile-ed-kn`'s `mutation.invariant` fatal cannot fire, and 1200.0 differs from the committed 800.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: an accepted change-n-pile-ed-kn emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-n-pile-ed-kn` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `nPileEdKn` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-n-pile-ed-kn diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the design pile axial load
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-n-pile-ed-kn diff decodes");
    assert_eq!(decoded.n_pile_ed_kn, Some(1200.0), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the committed diff must carry nPileEdKn = 1200.0 kN");
    assert!(decoded.v_ed_kn.is_none(), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: change-n-pile-ed-kn writes nPileEdKn and must leave `v_ed_kn` untouched");
    assert!(decoded.pile_l_m.is_none(), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: change-n-pile-ed-kn writes nPileEdKn and must leave `pile_l_m` untouched");
    assert!(decoded.artifact.is_none(), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the pile-load change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-n-pile-ed-kn diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: the committed diff did not carry before to after");
    assert_eq!(produced.n_pile_ed_kn, 1200.0, "change-n-pile-ed-kn/raises-the-design-pile-axial-load-to-1200-kn: applying the committed diff must land n_pile_ed_kn on 1200.0 kN");
}

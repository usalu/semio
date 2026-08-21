//! 🧪️ `change-h-ed-kn` fixture — `raises-the-design-horizontal-load-to-120-kn`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-h-ed-kn` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-h-ed-kn` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising H_Ed from 80.0 kN to 120.0 kN rewrites `h_ed_kn` alone. The H/V ratio the EN 1997-1 §6.5.3 sliding
/// check forms climbs from 0.16 to 0.24, but V_Ed is the other half of that ratio and must not move with it.
#[semio_framework_async_macros::async_test]
async fn raises_the_design_horizontal_load_to_120_kn() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-h-ed-kn applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.h_ed_kn, 120.0, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: h_ed_kn must read 120.0 kN once the change lands");
    assert_eq!(applied.v_ed_kn, before().v_ed_kn, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the vertical load is what mobilises sliding friction and is entered independently of the horizontal action");
}

/// ↩️ `change-h-ed-kn`'s inverse reads the OLD 80.0 kN out of BASE, so replaying it puts the 80.0 kN back on
/// `h_ed_kn`.
#[semio_framework_async_macros::async_test]
async fn restoring_80_kn_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-h-ed-kn applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the inverse of one change-h-ed-kn is exactly one change-h-ed-kn back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-h-ed-kn inverse step applies");
    }
    assert_eq!(snapshot.h_ed_kn, base.h_ed_kn, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the inverse must put the 80.0 kN back on `h_ed_kn`");
    assert_eq!(snapshot, base, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-h-ed-kn` payload are already canonical: decode → encode
/// is a fixed point, so `newHEdKn` (serde camelCase over `new_h_ed_kn`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-h-ed-kn payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-h-ed-kn payload reparses");
    assert_eq!(reencoded, original, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the committed change-h-ed-kn JSON is not canonical");
}

/// 🎯️ 120.0 kN is finite and differs from the committed 80.0 kN, so neither of
/// `change-h-ed-kn`'s early returns is taken.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the payload is finite, so `change-h-ed-kn`'s `mutation.invariant` fatal cannot fire, and 120.0 differs from the committed 80.0, so its `mutation.no-op` warning cannot either");
    assert!(produced.messages().is_empty(), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: an accepted change-h-ed-kn emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-h-ed-kn` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `hEdKn` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-h-ed-kn diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the design horizontal load
/// and nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-h-ed-kn diff decodes");
    assert_eq!(decoded.h_ed_kn, Some(120.0), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the committed diff must carry hEdKn = 120.0 kN");
    assert!(decoded.v_ed_kn.is_none(), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: change-h-ed-kn writes hEdKn and must leave `v_ed_kn` untouched");
    assert!(decoded.phi_deg.is_none(), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: change-h-ed-kn writes hEdKn and must leave `phi_deg` untouched");
    assert!(decoded.artifact.is_none(), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the horizontal-load change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-h-ed-kn diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: the committed diff did not carry before to after");
    assert_eq!(produced.h_ed_kn, 120.0, "change-h-ed-kn/raises-the-design-horizontal-load-to-120-kn: applying the committed diff must land h_ed_kn on 120.0 kN");
}

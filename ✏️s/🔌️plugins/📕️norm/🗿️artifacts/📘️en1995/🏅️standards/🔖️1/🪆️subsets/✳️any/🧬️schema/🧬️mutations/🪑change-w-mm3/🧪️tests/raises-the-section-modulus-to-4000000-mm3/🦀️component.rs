//! 🧪️ `change-w-mm3` fixture — `raises-the-section-modulus-to-4000000-mm3`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-w-mm3` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-w-mm3` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising W from 3000000.0 mm³ to 4000000.0 mm³ rewrites `w_mm3` alone. For the committed 200 × 300 section
/// bh²/6 is exactly 3000000.0 mm³, and this mutation must NOT push b or h to keep that identity — W is its
/// own declared input.
#[semio_framework_async_macros::async_test]
async fn raises_the_section_modulus_to_4000000_mm3() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-w-mm3 applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.w_mm3, 4000000.0, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: w_mm3 must read 4000000.0 mm³ once the change lands");
    assert_eq!(applied.h_mm, before().h_mm, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the section depth is a separate input row and must never be back-solved from a section-modulus edit");
}

/// ↩️ `change-w-mm3`'s inverse reads the OLD 3000000.0 mm³ out of BASE, so replaying it puts the 3000000.0 mm³
/// back on `w_mm3`.
#[semio_framework_async_macros::async_test]
async fn restoring_3000000_mm3_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-w-mm3 applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the inverse of one change-w-mm3 is exactly one change-w-mm3 back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-w-mm3 inverse step applies");
    }
    assert_eq!(snapshot.w_mm3, base.w_mm3, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the inverse must put the 3000000.0 mm³ back on `w_mm3`");
    assert_eq!(snapshot, base, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-w-mm3` payload are already canonical: decode → encode
/// is a fixed point, so `{"ChangeWMm3": {"newWMm3": 4000000.0}}` — externally tagged is spelled here exactly
/// as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-w-mm3 payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-w-mm3 payload reparses");
    assert_eq!(reencoded, original, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the committed change-w-mm3 JSON is not canonical");
}

/// 🎯️ 4000000.0 mm³ is finite and differs from the committed 3000000.0 mm³, so `change-w-mm3`
/// produces a clean outcome.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the payload is finite, so `change-w-mm3`'s `mutation.invariant` fatal cannot fire, and 4000000.0 differs from the committed 3000000.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: an accepted change-w-mm3 emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-w-mm3` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `wMm3` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-w-mm3 diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the section modulus and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-w-mm3 diff decodes");
    assert_eq!(decoded.w_mm3, Some(4000000.0), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the committed diff must carry wMm3 = 4000000.0 mm³");
    assert!(decoded.b_mm.is_none(), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: change-w-mm3 writes wMm3 and must leave `b_mm` untouched");
    assert!(decoded.h_mm.is_none(), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: change-w-mm3 writes wMm3 and must leave `h_mm` untouched");
    assert!(decoded.artifact.is_none(), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the section-modulus change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-w-mm3 diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: the committed diff did not carry before to after");
    assert_eq!(produced.w_mm3, 4000000.0, "change-w-mm3/raises-the-section-modulus-to-4000000-mm3: applying the committed diff must land w_mm3 on 4000000.0 mm³");
}

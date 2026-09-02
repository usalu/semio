//! 🧪️ `change-design-approach` fixture — `switches-from-design-approach-1-to-design-approach-2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-design-approach` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-design-approach` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Switching the EN 1997-1 §2.4.7.3 design approach from `da1str` to `da2` rewrites `design_approach` alone.
/// DA2 factors resistances where DA1/1 factors actions, so the partial-factor set changes completely — but
/// every declared action value must ride through untouched, because factoring happens in the check, not in
/// the document.
#[semio_framework_async_macros::async_test]
fn switches_from_design_approach_1_to_design_approach_2() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-design-approach applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.design_approach, "da2", "change-design-approach/switches-from-design-approach-1-to-design-approach-2: design_approach must read "da2" once the change lands");
    assert_eq!(applied.v_ed_kn, before().v_ed_kn, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: characteristic actions are entered unfactored and must survive an approach switch byte for byte");
}

/// ↩️ `change-design-approach`'s inverse reads the OLD "da1str" out of BASE, so replaying it puts the "da1str"
/// approach back on `design_approach`.
#[semio_framework_async_macros::async_test]
fn returning_to_design_approach_1_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-design-approach applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the inverse of one change-design-approach is exactly one change-design-approach back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-design-approach inverse step applies");
    }
    assert_eq!(snapshot.design_approach, base.design_approach, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the inverse must put the "da1str" approach back on `design_approach`");
    assert_eq!(snapshot, base, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-design-approach` payload are already canonical: decode
/// → encode is a fixed point, so `newDesignApproach`, a plain JSON string (the field is an unvalidated
/// `String`, not an enum) is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-design-approach payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-design-approach payload reparses");
    assert_eq!(reencoded, original, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the committed change-design-approach JSON is not canonical");
}

/// 🎯️ "da2" differs from the committed "da1str", so the equality guard — the only guard
/// `change-design-approach` has — does not degrade this to a `mutation.no-op` warning.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: `change-design-approach` has no finiteness guard — the field is a `String` — and "da2" differs from the committed "da1str", so its only guard, the equality one, stays shut");
    assert!(produced.messages().is_empty(), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: an accepted change-design-approach emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-design-approach` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `designApproach` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-design-approach diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the design approach and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-design-approach diff decodes");
    assert_eq!(decoded.design_approach, Some("da2".to_string()), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the committed diff must carry designApproach = "da2"");
    assert!(decoded.annex.is_none(), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: change-design-approach writes designApproach and must leave `annex` untouched");
    assert!(decoded.v_ed_kn.is_none(), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: change-design-approach writes designApproach and must leave `v_ed_kn` untouched");
    assert!(decoded.artifact.is_none(), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the design-approach switch, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-design-approach diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-design-approach/switches-from-design-approach-1-to-design-approach-2: the committed diff did not carry before to after");
    assert_eq!(produced.design_approach, "da2", "change-design-approach/switches-from-design-approach-1-to-design-approach-2: applying the committed diff must land design_approach on "da2"");
}

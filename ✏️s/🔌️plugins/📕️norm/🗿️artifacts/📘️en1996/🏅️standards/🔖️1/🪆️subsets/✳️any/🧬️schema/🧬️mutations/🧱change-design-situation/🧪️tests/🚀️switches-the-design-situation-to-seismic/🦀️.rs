//! 🧪️ `change-design-situation` fixture — `🚀️switches-the-design-situation-to-seismic`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1996Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-design-situation` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-design-situation` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1996Diff> {
    <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Switching the design situation from `Persistent` to `Seismic` rewrites `design_situation` alone — none of
/// the four EN 1990 Table A1.1 situations reaches back into the declared actions.
#[semio_framework_async_macros::async_test]
fn switches_the_design_situation_to_seismic() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-design-situation applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-design-situation/switches-the-design-situation-to-seismic: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.design_situation, crate::document::DesignSituation::Seismic, "change-design-situation/switches-the-design-situation-to-seismic: design_situation must read `DesignSituation::Seismic` once the change lands");
    assert_eq!(applied.n_ed_kn, before().n_ed_kn, "change-design-situation/switches-the-design-situation-to-seismic: the declared design actions are entered per situation by the user and are never rescaled by this mutation");
}

/// ↩️ `change-design-situation`'s inverse reads the OLD `DesignSituation::Persistent` out of BASE, so replaying
/// it puts the persistent situation back on `design_situation`.
#[semio_framework_async_macros::async_test]
fn returning_to_the_persistent_situation_restores_before() {
    let base = before();
    let forward = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-design-situation applies");
    let inverse = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-design-situation/switches-the-design-situation-to-seismic: the inverse of one change-design-situation is exactly one change-design-situation back");
    for step in &inverse {
        let undo = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-design-situation inverse step applies");
    }
    assert_eq!(snapshot.design_situation, base.design_situation, "change-design-situation/switches-the-design-situation-to-seismic: the inverse must put the persistent situation back on `design_situation`");
    assert_eq!(snapshot, base, "change-design-situation/switches-the-design-situation-to-seismic: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-design-situation` payload are already canonical: decode
/// → encode is a fixed point, so `"Seismic"` — `DesignSituation` carries no serde rename is spelled here
/// exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1996Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-design-situation/switches-the-design-situation-to-seismic: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-design-situation payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-design-situation payload reparses");
    assert_eq!(reencoded, original, "change-design-situation/switches-the-design-situation-to-seismic: the committed change-design-situation JSON is not canonical");
}

/// 🎯️ `Seismic` differs from the committed `Persistent`, so `change-design-situation`'s equality
/// guard does not degrade this to a `mutation.no-op` warning.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-design-situation/switches-the-design-situation-to-seismic: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-design-situation/switches-the-design-situation-to-seismic: `change-design-situation` has no numeric-finiteness guard at all — only the equality guard — and `DesignSituation::Seismic` differs from the committed committed `Persistent`, so `mutation.no-op` must not fire");
    assert!(produced.messages().is_empty(), "change-design-situation/switches-the-design-situation-to-seismic: an accepted change-design-situation emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-design-situation` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `designSituation` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-design-situation diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-design-situation/switches-the-design-situation-to-seismic: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1996Diff`, re-encodes unchanged, and carries the design situation and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-design-situation diff decodes");
    assert_eq!(decoded.design_situation, Some(crate::document::DesignSituation::Seismic), "change-design-situation/switches-the-design-situation-to-seismic: the committed diff must carry designSituation = `DesignSituation::Seismic`");
    assert!(decoded.annex.is_none(), "change-design-situation/switches-the-design-situation-to-seismic: change-design-situation writes designSituation and must leave `annex` untouched");
    assert!(decoded.n_ed_kn.is_none(), "change-design-situation/switches-the-design-situation-to-seismic: change-design-situation writes designSituation and must leave `n_ed_kn` untouched");
    assert!(decoded.artifact.is_none(), "change-design-situation/switches-the-design-situation-to-seismic: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-design-situation/switches-the-design-situation-to-seismic: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the design-situation switch, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1996Diff = serde_json::from_str(DIFF).expect("the committed change-design-situation diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-design-situation/switches-the-design-situation-to-seismic: the committed diff did not carry before to after");
    assert_eq!(produced.design_situation, crate::document::DesignSituation::Seismic, "change-design-situation/switches-the-design-situation-to-seismic: applying the committed diff must land design_situation on `DesignSituation::Seismic`");
}

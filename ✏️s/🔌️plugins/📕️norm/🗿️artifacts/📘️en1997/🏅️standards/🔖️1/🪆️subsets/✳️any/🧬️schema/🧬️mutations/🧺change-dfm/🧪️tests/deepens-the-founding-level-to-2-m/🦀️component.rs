//! 🧪️ `change-dfm` fixture — `deepens-the-founding-level-to-2-m`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-dfm` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-dfm` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Deepening D_f from 1.5 m to 2.0 m rewrites `d_f_m` alone. The overburden surcharge q = γ·D_f rises from
/// 27.0 kPa to 36.0 kPa, but the unit weight supplying γ is a soil property and stays as committed.
#[semio_framework_async_macros::async_test]
async fn deepens_the_founding_level_to_2_m() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-dfm applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-dfm/deepens-the-founding-level-to-2-m: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.d_f_m, 2.0, "change-dfm/deepens-the-founding-level-to-2-m: d_f_m must read 2.0 m once the change lands");
    assert_eq!(applied.gamma_kn_m3, before().gamma_kn_m3, "change-dfm/deepens-the-founding-level-to-2-m: the soil unit weight is a material property and must not be adjusted by a founding-depth edit");
}

/// ↩️ `change-dfm`'s inverse reads the OLD 1.5 m out of BASE, so replaying it puts the 1.5 m founding depth back
/// on `d_f_m`.
#[semio_framework_async_macros::async_test]
async fn restoring_1_5_m_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-dfm applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-dfm/deepens-the-founding-level-to-2-m: the inverse of one change-dfm is exactly one change-dfm back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-dfm inverse step applies");
    }
    assert_eq!(snapshot.d_f_m, base.d_f_m, "change-dfm/deepens-the-founding-level-to-2-m: the inverse must put the 1.5 m founding depth back on `d_f_m`");
    assert_eq!(snapshot, base, "change-dfm/deepens-the-founding-level-to-2-m: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-dfm` payload are already canonical: decode → encode is
/// a fixed point, so `newDFM` — serde camelCase over `new_d_f_m`, one capital per underscore-separated
/// segment is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-dfm/deepens-the-founding-level-to-2-m: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-dfm payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-dfm payload reparses");
    assert_eq!(reencoded, original, "change-dfm/deepens-the-founding-level-to-2-m: the committed change-dfm JSON is not canonical");
}

/// 🎯️ 2.0 m is finite and differs from the committed 1.5 m, so `change-dfm` (whose guard message
/// reads "Founding depth D_f [m]") stays silent.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-dfm/deepens-the-founding-level-to-2-m: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-dfm/deepens-the-founding-level-to-2-m: the payload is finite, so `change-dfm`'s `mutation.invariant` fatal cannot fire, and 2.0 differs from the committed 1.5, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-dfm/deepens-the-founding-level-to-2-m: an accepted change-dfm emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-dfm` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `dFM` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-dfm diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-dfm/deepens-the-founding-level-to-2-m: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the founding depth and
/// nothing else.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-dfm diff decodes");
    assert_eq!(decoded.d_f_m, Some(2.0), "change-dfm/deepens-the-founding-level-to-2-m: the committed diff must carry dFM = 2.0 m");
    assert!(decoded.gamma_kn_m3.is_none(), "change-dfm/deepens-the-founding-level-to-2-m: change-dfm writes dFM and must leave `gamma_kn_m3` untouched");
    assert!(decoded.b_m.is_none(), "change-dfm/deepens-the-founding-level-to-2-m: change-dfm writes dFM and must leave `b_m` untouched");
    assert!(decoded.artifact.is_none(), "change-dfm/deepens-the-founding-level-to-2-m: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-dfm/deepens-the-founding-level-to-2-m: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the founding-depth change, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-dfm diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-dfm/deepens-the-founding-level-to-2-m: the committed diff did not carry before to after");
    assert_eq!(produced.d_f_m, 2.0, "change-dfm/deepens-the-founding-level-to-2-m: applying the committed diff must land d_f_m on 2.0 m");
}

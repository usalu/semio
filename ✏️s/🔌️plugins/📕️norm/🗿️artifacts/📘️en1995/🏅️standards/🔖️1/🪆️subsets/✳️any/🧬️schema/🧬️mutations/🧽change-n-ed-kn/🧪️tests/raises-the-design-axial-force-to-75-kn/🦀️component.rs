//! 🧪️ `change-n-ed-kn` fixture — `raises-the-design-axial-force-to-75-kn`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-n-ed-kn` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-n-ed-kn` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising N_Ed from 50.0 kN to 75.0 kN rewrites `n_ed_kn` alone — the gross area it is divided by for the
/// §6.2.4 combined bending-and-compression interaction is untouched.
#[semio_framework_async_macros::async_test]
fn raises_the_design_axial_force_to_75_kn() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-n-ed-kn applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.n_ed_kn, 75.0, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: n_ed_kn must read 75.0 kN once the change lands");
    assert_eq!(applied.a_mm2, before().a_mm2, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the gross area is the denominator of the compressive stress and is a section property, not a load");
}

/// ↩️ `change-n-ed-kn`'s inverse reads the OLD 50.0 kN out of BASE, so replaying it puts the 50.0 kN back on
/// `n_ed_kn`.
#[semio_framework_async_macros::async_test]
fn restoring_50_kn_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-n-ed-kn applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the inverse of one change-n-ed-kn is exactly one change-n-ed-kn back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-n-ed-kn inverse step applies");
    }
    assert_eq!(snapshot.n_ed_kn, base.n_ed_kn, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the inverse must put the 50.0 kN back on `n_ed_kn`");
    assert_eq!(snapshot, base, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-n-ed-kn` payload are already canonical: decode → encode
/// is a fixed point, so `{"ChangeNEdKn": {"newNEdKn": 75.0}}` — externally tagged is spelled here exactly as
/// this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-n-ed-kn payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-n-ed-kn payload reparses");
    assert_eq!(reencoded, original, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the committed change-n-ed-kn JSON is not canonical");
}

/// 🎯️ 75.0 kN is finite and differs from the committed 50.0 kN, so `change-n-ed-kn` returns an
/// empty message list.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the payload is finite, so `change-n-ed-kn`'s `mutation.invariant` fatal cannot fire, and 75.0 differs from the committed 50.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: an accepted change-n-ed-kn emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-n-ed-kn` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `nEdKn` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-n-ed-kn diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the design axial force and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-n-ed-kn diff decodes");
    assert_eq!(decoded.n_ed_kn, Some(75.0), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the committed diff must carry nEdKn = 75.0 kN");
    assert!(decoded.a_mm2.is_none(), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: change-n-ed-kn writes nEdKn and must leave `a_mm2` untouched");
    assert!(decoded.f_c_0_k.is_none(), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: change-n-ed-kn writes nEdKn and must leave `f_c_0_k` untouched");
    assert!(decoded.artifact.is_none(), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the axial-force change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-n-ed-kn diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: the committed diff did not carry before to after");
    assert_eq!(produced.n_ed_kn, 75.0, "change-n-ed-kn/raises-the-design-axial-force-to-75-kn: applying the committed diff must land n_ed_kn on 75.0 kN");
}

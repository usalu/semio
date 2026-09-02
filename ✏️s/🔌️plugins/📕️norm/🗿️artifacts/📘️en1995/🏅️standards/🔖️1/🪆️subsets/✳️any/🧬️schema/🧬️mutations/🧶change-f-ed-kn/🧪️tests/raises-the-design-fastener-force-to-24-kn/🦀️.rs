//! 🧪️ `change-f-ed-kn` fixture — `raises-the-design-fastener-force-to-24-kn`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-f-ed-kn` never writes it, so it stays `None` and rides the JSON round trip as a plain
//! `null`; the two nested states `None` and `Some(None)` are NOT distinguishable in this file's
//! committed diff, and nothing here asserts that they are.

use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> En1995Snapshot {
    serde_json::from_str(BEFORE).expect("the committed before-snapshot decodes")
}
fn expected_after() -> En1995Snapshot {
    serde_json::from_str(AFTER).expect("the committed after-snapshot decodes")
}
fn mutation() -> En1995Mutation {
    serde_json::from_str(MUTATION).expect("the committed `change-f-ed-kn` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Raising the connection's design force from 18.0 kN to 24.0 kN rewrites `f_ed_kn` alone — the effective
/// connection area it is carried by is a detailing quantity and must not be enlarged to compensate.
#[semio_framework_async_macros::async_test]
fn raises_the_design_fastener_force_to_24_kn() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-f-ed-kn applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.f_ed_kn, 24.0, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: f_ed_kn must read 24.0 kN once the change lands");
    assert_eq!(applied.a_ef_mm2, before().a_ef_mm2, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the effective connection area is the resistance side of the §8 connection check and is declared separately");
}

/// ↩️ `change-f-ed-kn`'s inverse reads the OLD 18.0 kN out of BASE, so replaying it puts the 18.0 kN back on
/// `f_ed_kn`.
#[semio_framework_async_macros::async_test]
fn restoring_18_kn_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-f-ed-kn applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the inverse of one change-f-ed-kn is exactly one change-f-ed-kn back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-f-ed-kn inverse step applies");
    }
    assert_eq!(snapshot.f_ed_kn, base.f_ed_kn, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the inverse must put the 18.0 kN back on `f_ed_kn`");
    assert_eq!(snapshot, base, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-f-ed-kn` payload are already canonical: decode → encode
/// is a fixed point, so `{"ChangeFEdKn": {"newFEdKn": 24.0}}` — externally tagged is spelled here exactly as
/// this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-f-ed-kn payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-f-ed-kn payload reparses");
    assert_eq!(reencoded, original, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the committed change-f-ed-kn JSON is not canonical");
}

/// 🎯️ 24.0 kN is finite and differs from the committed 18.0 kN, so `change-f-ed-kn` produces a
/// clean outcome.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the payload is finite, so `change-f-ed-kn`'s `mutation.invariant` fatal cannot fire, and 24.0 differs from the committed 18.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: an accepted change-f-ed-kn emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-f-ed-kn` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `fEdKn` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-f-ed-kn diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the design fastener force
/// and nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-f-ed-kn diff decodes");
    assert_eq!(decoded.f_ed_kn, Some(24.0), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the committed diff must carry fEdKn = 24.0 kN");
    assert!(decoded.a_ef_mm2.is_none(), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: change-f-ed-kn writes fEdKn and must leave `a_ef_mm2` untouched");
    assert!(decoded.v_ed_kn.is_none(), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: change-f-ed-kn writes fEdKn and must leave `v_ed_kn` untouched");
    assert!(decoded.artifact.is_none(), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the connection-force change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-f-ed-kn diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: the committed diff did not carry before to after");
    assert_eq!(produced.f_ed_kn, 24.0, "change-f-ed-kn/raises-the-design-fastener-force-to-24-kn: applying the committed diff must land f_ed_kn on 24.0 kN");
}

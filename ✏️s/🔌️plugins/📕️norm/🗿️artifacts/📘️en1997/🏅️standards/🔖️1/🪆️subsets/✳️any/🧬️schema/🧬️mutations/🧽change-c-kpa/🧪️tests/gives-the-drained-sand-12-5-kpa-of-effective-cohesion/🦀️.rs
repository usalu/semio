//! 🧪️ `change-c-kpa` fixture — `gives-the-drained-sand-12-5-kpa-of-effective-cohesion`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1997Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-c-kpa` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-c-kpa` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1997Diff> {
    <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Moving c' off zero to 12.5 kPa rewrites `c_kpa` alone. This is the one en1997 case whose committed BEFORE
/// value is 0.0, which makes it the fixture that pins that `change-c-kpa`'s no-op guard compares against the
/// base value and not against a falsy zero.
#[semio_framework_async_macros::async_test]
fn gives_the_drained_sand_12_5_kpa_of_effective_cohesion() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-c-kpa applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.c_kpa, 12.5, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: c_kpa must read 12.5 kPa once the change lands");
    assert_eq!(applied.phi_deg, before().phi_deg, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the friction angle is the other Mohr-Coulomb parameter and must not be reduced when cohesion appears");
}

/// ↩️ `change-c-kpa`'s inverse reads the OLD 0.0 kPa out of BASE, so replaying it puts the cohesionless 0.0 kPa
/// back on `c_kpa`.
#[semio_framework_async_macros::async_test]
fn restoring_the_cohesionless_0_kpa_restores_before() {
    let base = before();
    let forward = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-c-kpa applies");
    let inverse = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the inverse of one change-c-kpa is exactly one change-c-kpa back");
    for step in &inverse {
        let undo = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-c-kpa inverse step applies");
    }
    assert_eq!(snapshot.c_kpa, base.c_kpa, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the inverse must put the cohesionless 0.0 kPa back on `c_kpa`");
    assert_eq!(snapshot, base, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-c-kpa` payload are already canonical: decode → encode
/// is a fixed point, so `newCKpa` (serde camelCase over `new_c_kpa`) is spelled here exactly as this
/// artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1997Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-c-kpa payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-c-kpa payload reparses");
    assert_eq!(reencoded, original, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the committed change-c-kpa JSON is not canonical");
}

/// 🎯️ 12.5 kPa is finite and differs from the committed 0.0 kPa. `change-c-kpa`'s guard is a plain
/// `base.c_kpa == payload.new_c_kpa` equality test, so a zero base is not special-cased.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(
        produced.worst_level(),
        None,
        "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the payload is finite, so `change-c-kpa`'s `mutation.invariant` fatal cannot fire, and 12.5 differs from the committed 0.0, so its `mutation.no-op` warning cannot either"
    );
    assert!(produced.messages().is_empty(), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: an accepted change-c-kpa emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-c-kpa` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `cKpa` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-c-kpa diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1997Diff`, re-encodes unchanged, and carries the effective cohesion and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-c-kpa diff decodes");
    assert_eq!(decoded.c_kpa, Some(12.5), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the committed diff must carry cKpa = 12.5 kPa");
    assert!(decoded.phi_deg.is_none(), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: change-c-kpa writes cKpa and must leave `phi_deg` untouched");
    assert!(decoded.gamma_kn_m3.is_none(), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: change-c-kpa writes cKpa and must leave `gamma_kn_m3` untouched");
    assert!(decoded.artifact.is_none(), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the cohesion change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1997Diff = serde_json::from_str(DIFF).expect("the committed change-c-kpa diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: the committed diff did not carry before to after");
    assert_eq!(produced.c_kpa, 12.5, "change-c-kpa/gives-the-drained-sand-12-5-kpa-of-effective-cohesion: applying the committed diff must land c_kpa on 12.5 kPa");
}

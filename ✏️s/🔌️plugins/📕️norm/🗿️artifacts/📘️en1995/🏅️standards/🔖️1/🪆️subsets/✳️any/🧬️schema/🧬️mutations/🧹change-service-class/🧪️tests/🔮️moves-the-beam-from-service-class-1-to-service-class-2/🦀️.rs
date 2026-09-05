//! 🧪️ `change-service-class` fixture — `🔮️moves-the-beam-from-service-class-1-to-service-class-2`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-service-class` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-service-class` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Moving from service class 1 to service class 2 rewrites `service_class` alone. k_mod and k_def are both
/// indexed by (service class, load duration), so the load duration — the OTHER index — must survive untouched
/// for that lookup to stay meaningful.
#[semio_framework_async_macros::async_test]
fn moves_the_beam_from_service_class_1_to_service_class_2() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-service-class applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.service_class, "sc2", "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: service_class must read "sc2" once the change lands");
    assert_eq!(applied.load_duration, before().load_duration, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the load duration is the second index into the same k_mod table and is an independent document decision");
}

/// ↩️ `change-service-class`'s inverse reads the OLD "sc1" out of BASE, so replaying it puts the "sc1" service
/// class back on `service_class`.
#[semio_framework_async_macros::async_test]
fn returning_to_service_class_1_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-service-class applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the inverse of one change-service-class is exactly one change-service-class back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-service-class inverse step applies");
    }
    assert_eq!(snapshot.service_class, base.service_class, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the inverse must put the "sc1" service class back on `service_class`");
    assert_eq!(snapshot, base, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-service-class` payload are already canonical: decode →
/// encode is a fixed point, so `{"ChangeServiceClass": {"newServiceClass": "sc2"}}` — the field is an
/// unvalidated `String`, not an enum is spelled here exactly as this artifact's own serde attributes render
/// it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-service-class payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-service-class payload reparses");
    assert_eq!(reencoded, original, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the committed change-service-class JSON is not canonical");
}

/// 🎯️ "sc2" differs from the committed "sc1", so the equality guard — `change-service-class`'s only
/// guard — does not degrade this to a `mutation.no-op` warning.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: `service_class` is a `String`, so `change-service-class` has no finiteness guard; "sc2" differs from the committed "sc1", so its equality guard stays shut");
    assert!(produced.messages().is_empty(), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: an accepted change-service-class emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-service-class` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `serviceClass` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-service-class diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the service class and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-service-class diff decodes");
    assert_eq!(decoded.service_class, Some("sc2".to_string()), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the committed diff must carry serviceClass = "sc2"");
    assert!(decoded.load_duration.is_none(), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: change-service-class writes serviceClass and must leave `load_duration` untouched");
    assert!(decoded.annex.is_none(), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: change-service-class writes serviceClass and must leave `annex` untouched");
    assert!(decoded.artifact.is_none(), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the service-class change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-service-class diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: the committed diff did not carry before to after");
    assert_eq!(produced.service_class, "sc2", "change-service-class/moves-the-beam-from-service-class-1-to-service-class-2: applying the committed diff must land service_class on "sc2"");
}

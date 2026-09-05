//! 🧪️ `change-load-duration` fixture — `🌲️shortens-the-load-duration-class-from-medium-to-short`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.
//!
//! `En1995Diff.selected_check_index` is the artifact's one `Option<Option<u32>>` (presence lane).
//! `change-load-duration` never writes it, so it stays `None` and rides the JSON round trip as a plain
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
    serde_json::from_str(MUTATION).expect("the committed `change-load-duration` payload decodes")
}
fn built_outcome() -> protocol::MutationOutcome<En1995Diff> {
    <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &before())
}

/// ▶️ Shortening the load-duration class from medium-term to short-term rewrites `load_duration` alone. k_mod
/// rises for the same service class, but the service class itself — the other k_mod index — must not change.
#[semio_framework_async_macros::async_test]
fn shortens_the_load_duration_class_from_medium_to_short() {
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &before()).expect("change-load-duration applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the applied state differs from the committed after-snapshot");
    assert_eq!(applied.load_duration, "short", "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: load_duration must read "short" once the change lands");
    assert_eq!(applied.service_class, before().service_class, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the service class describes the moisture environment and has nothing to do with how long the load acts");
}

/// ↩️ `change-load-duration`'s inverse reads the OLD "medium" out of BASE, so replaying it puts the "medium"
/// duration class back on `load_duration`.
#[semio_framework_async_macros::async_test]
fn returning_to_the_medium_term_class_restores_before() {
    let base = before();
    let forward = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::MutationDiff::apply(forward.diff(), &base).expect("the forward change-load-duration applies");
    let inverse = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the inverse of one change-load-duration is exactly one change-load-duration back");
    for step in &inverse {
        let undo = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(undo.diff(), &snapshot).expect("the change-load-duration inverse step applies");
    }
    assert_eq!(snapshot.load_duration, base.load_duration, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the inverse must put the "medium" duration class back on `load_duration`");
    assert_eq!(snapshot, base, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: replaying the inverse did not restore the whole before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed `change-load-duration` payload are already canonical: decode →
/// encode is a fixed point, so `{"ChangeLoadDuration": {"newLoadDuration": "short"}}` — the field is an
/// unvalidated `String` is spelled here exactly as this artifact's own serde attributes render it.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1995Snapshot = serde_json::from_str(text).expect("the committed snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("the committed snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("the committed snapshot reparses");
        assert_eq!(reencoded, original, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("the change-load-duration payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("the change-load-duration payload reparses");
    assert_eq!(reencoded, original, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the committed change-load-duration JSON is not canonical");
}

/// 🎯️ "short" differs from the committed "medium", so `change-load-duration`'s only guard, the
/// equality one, stays shut.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("the committed outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: `load_duration` is a `String`, so `change-load-duration` has no finiteness guard; "short" differs from the committed "medium", so its equality guard stays shut");
    assert!(produced.messages().is_empty(), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: an accepted change-load-duration emits no diagnostics at all");
}

/// 🔺️ The sparse delta `change-load-duration` produces is exactly the committed diff — the load-bearing
/// assertion of this fixture: it pins that only `loadDuration` is written, not merely that the end state
/// matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let produced = serde_json::to_value(built_outcome().diff()).expect("the produced change-load-duration diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff decodes");
    assert_eq!(produced, committed, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `En1995Diff`, re-encodes unchanged, and carries the load-duration class and
/// nothing else.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-load-duration diff decodes");
    assert_eq!(decoded.load_duration, Some("short".to_string()), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the committed diff must carry loadDuration = "short"");
    assert!(decoded.service_class.is_none(), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: change-load-duration writes loadDuration and must leave `service_class` untouched");
    assert!(decoded.fire_duration_min.is_none(), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: change-load-duration writes loadDuration and must leave `fire_duration_min` untouched");
    assert!(decoded.artifact.is_none(), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: a field-scoped change must never fall back to a whole-artifact replacement");
    let reencoded = serde_json::to_value(&decoded).expect("the committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("the committed diff reparses");
    assert_eq!(reencoded, original, "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-snapshot to the after-snapshot: it is a complete
/// description of the duration-class change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1995Diff = serde_json::from_str(DIFF).expect("the committed change-load-duration diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("the committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: the committed diff did not carry before to after");
    assert_eq!(produced.load_duration, "short", "change-load-duration/shortens-the-load-duration-class-from-medium-to-short: applying the committed diff must land load_duration on "short"");
}

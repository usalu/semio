//! 🧪️ `change-counter` fixture — `🔢️sets-counter-to-seven`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::vcs::mutations::{apply_vcs_mutation, inverse_vcs_mutation, VcsDemoMutation};
use crate::artifacts::vcs::VcsSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> VcsSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> VcsSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> VcsDemoMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("change-counter applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-counter/sets-counter-to-seven: applied state differs from committed after-snapshot");
}

/// 🔢 `change-counter` is an ABSOLUTE set of the `counter` scalar, not an increment: 3 becomes the
/// payload's 7, never 3 + 7.
#[semio_framework_async_macros::async_test]
fn counter_is_set_absolutely_not_incremented() {
    let base = before();
    assert_eq!(base.counter, 3, "sets-counter-to-seven's before-snapshot must start at 3 for this assertion to mean anything");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("change-counter applies");
    assert_eq!(snapshot.counter, 7, "change-counter must write the payload's new_counter verbatim");
    assert_ne!(snapshot.counter, base.counter + 7, "change-counter is absolute; an increment would land on 10");
    assert_eq!(snapshot.title, base.title, "change-counter must not touch title");
    assert_eq!(snapshot.notes, base.notes, "change-counter must not touch notes");
    assert_eq!(snapshot.status, base.status, "change-counter must not touch status");
    assert_eq!(snapshot.tags, base.tags, "change-counter must not touch tags");
}

/// ↩️ The inverse is a `change-counter` back to BASE's own counter value.
#[semio_framework_async_macros::async_test]
fn inverse_restores_the_previous_counter() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_vcs_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-counter undoes with exactly one counter-set");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_vcs_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-counter/sets-counter-to-seven: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: VcsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-counter/sets-counter-to-seven: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-counter/sets-counter-to-seven: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: a clean apply, and the sparse diff pins the
/// `counter` field alone.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-counter/sets-counter-to-seven declares an applied outcome");
    let produced = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-counter/sets-counter-to-seven: 7 differs from 3, so no no-op warning is expected, got {:?}", produced.messages());
    assert_eq!(produced.diff().counter, Some(7), "change-counter's sparse diff must pin exactly the counter field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: the `counter` lane carries the ABSOLUTE new
/// value 7, never a delta of +4 — the wire format has no increment shape to express one in.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-counter/sets-counter-to-seven: produced diff differs from the committed 🔺️diff/🔣️.json");
    let diff = outcome.diff();
    assert_eq!(diff.counter, Some(7), "the counter lane carries the final value, not a delta");
    assert!(diff.title.is_none() && diff.notes.is_none() && diff.status.is_none() && diff.tags.is_none(), "change-counter may write counter and nothing else");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `VcsDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-counter/sets-counter-to-seven: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::vcs::VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-counter/sets-counter-to-seven: committed diff did not carry before to after");
}

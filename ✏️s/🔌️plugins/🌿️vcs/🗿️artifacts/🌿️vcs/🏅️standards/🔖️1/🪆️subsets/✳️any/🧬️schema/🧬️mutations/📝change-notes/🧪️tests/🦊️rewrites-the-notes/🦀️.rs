//! 🧪️ `change-notes` fixture — `🦊️rewrites-the-notes`.
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
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("change-notes applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-notes/rewrites-the-notes: applied state differs from committed after-snapshot");
}

/// 📝 `change-notes` REPLACES the whole `notes` scalar — there is no append/patch semantics in this
/// vocabulary, so the old prose must be gone, not extended.
#[semio_framework_async_macros::async_test]
fn notes_are_replaced_wholesale() {
    let base = before();
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("change-notes applies");
    assert_eq!(snapshot.notes, "Reviewed and ready to publish.", "change-notes must write the payload's new_notes verbatim");
    assert!(!snapshot.notes.contains(base.notes.as_str()), "change-notes replaces, it never appends to, the previous notes");
    assert_eq!(snapshot.status, base.status, "change-notes must not touch status");
    assert_eq!(snapshot.title, base.title, "change-notes must not touch title");
    assert_eq!(snapshot.counter, base.counter, "change-notes must not touch counter");
    assert_eq!(snapshot.tags, base.tags, "change-notes must not touch tags");
}

/// ↩️ The inverse is a `change-notes` back to the prose BASE carried.
#[semio_framework_async_macros::async_test]
fn inverse_restores_the_previous_notes() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_vcs_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-notes undoes with exactly one counter-write");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_vcs_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-notes/rewrites-the-notes: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: VcsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-notes/rewrites-the-notes: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-notes/rewrites-the-notes: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: a clean apply, with the `notes` field alone
/// pinned in the sparse diff.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-notes/rewrites-the-notes declares an applied outcome");
    let produced = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-notes/rewrites-the-notes: the prose really changes, so no no-op warning is expected, got {:?}", produced.messages());
    assert_eq!(produced.diff().notes.as_deref(), Some("Reviewed and ready to publish."), "change-notes's sparse diff must pin exactly the notes field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: the `notes` lane carries the FULL replacement
/// prose. `VcsDiff` has no append or text-splice shape, so the whole string travels every time.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-notes/rewrites-the-notes: produced diff differs from the committed 🔺️diff/🔣️.json");
    let diff = outcome.diff();
    assert_eq!(diff.notes.as_deref(), Some("Reviewed and ready to publish."), "the notes lane carries the whole replacement string");
    assert!(diff.title.is_none() && diff.counter.is_none() && diff.status.is_none() && diff.tags.is_none(), "change-notes may write notes and nothing else");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `VcsDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-notes/rewrites-the-notes: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::vcs::VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-notes/rewrites-the-notes: committed diff did not carry before to after");
}

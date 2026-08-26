//! 🧪️ `rename-vcs` fixture — `retitles-the-document`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::vcs::mutations::{apply_vcs_mutation, inverse_vcs_mutation, VcsDemoMutation};
use crate::artifacts::vcs::VcsSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

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
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("rename-vcs applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "rename-vcs/retitles-the-document: applied state differs from committed after-snapshot");
}

/// ✏️ `rename-vcs` writes the document's identity `title` and nothing else — `counter`, `notes`,
/// `status` and the tag set are all outside this mutation's reach.
#[semio_framework_async_macros::async_test]
fn only_the_title_changes() {
    let base = before();
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("rename-vcs applies");
    assert_eq!(snapshot.title, "Retitled Fixture", "rename-vcs must write the payload's new_title into title");
    assert_ne!(snapshot.title, base.title, "rename-vcs/retitles-the-document must actually change the title, or it is a no-op fixture");
    assert_eq!(snapshot.counter, base.counter, "rename-vcs must not touch counter");
    assert_eq!(snapshot.notes, base.notes, "rename-vcs must not touch notes");
    assert_eq!(snapshot.status, base.status, "rename-vcs must not touch status");
    assert_eq!(snapshot.tags, base.tags, "rename-vcs must not touch tags");
}

/// ↩️ The inverse is a `rename-vcs` back to the title BASE carried, restoring `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_the_previous_title() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_vcs_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "rename-vcs undoes with exactly one counter-rename");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_vcs_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-vcs/retitles-the-document: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: VcsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-vcs/retitles-the-document: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-vcs/retitles-the-document: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the diff builder actually reports: a clean apply with no
/// `mutation.no-op` warning, because the requested title differs from BASE's.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-vcs/retitles-the-document declares an applied outcome");
    let produced = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "rename-vcs/retitles-the-document: a real retitle must not carry any diagnostic, got {:?}", produced.messages());
    assert_eq!(produced.diff().title.as_deref(), Some("Retitled Fixture"), "rename-vcs's sparse diff must pin exactly the title field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a single `title` scalar. `VcsDiff` carries no
/// `skip_serializing_if`, so every other lane — including the whole-artifact `artifact` escape hatch
/// — is committed as an explicit `null`, which is what makes a whole-document rewrite impossible to
/// sneak past this fixture.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-vcs/retitles-the-document: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let diff = outcome.diff();
    assert!(diff.artifact.is_none(), "rename-vcs must never reach for the whole-artifact replacement lane");
    assert!(diff.counter.is_none() && diff.notes.is_none() && diff.status.is_none() && diff.tags.is_none(), "rename-vcs may write title and nothing else");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `VcsDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-vcs/retitles-the-document: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::vcs::VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-vcs/retitles-the-document: committed diff did not carry before to after");
}

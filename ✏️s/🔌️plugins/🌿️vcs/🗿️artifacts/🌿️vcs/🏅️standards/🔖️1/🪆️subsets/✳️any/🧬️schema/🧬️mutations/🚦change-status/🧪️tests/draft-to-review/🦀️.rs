//! 🧪️ `change-status` fixture — `draft-to-review`.
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
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("change-status applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-status/draft-to-review: applied state differs from committed after-snapshot");
}

/// 🚦 `status` is a free-form string in this schema, not an enum: the diff builder accepts any
/// value that differs from BASE's and writes it verbatim, without consulting a workflow table.
#[semio_framework_async_macros::async_test]
fn status_moves_from_draft_to_review() {
    let base = before();
    assert_eq!(base.status, "draft", "draft-to-review's before-snapshot must start in draft");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("change-status applies");
    assert_eq!(snapshot.status, "review", "change-status must write the payload's new_status verbatim");
    assert_eq!(snapshot.notes, base.notes, "change-status must not touch notes");
    assert_eq!(snapshot.counter, base.counter, "change-status must not touch counter");
    assert_eq!(snapshot.title, base.title, "change-status must not touch title");
    assert_eq!(snapshot.tags, base.tags, "change-status must not touch tags — a status move is not a tag move");
}

/// ↩️ The inverse is a `change-status` back to the status BASE carried.
#[semio_framework_async_macros::async_test]
fn inverse_restores_the_previous_status() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_vcs_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "change-status undoes with exactly one counter-write");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_vcs_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-status/draft-to-review: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: VcsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-status/draft-to-review: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-status/draft-to-review: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: a clean apply with the `status` field alone
/// pinned in the sparse diff.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-status/draft-to-review declares an applied outcome");
    let produced = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-status/draft-to-review: review differs from draft, so no no-op warning is expected, got {:?}", produced.messages());
    assert_eq!(produced.diff().status.as_deref(), Some("review"), "change-status's sparse diff must pin exactly the status field");
}

/// 🔺️ The produced diff is EXACTLY the committed one: only the `status` lane. A workflow move must
/// not drag the tag set along with it, and the committed `"tags": null` is what pins that.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-status/draft-to-review: produced diff differs from the committed 🔺️diff/🔣️.json");
    let diff = outcome.diff();
    assert_eq!(diff.status.as_deref(), Some("review"), "the status lane carries the new status");
    assert!(diff.tags.is_none(), "a status move must not touch the tag collection");
    assert!(diff.title.is_none() && diff.counter.is_none() && diff.notes.is_none(), "change-status may write status and nothing else");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `VcsDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-status/draft-to-review: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::vcs::VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-status/draft-to-review: committed diff did not carry before to after");
}

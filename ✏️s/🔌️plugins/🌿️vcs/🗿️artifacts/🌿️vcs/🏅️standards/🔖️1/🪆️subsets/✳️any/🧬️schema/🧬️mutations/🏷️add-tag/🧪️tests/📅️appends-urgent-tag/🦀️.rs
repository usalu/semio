//! 🧪️ `add-tag` fixture — `📅️appends-urgent-tag`.
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
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("add-tag applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "add-tag/appends-urgent-tag: applied state differs from committed after-snapshot");
}

/// 🏷️ `tags` is set-like but ORDERED on the wire: the tag delta appends the new member after the
/// members BASE already carried, it never re-sorts or de-duplicates the existing list.
#[semio_framework_async_macros::async_test]
fn tag_is_appended_after_the_existing_members() {
    let base = before();
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("add-tag applies");
    assert_eq!(snapshot.tags.len(), base.tags.len() + 1, "add-tag adds exactly one member");
    assert_eq!(snapshot.tags.last().map(String::as_str), Some("urgent"), "the added tag lands at the END of the list");
    assert_eq!(snapshot.tags.first().map(String::as_str), Some("review"), "add-tag must leave the pre-existing members in their original order");
    assert_eq!(snapshot.title, base.title, "add-tag must not touch title");
    assert_eq!(snapshot.status, base.status, "add-tag must not touch status");
}

/// ↩️ Because BASE did NOT already carry `urgent`, the inverse is a real `remove-tag` step.
#[semio_framework_async_macros::async_test]
fn inverse_removes_the_tag_it_added() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_vcs_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "add-tag against a base that lacks the tag undoes with exactly one remove-tag");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_vcs_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "add-tag/appends-urgent-tag: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: VcsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "add-tag/appends-urgent-tag: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "add-tag/appends-urgent-tag: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder: applied with no `mutation.no-op` warning
/// (that warning is reserved for a duplicate add), and the delta is `added`-only.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "add-tag/appends-urgent-tag declares an applied outcome");
    let produced = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "add-tag/appends-urgent-tag: urgent is absent from BASE, so no no-op warning is expected, got {:?}", produced.messages());
    let delta = produced.diff().tags.clone().expect("add-tag's diff pins a tags delta");
    assert_eq!(delta.added, vec!["urgent".to_string()], "add-tag's delta must carry the one added tag");
    assert!(delta.removed.is_empty(), "add-tag never removes a tag");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a `tags` DELTA carrying one `added` member and
/// an empty `removed`. The member BASE already held never appears — the delta describes the CHANGE,
/// not the resulting list, which is precisely what a whole-collection rewrite would get wrong.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "add-tag/appends-urgent-tag: produced diff differs from the committed 🔺️diff/🔣️.json");
    let delta = outcome.diff().tags.clone().expect("add-tag pins a tags delta");
    assert_eq!(delta.added, vec!["urgent".to_string()], "only the new member travels in the delta");
    assert!(delta.removed.is_empty(), "an add never populates the removed lane");
    assert!(!delta.added.contains(&"review".to_string()), "the member BASE already carried must not be re-sent");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `VcsDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "add-tag/appends-urgent-tag: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::vcs::VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "add-tag/appends-urgent-tag: committed diff did not carry before to after");
}

//! 🧪️ `remove-tag` fixture — `detaches-the-review-tag`.
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
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("remove-tag applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "remove-tag/detaches-the-review-tag: applied state differs from committed after-snapshot");
}

/// 🗑️ Removal is by VALUE and surgical: only the named member leaves, the surviving members keep
/// their relative order.
#[semio_framework_async_macros::async_test]
fn only_the_named_tag_is_detached() {
    let base = before();
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation()).expect("remove-tag applies");
    assert!(!snapshot.tags.iter().any(|tag| tag == "review"), "remove-tag must detach the named member");
    assert_eq!(snapshot.tags, vec!["urgent".to_string()], "remove-tag must leave every other member, in order");
    assert_eq!(snapshot.status, base.status, "remove-tag must not touch status — a tag is not the status field");
    assert_eq!(snapshot.title, base.title, "remove-tag must not touch title");
}

/// ↩️ Because BASE DID carry `review`, the inverse is a real `add-tag` step. The tag delta appends,
/// so undo restores MEMBERSHIP, not position: `review` comes back at the end of the list rather than
/// at its original index — asserted here as set membership, deliberately not as snapshot equality.
#[semio_framework_async_macros::async_test]
fn inverse_re_adds_the_tag_it_detached() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_vcs_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "remove-tag against a base that has the tag undoes with exactly one add-tag");
    let mut snapshot = base.clone();
    apply_vcs_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_vcs_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot.tags.len(), base.tags.len(), "the re-added tag restores the member count");
    assert!(snapshot.tags.iter().any(|tag| tag == "review"), "remove-tag/detaches-the-review-tag: inverse must bring the review tag back");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: VcsSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-tag/detaches-the-review-tag: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-tag/detaches-the-review-tag: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches the diff builder. `remove-tag` is this facet's only verb with a
/// real `mutation.target-missing` Error path; this case deliberately exercises the SUCCESS branch,
/// so no diagnostic may appear.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "remove-tag/detaches-the-review-tag declares an applied outcome");
    let produced = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "remove-tag/detaches-the-review-tag: review IS present in BASE, so target-missing must not fire, got {:?}", produced.messages());
    let delta = produced.diff().tags.clone().expect("remove-tag's diff pins a tags delta");
    assert_eq!(delta.removed, vec!["review".to_string()], "remove-tag's delta must carry the one removed tag");
    assert!(delta.added.is_empty(), "remove-tag never adds a tag");
}

/// 🔺️ The produced diff is EXACTLY the committed one: a `tags` DELTA carrying one `removed` member
/// and an empty `added`. The surviving member is absent from the diff entirely — removal is expressed
/// by naming what LEAVES, never by re-sending what stays.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let outcome = <VcsDemoMutation as protocol::Mutation<VcsSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "remove-tag/detaches-the-review-tag: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let delta = outcome.diff().tags.clone().expect("remove-tag pins a tags delta");
    assert_eq!(delta.removed, vec!["review".to_string()], "only the detached member travels in the delta");
    assert!(delta.added.is_empty(), "a remove never populates the added lane");
    assert!(!delta.removed.contains(&"urgent".to_string()), "the surviving member must not appear in the delta at all");
}

/// 🔣️ The committed diff is itself canonical: it decodes to the artifact's own diff type and
/// re-encodes byte-for-byte, so the file is a faithful `VcsDiff`, not prose that merely resembles one.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "remove-tag/detaches-the-review-tag: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff DIRECTLY to `before` yields the committed `after` — the diff is a
/// complete description of the change, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::vcs::VcsDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::vcs::VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-tag/detaches-the-review-tag: committed diff did not carry before to after");
}

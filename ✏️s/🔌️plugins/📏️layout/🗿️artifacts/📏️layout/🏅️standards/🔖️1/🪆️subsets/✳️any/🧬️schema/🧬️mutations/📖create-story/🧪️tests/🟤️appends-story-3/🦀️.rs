//! 🧪️ `create-story` fixture — `🟤️appends-story-3`.
//!
//! Proves a `TextStory` record enters the stories collection with its style runs.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    serde_json::from_str(BEFORE).expect("create-story/appends-story-3: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("create-story/appends-story-3: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("create-story/appends-story-3: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("create-story applies to its committed before-snapshot")
}

/// ▶️ `create-story` appends the payload's `TextStory` and leaves frames/links alone.
#[semio_framework_async_macros::async_test]
async fn brings_story_3_into_the_stories_collection() {
    let after = applied();
    assert_eq!(after.stories.iter().map(|story| story.id.as_str()).collect::<Vec<_>>(), vec!["story-1", "story-2", "story-3"], "create-story appends the new story");
    let created = after.stories.iter().find(|story| story.id == "story-3").expect("create-story inserts story-3");
    assert_eq!(created.content, "Caption.", "create-story must carry the payload story's body");
    assert!(created.style_runs.is_empty(), "create-story must carry the payload story's (empty) style runs");
    assert_eq!(after.links.len(), 2, "create-story must not touch the links collection");
    assert_eq!(after, expected_after(), "create-story/appends-story-3: applied state differs from the committed after-snapshot");
}

/// ↩️ `create-story` always inverts to `delete-story` of the id it minted.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_story_it_created() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "create-story inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeleteStory(step) => assert_eq!(step.id, "story-3", "the inverse must delete the story id create-story minted"),
        other => panic!("create-story must invert to delete-story, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("create-story/appends-story-3: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-story/appends-story-3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-story/appends-story-3: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-story/appends-story-3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-story`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-story/appends-story-3: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-story/appends-story-3: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().stories.as_ref().expect("create-story fills the stories delta");
    assert_eq!(delta.added.len(), 1, "create-story adds exactly one story");
    assert_eq!(delta.added[0].id, "story-3", "create-story's `added` entry is the payload story");
    assert!(produced.diff().pages.is_none(), "create-story must not emit a pages delta");
}

/// 🔺️ The sparse delta `create-story` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only the `stories` delta is populated — `pages` and `links` stay null, so no frame is rethreaded.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-story/appends-story-3: create-story must emit a stories delta only, leaving the pages and links deltas null");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-story/appends-story-3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-story` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-story/appends-story-3: committed diff did not carry before to after");
}

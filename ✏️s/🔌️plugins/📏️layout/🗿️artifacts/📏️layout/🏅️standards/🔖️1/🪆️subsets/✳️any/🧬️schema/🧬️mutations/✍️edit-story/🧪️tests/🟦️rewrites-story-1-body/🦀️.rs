//! 🧪️ `edit-story` fixture — `🟦️rewrites-story-1-body`.
//!
//! Proves the story body is replaced wholesale while style runs survive.
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
    serde_json::from_str(BEFORE).expect("edit-story/rewrites-story-1-body: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("edit-story/rewrites-story-1-body: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("edit-story/rewrites-story-1-body: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("edit-story applies to its committed before-snapshot")
}

/// ▶️ `edit-story` patches only `content`; the `styleRuns` table is not part of the patch.
#[semio_framework_async_macros::async_test]
async fn replaces_the_story_body_and_keeps_its_style_runs() {
    let after = applied();
    let story = after.stories.iter().find(|story| story.id == "story-1").expect("story-1 survives");
    assert_eq!(story.content, "Alpha body, revised.", "edit-story must replace the addressed story's body");
    assert!(story.style_runs.is_empty(), "edit-story must leave the style runs exactly as BASE had them");
    assert_eq!(after.stories[1].content, "Spare body.", "edit-story must not rewrite sibling stories");
    assert_eq!(after, expected_after(), "edit-story/rewrites-story-1-body: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is an `edit-story` carrying BASE's body text.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_original_story_body() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "edit-story inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::EditStory(step) => {
            assert_eq!(step.id, "story-1", "the inverse must address the same story");
            assert_eq!(step.new_content, "Alpha body.", "the inverse must carry the pre-edit body text");
        }
        other => panic!("edit-story must invert to edit-story, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("edit-story/rewrites-story-1-body: inverse step applies");
    }
    assert_eq!(snapshot, base, "edit-story/rewrites-story-1-body: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "edit-story/rewrites-story-1-body: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "edit-story/rewrites-story-1-body: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `edit-story`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "edit-story/rewrites-story-1-body: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "edit-story/rewrites-story-1-body: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().stories.as_ref().expect("edit-story fills the stories delta");
    assert_eq!(delta.patched.len(), 1, "edit-story patches exactly one story");
    assert_eq!(delta.patched[0].id, "story-1", "edit-story's patch entry addresses story-1");
    assert_eq!(delta.patched[0].patch.content.as_deref(), Some("Alpha body, revised."), "edit-story fills the patch's `content` field");
}

/// 🔺️ The sparse delta `edit-story` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `stories.patched[0].patch.content` is populated — `TextStoryPatch` cannot express a style-run edit at all.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "edit-story/rewrites-story-1-body: edit-story must emit a story patch carrying the replacement body and nothing else");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "edit-story/rewrites-story-1-body: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `edit-story` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "edit-story/rewrites-story-1-body: committed diff did not carry before to after");
}

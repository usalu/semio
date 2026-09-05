//! 🧪️ `delete-story` fixture — `🚫️removes-story-2`.
//!
//! Proves a story leaves the collection without cascading into the text frame that threads it.
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
    serde_json::from_str(BEFORE).expect("delete-story/removes-story-2: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("delete-story/removes-story-2: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("delete-story/removes-story-2: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("delete-story applies to its committed before-snapshot")
}

/// ▶️ `delete-story` removes the record only — there is no cascade into frames' `story_id`.
#[semio_framework_async_macros::async_test]
async fn drops_story_2_and_leaves_the_text_frame_thread_alone() {
    let after = applied();
    assert_eq!(after.stories.iter().map(|story| story.id.as_str()).collect::<Vec<_>>(), vec!["story-1"], "delete-story must remove story-2 and only story-2");
    assert_eq!(after.pages[0].frames.len(), 2, "delete-story must not remove the text frame that references a story");
    assert_eq!(after.links.len(), 2, "delete-story must not touch the links collection");
    assert_eq!(after, expected_after(), "delete-story/removes-story-2: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-story` carrying the removed story's body and its original index.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_story_2_with_its_body() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "delete-story inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreateStory(step) => {
            assert_eq!(step.story.id, "story-2", "the inverse must recreate the removed story");
            assert_eq!(step.story.content, "Spare body.", "the inverse must carry the removed story's body, not a stub");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed story's original index");
        }
        other => panic!("delete-story must invert to create-story, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("delete-story/removes-story-2: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-story/removes-story-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-story/removes-story-2: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-story/removes-story-2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-story`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-story/removes-story-2: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-story/removes-story-2: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().stories.as_ref().expect("delete-story fills the stories delta");
    assert_eq!(delta.removed, vec!["story-2".to_string()], "delete-story's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-story touches only the `removed` arm of the stories delta");
}

/// 🔺️ The sparse delta `delete-story` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `stories.removed` is populated — the absence of a `pages` delta is what proves there is no cascade into the text frame.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-story/removes-story-2: delete-story must emit a stories delta only, proving no cascade into the frames that thread stories");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-story/removes-story-2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `delete-story` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-story/removes-story-2: committed diff did not carry before to after");
}

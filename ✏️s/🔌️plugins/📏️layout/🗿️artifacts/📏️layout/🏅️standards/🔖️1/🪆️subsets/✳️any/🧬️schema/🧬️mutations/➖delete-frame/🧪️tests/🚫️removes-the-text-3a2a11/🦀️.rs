//! 🧪️ `delete-frame` fixture — `🚫️removes-the-text-frame-and-its-layer-membership`.
//!
//! Proves the frame is dropped AND unregistered from every layer's `object_ids` in one step.
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
    serde_json::from_str(BEFORE).expect("delete-frame/removes-the-text-frame-and-its-layer-membership: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("delete-frame/removes-the-text-frame-and-its-layer-membership: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("delete-frame/removes-the-text-frame-and-its-layer-membership: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("delete-frame applies to its committed before-snapshot")
}

/// ▶️ `delete-frame` cascades into the page's layers — a dangling `object_ids` entry would be a bug.
#[semio_framework_async_macros::async_test]
async fn drops_the_frame_and_unregisters_it_from_every_layer() {
    let after = applied();
    let page = &after.pages[0];
    assert_eq!(page.frames.iter().map(|frame| frame.id()).collect::<Vec<_>>(), vec!["frame-rect"], "delete-frame must remove the addressed frame and only it");
    assert_eq!(page.layers[0].object_ids, vec!["frame-rect".to_string()], "delete-frame must unregister the id from the page's layers");
    assert_eq!(after.stories.len(), 2, "delete-frame must not cascade into the story the text frame threaded");
    assert_eq!(after, expected_after(), "delete-frame/removes-the-text-frame-and-its-layer-membership: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-frame` carrying the removed frame, its index, and the layer that held it.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_frame_at_its_index_and_layer() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "delete-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreateFrame(step) => {
            assert_eq!(step.page_id, "page-1", "the inverse must address the same page");
            assert_eq!(step.frame.id(), "frame-text", "the inverse must recreate the removed frame");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed frame's original index within the page");
            assert_eq!(step.layer_id.as_deref(), Some("layer-1"), "the inverse must capture which layer had the frame registered");
        }
        other => panic!("delete-frame must invert to create-frame, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("delete-frame/removes-the-text-frame-and-its-layer-membership: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-frame/removes-the-text-frame-and-its-layer-membership: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-frame/removes-the-text-frame-and-its-layer-membership: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-frame/removes-the-text-frame-and-its-layer-membership: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-frame`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-frame/removes-the-text-frame-and-its-layer-membership: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-frame/removes-the-text-frame-and-its-layer-membership: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("delete-frame fills the pages delta").patched[0].patch;
    assert_eq!(patch.frame_removed.as_deref(), Some("frame-text"), "delete-frame fills the page patch's `frame_removed` fragment");
    assert!(patch.frame_added.is_none() && patch.frame_patched.is_none(), "delete-frame emits only the `frame_removed` fragment");
}

/// 🔺️ The sparse delta `delete-frame` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here `frame_removed` carries the bare frame id; the layer-membership cascade is apply-side behaviour, not something the diff spells out.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-frame/removes-the-text-frame-and-its-layer-membership: delete-frame must emit a nested `frame_removed` fragment carrying the bare frame id");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-frame/removes-the-text-frame-and-its-layer-membership: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `delete-frame` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-frame/removes-the-text-frame-and-its-layer-membership: committed diff did not carry before to after");
}

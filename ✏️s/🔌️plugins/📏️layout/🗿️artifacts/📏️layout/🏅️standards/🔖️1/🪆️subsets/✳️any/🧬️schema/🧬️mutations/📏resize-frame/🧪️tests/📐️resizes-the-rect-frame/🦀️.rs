//! 🧪️ `resize-frame` fixture — `📐️resizes-the-rect-frame`.
//!
//! Proves the bounds extent changes while the origin stays anchored.
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
    serde_json::from_str(BEFORE).expect("resize-frame/resizes-the-rect-frame: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("resize-frame/resizes-the-rect-frame: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("resize-frame/resizes-the-rect-frame: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("resize-frame applies to its committed before-snapshot")
}

/// ▶️ `resize-frame` writes `bounds.w`/`bounds.h`; the origin stays anchored (no re-centering).
#[semio_framework_async_macros::async_test]
async fn rescales_the_bounds_extent_only() {
    let after = applied();
    let page = &after.pages[0];
    let bounds = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives").bounds();
    assert_eq!((bounds.width, bounds.height), (90.0, 70.0), "resize-frame must write the payload extent into the frame bounds");
    assert_eq!((bounds.x, bounds.y), (20.0, 30.0), "resize-frame must keep the origin anchored — it does not re-centre the frame");
    assert_eq!(page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives").bounds().width, 160.0, "resize-frame must not resize sibling frames");
    assert_eq!(after, expected_after(), "resize-frame/resizes-the-rect-frame: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `resize-frame` carrying the extent captured from BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_original_extent() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "resize-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ResizeFrame(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!((step.new_width, step.new_height), (60.0, 40.0), "the inverse must carry the pre-resize extent");
        }
        other => panic!("resize-frame must invert to resize-frame, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("resize-frame/resizes-the-rect-frame: inverse step applies");
    }
    assert_eq!(snapshot, base, "resize-frame/resizes-the-rect-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "resize-frame/resizes-the-rect-frame: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "resize-frame/resizes-the-rect-frame: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `resize-frame`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "resize-frame/resizes-the-rect-frame: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "resize-frame/resizes-the-rect-frame: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("resize-frame fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("resize-frame fills the page patch's `frame_patched` fragment");
    assert_eq!((patched.patch.width, patched.patch.height), (Some(90.0), Some(70.0)), "resize-frame fills only width/height of the frame patch");
    assert!(patched.patch.x.is_none() && patched.patch.y.is_none(), "resize-frame must not emit a position patch");
}

/// 🔺️ The sparse delta `resize-frame` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the nested `FramePatch` populates only `width`/`height`; `x`/`y` stay null, so the origin cannot drift.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "resize-frame/resizes-the-rect-frame: resize-frame must emit a nested frame patch populating only width and height");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "resize-frame/resizes-the-rect-frame: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `resize-frame` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "resize-frame/resizes-the-rect-frame: committed diff did not carry before to after");
}

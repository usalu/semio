//! 🧪️ `move-frame` fixture — `moves-the-rect-frame`.
//!
//! Proves the bounds origin moves while the extent and rotation stay fixed.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> LayoutSnapshot {
    serde_json::from_str(BEFORE).expect("move-frame/moves-the-rect-frame: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("move-frame/moves-the-rect-frame: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("move-frame/moves-the-rect-frame: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("move-frame applies to its committed before-snapshot")
}

/// ▶️ `move-frame` writes `bounds.x`/`bounds.y`; width, height and rotation are untouched.
#[semio_framework_async_macros::async_test]
async fn translates_the_bounds_origin_only() {
    let after = applied();
    let page = &after.pages[0];
    let bounds = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives").bounds();
    assert_eq!((bounds.x, bounds.y), (55.0, 65.0), "move-frame must write the payload position into the frame bounds");
    assert_eq!((bounds.width, bounds.height), (60.0, 40.0), "move-frame must not resize the frame");
    assert_eq!(bounds.rotation, 0.0, "move-frame must not rotate the frame");
    assert_eq!(page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives").bounds().x, 20.0, "move-frame must not move sibling frames");
    assert_eq!(after, expected_after(), "move-frame/moves-the-rect-frame: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `move-frame` carrying the bounds origin captured from BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_moves_the_rect_frame_back() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "move-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::MoveFrame(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!((step.new_x, step.new_y), (20.0, 30.0), "the inverse must carry the pre-move bounds origin");
        }
        other => panic!("move-frame must invert to move-frame, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("move-frame/moves-the-rect-frame: inverse step applies");
    }
    assert_eq!(snapshot, base, "move-frame/moves-the-rect-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "move-frame/moves-the-rect-frame: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "move-frame/moves-the-rect-frame: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `move-frame`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "move-frame/moves-the-rect-frame: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "move-frame/moves-the-rect-frame: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("move-frame fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("move-frame fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.frame_id, "frame-rect", "the `frame_patched` fragment addresses the moved frame");
    assert_eq!((patched.patch.x, patched.patch.y), (Some(55.0), Some(65.0)), "move-frame fills only x/y of the frame patch");
    assert!(patched.patch.width.is_none() && patched.patch.height.is_none(), "move-frame must not emit a size patch");
}

/// 🔺️ The sparse delta `move-frame` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the nested `FramePatch` populates only `x`/`y`; `width`/`height`/`rotation`-bearing fields stay null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "move-frame/moves-the-rect-frame: move-frame must emit a nested frame patch populating only x and y");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "move-frame/moves-the-rect-frame: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `move-frame` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "move-frame/moves-the-rect-frame: committed diff did not carry before to after");
}

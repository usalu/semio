//! 🧪️ `change-frame-stroke` fixture — `adds-a-stroke-to-the-rect-frame`.
//!
//! Proves the Rect-only `stroke` field goes from cleared to painted and `fill` is left alone.
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
    serde_json::from_str(BEFORE).expect("change-frame-stroke/adds-a-stroke-to-the-rect-frame: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("change-frame-stroke/adds-a-stroke-to-the-rect-frame: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("change-frame-stroke/adds-a-stroke-to-the-rect-frame: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-frame-stroke applies to its committed before-snapshot")
}

/// ▶️ `change-frame-stroke` writes the Rect-variant `stroke`; the `fill` stays as BASE painted it.
#[semio_framework_async_macros::async_test]
async fn paints_a_stroke_without_touching_the_fill() {
    let after = applied();
    let page = &after.pages[0];
    let frame = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives");
    let crate::artifacts::layout::Frame::Rect { fill, stroke, .. } = frame else { panic!("change-frame-stroke targets the rect frame") };
    assert_eq!(*stroke, Some([0.0, 0.0, 0.0, 1.0]), "change-frame-stroke must write the payload RGBA into the rect's stroke");
    assert_eq!(*fill, Some([1.0, 1.0, 1.0, 1.0]), "change-frame-stroke must leave the fill exactly as BASE painted it");
    assert_eq!(after, expected_after(), "change-frame-stroke/adds-a-stroke-to-the-rect-frame: applied state differs from the committed after-snapshot");
}

/// ↩️ BASE had no stroke, so the inverse carries `None` — the doubly-optional patch's "cleared" arm.
#[semio_framework_async_macros::async_test]
async fn inverse_clears_the_stroke_again() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-frame-stroke inverts to exactly one step on a rect frame");
    match &inverse[0] {
        LayoutMutation::ChangeFrameStroke(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert!(step.new_stroke.is_none(), "the inverse must carry BASE's cleared stroke");
        }
        other => panic!("change-frame-stroke must invert to change-frame-stroke, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-frame-stroke/adds-a-stroke-to-the-rect-frame: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-frame-stroke/adds-a-stroke-to-the-rect-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-frame-stroke/adds-a-stroke-to-the-rect-frame: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-frame-stroke/adds-a-stroke-to-the-rect-frame: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-frame-stroke`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-frame-stroke/adds-a-stroke-to-the-rect-frame: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-frame-stroke/adds-a-stroke-to-the-rect-frame: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("change-frame-stroke fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("change-frame-stroke fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.patch.stroke, Some(Some([0.0, 0.0, 0.0, 1.0])), "change-frame-stroke fills the doubly-optional `stroke` field");
    assert!(patched.patch.fill.is_none(), "change-frame-stroke must leave the `fill` field of the frame patch unset");
}

/// 🔺️ The sparse delta `change-frame-stroke` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the doubly-optional `stroke` serializes to the bare RGBA array; `fill` stays null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-frame-stroke/adds-a-stroke-to-the-rect-frame: change-frame-stroke must emit a nested frame patch populating only stroke");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-frame-stroke/adds-a-stroke-to-the-rect-frame: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-frame-stroke` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-frame-stroke/adds-a-stroke-to-the-rect-frame: committed diff did not carry before to after");
}

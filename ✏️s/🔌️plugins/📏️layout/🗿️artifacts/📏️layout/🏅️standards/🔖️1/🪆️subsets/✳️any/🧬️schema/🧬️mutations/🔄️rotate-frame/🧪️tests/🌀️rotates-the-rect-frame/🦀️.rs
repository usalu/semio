//! 🧪️ `rotate-frame` fixture — `🌀️rotates-the-rect-frame`.
//!
//! Proves the frame's rotation turns while its origin and extent stay fixed.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️rotate-frame/🌀️rotates-the-rect-frame/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️rotate-frame/🌀️rotates-the-rect-frame/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️rotate-frame/🌀️rotates-the-rect-frame/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️rotate-frame/🌀️rotates-the-rect-frame/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕹️rotate-frame/🌀️rotates-the-rect-frame/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(BEFORE).expect("rotate-frame/rotates-the-rect-frame: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(AFTER).expect("rotate-frame/rotates-the-rect-frame: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("rotate-frame/rotates-the-rect-frame: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("rotate-frame applies to its committed before-snapshot")
}

/// ▶️ `rotate-frame` writes `bounds.rotation`; origin, width and height are untouched.
#[semio_framework_async_macros::async_test]
async fn turns_the_rotation_only() {
    let after = applied();
    let page = &after.pages[0];
    let bounds = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives").bounds();
    assert_eq!(bounds.rotation, 15.0, "rotate-frame must write the payload rotation into the frame bounds");
    assert_eq!((bounds.x, bounds.y, bounds.width, bounds.height), (20.0, 30.0, 60.0, 40.0), "rotate-frame must neither move nor resize the frame");
    assert_eq!(page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives").bounds().rotation, 0.0, "rotate-frame must not rotate sibling frames");
    assert_eq!(after, expected_after(), "rotate-frame/rotates-the-rect-frame: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `rotate-frame` carrying the rotation captured from BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_turns_the_rect_frame_back() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "rotate-frame inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::RotateFrame(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!(step.new_rotation, 0.0, "the inverse must carry the pre-rotation angle");
        }
        other => panic!("rotate-frame must invert to rotate-frame, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("rotate-frame/rotates-the-rect-frame: inverse step applies");
    }
    assert_eq!(snapshot, base, "rotate-frame/rotates-the-rect-frame: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = dsl::os_pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rotate-frame/rotates-the-rect-frame: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rotate-frame/rotates-the-rect-frame: committed mutation JSON is not canonical");
}

/// 🔺️ The produced diff is the committed `🔺️diff`, and the committed outcome is the produced one.
#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff_and_outcome() {
    let outcome = mutation().diff(&before());
    let produced: serde_json::Value = serde_json::from_str(&dsl::os_pack::to_json_string(outcome.diff())).expect("diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rotate-frame/rotates-the-rect-frame: produced diff differs from the committed 🔺️diff");
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    assert!(outcome.messages().is_empty(), "rotate-frame/rotates-the-rect-frame: an applied rotation raises no diagnostic");
}

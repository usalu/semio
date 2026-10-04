//! 🧪️ `set-frame-flags` fixture — `🔒️locks-frame-1`.
//!
//! Proves the rect frame is locked and hidden while its geometry, paint and sibling frames stay untouched.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔒set-frame-flags/🔒️locks-frame-1/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔒set-frame-flags/🔒️locks-frame-1/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔒set-frame-flags/🔒️locks-frame-1/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔒set-frame-flags/🔒️locks-frame-1/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔒set-frame-flags/🔒️locks-frame-1/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("set-frame-flags/locks-frame-1: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("set-frame-flags/locks-frame-1: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("set-frame-flags/locks-frame-1: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("set-frame-flags applies to its committed before-snapshot")
}

/// ▶️ `set-frame-flags` writes both flags of the addressed frame and leaves its siblings alone.
#[semio_framework_async_macros::async_test]
async fn locks_and_hides_the_rect_frame_only() {
    let after = applied();
    let page = &after.pages[0];
    let rect = page.frames.iter().find(|frame| frame.id() == "frame-rect").expect("the rect frame survives");
    assert!(rect.locked(), "set-frame-flags must lock the rect frame");
    assert!(!rect.visible(), "set-frame-flags must hide the rect frame");
    let text = page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives");
    assert!(!text.locked() && text.visible(), "set-frame-flags must not touch sibling frames");
    assert_eq!(after, expected_after(), "set-frame-flags/locks-frame-1: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `set-frame-flags` carrying the flags captured from BASE for exactly the fields the payload set.
#[semio_framework_async_macros::async_test]
async fn inverse_unlocks_and_shows_the_rect_frame() {
    let base = before();
    let inverse = mutation().inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "set-frame-flags inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::SetFrameFlags(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-rect"), "the inverse must address the same frame on the same page");
            assert_eq!((step.locked, step.visible), (Some(false), Some(true)), "the inverse must carry the pre-edit flags");
        }
        other => panic!("set-frame-flags must invert to set-frame-flags, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("set-frame-flags/locks-frame-1: inverse step applies");
    }
    assert_eq!(snapshot, base, "set-frame-flags/locks-frame-1: inverse did not restore the before-snapshot (default flags are stored as null)");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "set-frame-flags/locks-frame-1: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "set-frame-flags/locks-frame-1: committed mutation JSON is not canonical");
}

/// 🔺️ The produced diff is the committed `🔺️diff` (one frame patch with only `locked`/`visible`), and the committed outcome is the produced one.
#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff_and_outcome() {
    let outcome = mutation().diff(&before());
    let produced: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "set-frame-flags/locks-frame-1: produced diff differs from the committed 🔺️diff");
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    assert!(outcome.messages().is_empty(), "set-frame-flags/locks-frame-1: an applied flag change raises no diagnostic");
}

/// 🚫️ A missing frame is rejected and never applies.
#[semio_framework_async_macros::async_test]
async fn rejects_a_missing_frame() {
    let outcome = LayoutMutation::SetFrameFlags(super::SetFrameFlags { page_id: "page-1".into(), frame_id: "frame-missing".into(), locked: Some(true), visible: None }).diff(&before());
    assert!(!outcome.messages().is_empty(), "set-frame-flags must reject a frame that does not exist");
    assert!(outcome.diff().pages.is_none(), "a rejected set-frame-flags carries no page delta");
}

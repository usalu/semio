//! 🧪️ `change-frame-columns` fixture — `🔤️splits-the-text-frame-into-two-columns`.
//!
//! Proves the Text-only `columns` count is replaced and the wrap mode is left alone.
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
    serde_json::from_str(BEFORE).expect("change-frame-columns/splits-the-text-frame-into-two-columns: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("change-frame-columns/splits-the-text-frame-into-two-columns: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("change-frame-columns/splits-the-text-frame-into-two-columns: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-frame-columns applies to its committed before-snapshot")
}

/// ▶️ `change-frame-columns` writes the Text-variant `columns` count only.
#[semio_framework_async_macros::async_test]
async fn recolumns_the_text_frame_without_changing_its_wrap_mode() {
    let after = applied();
    let page = &after.pages[0];
    let frame = page.frames.iter().find(|frame| frame.id() == "frame-text").expect("the text frame survives");
    let crate::artifacts::layout::Frame::Text { wrap_mode, columns, inset, .. } = frame else { panic!("change-frame-columns targets the text frame") };
    assert_eq!(*columns, 2, "change-frame-columns must write the payload column count");
    assert_eq!(wrap_mode, "box", "change-frame-columns must leave the wrap mode at its BASE value");
    assert_eq!(inset.width, 0.0, "change-frame-columns must not touch the text inset");
    assert_eq!(after, expected_after(), "change-frame-columns/splits-the-text-frame-into-two-columns: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `change-frame-columns` carrying BASE's column count.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_single_column_text_frame() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-frame-columns inverts to exactly one step on a text frame");
    match &inverse[0] {
        LayoutMutation::ChangeFrameColumns(step) => {
            assert_eq!((step.page_id.as_str(), step.frame_id.as_str()), ("page-1", "frame-text"), "the inverse must address the same frame on the same page");
            assert_eq!(step.new_columns, 1, "the inverse must carry the pre-edit column count");
        }
        other => panic!("change-frame-columns must invert to change-frame-columns, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-frame-columns/splits-the-text-frame-into-two-columns: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-frame-columns/splits-the-text-frame-into-two-columns: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-frame-columns/splits-the-text-frame-into-two-columns: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-frame-columns/splits-the-text-frame-into-two-columns: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-frame-columns`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-frame-columns/splits-the-text-frame-into-two-columns: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-frame-columns/splits-the-text-frame-into-two-columns: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("change-frame-columns fills the pages delta").patched[0].patch;
    let patched = patch.frame_patched.as_ref().expect("change-frame-columns fills the page patch's `frame_patched` fragment");
    assert_eq!(patched.patch.columns, Some(2), "change-frame-columns fills the `columns` field of the frame patch");
    assert!(patched.patch.wrap_mode.is_none(), "change-frame-columns must leave the `wrap_mode` field of the frame patch unset");
}

/// 🔺️ The sparse delta `change-frame-columns` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `columns` is populated — `wrap_mode` stays null in the same shared `FramePatch`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-frame-columns/splits-the-text-frame-into-two-columns: change-frame-columns must emit a nested frame patch populating only columns");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-frame-columns/splits-the-text-frame-into-two-columns: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-frame-columns` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-frame-columns/splits-the-text-frame-into-two-columns: committed diff did not carry before to after");
}

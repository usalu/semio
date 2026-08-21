//! 🧪️ `change-page-width` fixture — `widens-page-1`.
//!
//! Proves the page `width` scalar moves independently of `height`.
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
    serde_json::from_str(BEFORE).expect("change-page-width/widens-page-1: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("change-page-width/widens-page-1: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("change-page-width/widens-page-1: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-page-width applies to its committed before-snapshot")
}

/// ▶️ `change-page-width` is a single-axis setter — `height` must stay put.
#[semio_framework_async_macros::async_test]
async fn widens_page_1_without_changing_its_height() {
    let after = applied();
    let page = after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives");
    assert_eq!(page.width, 240.0, "change-page-width must set the addressed page's width");
    assert_eq!(page.height, 300.0, "change-page-width must leave the height at its BASE value");
    assert_eq!(after.pages[1].width, 200.0, "change-page-width must not resize sibling pages");
    assert_eq!(after, expected_after(), "change-page-width/widens-page-1: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `change-page-width` carrying BASE's width.
#[semio_framework_async_macros::async_test]
async fn inverse_narrows_page_1_back_to_200() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-page-width inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangePageWidth(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!(step.new_width, 200.0, "the inverse must carry the pre-edit page width");
        }
        other => panic!("change-page-width must invert to change-page-width, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-page-width/widens-page-1: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-page-width/widens-page-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-page-width/widens-page-1: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-page-width/widens-page-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-page-width`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-page-width/widens-page-1: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-page-width/widens-page-1: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().pages.as_ref().expect("change-page-width fills the pages delta");
    assert_eq!(delta.patched[0].patch.width, Some(240.0), "change-page-width fills the patch's `width` field");
    assert!(delta.patched[0].patch.height.is_none(), "change-page-width must not emit a `height` patch");
}

/// 🔺️ The sparse delta `change-page-width` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `pages.patched[0].patch.width` is populated — `height` stays null, proving the single-axis contract at the diff level.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-page-width/widens-page-1: change-page-width must emit a page patch in which only `width` is populated");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-page-width/widens-page-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-page-width` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-page-width/widens-page-1: committed diff did not carry before to after");
}

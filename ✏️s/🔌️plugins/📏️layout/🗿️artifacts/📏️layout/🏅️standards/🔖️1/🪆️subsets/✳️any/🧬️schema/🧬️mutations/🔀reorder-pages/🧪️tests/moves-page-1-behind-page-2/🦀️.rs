//! 🧪️ `reorder-pages` fixture — `moves-page-1-behind-page-2`.
//!
//! Proves the index-addressed reorder emits a COMPLETE final order, not a swap.
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
    serde_json::from_str(BEFORE).expect("reorder-pages/moves-page-1-behind-page-2: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("reorder-pages/moves-page-1-behind-page-2: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("reorder-pages/moves-page-1-behind-page-2: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("reorder-pages applies to its committed before-snapshot")
}

/// ▶️ `reorder-pages` only permutes — no page record's own fields may change.
#[semio_framework_async_macros::async_test]
async fn permutes_the_page_order_without_editing_any_page() {
    let after = applied();
    assert_eq!(after.pages.iter().map(|page| page.id.as_str()).collect::<Vec<_>>(), vec!["page-2", "page-1"], "reorder-pages must move page-1 to index 1");
    assert_eq!(after.pages[1].name, "Cover", "reorder-pages must not edit the moved page's own fields");
    assert_eq!(after.pages[1].frames.len(), 2, "reorder-pages must not disturb the moved page's frames");
    assert_eq!(after, expected_after(), "reorder-pages/moves-page-1-behind-page-2: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `reorder-pages` back to the index page-1 occupied in BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_reorders_page_1_back_to_the_front() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "reorder-pages inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ReorderPages(step) => {
            assert_eq!(step.id, "page-1", "the inverse must move the same page");
            assert_eq!(step.to_index, 0, "the inverse must target page-1's original index in BASE");
        }
        other => panic!("reorder-pages must invert to reorder-pages, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("reorder-pages/moves-page-1-behind-page-2: inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-pages/moves-page-1-behind-page-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-pages/moves-page-1-behind-page-2: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-pages/moves-page-1-behind-page-2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `reorder-pages`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "reorder-pages/moves-page-1-behind-page-2: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "reorder-pages/moves-page-1-behind-page-2: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().pages.as_ref().expect("reorder-pages fills the pages delta");
    assert_eq!(delta.reordered.as_deref(), Some(["page-2".to_string(), "page-1".to_string()].as_slice()), "reorder-pages emits the complete final id order");
    assert!(delta.added.is_empty() && delta.removed.is_empty() && delta.patched.is_empty(), "reorder-pages touches only the `reordered` arm of the pages delta");
}

/// 🔺️ The sparse delta `reorder-pages` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `pages.reordered` is populated, and it is the COMPLETE final id order — never a from/to index pair.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-pages/moves-page-1-behind-page-2: reorder-pages must emit a pages delta whose only populated arm is `reordered`, holding the complete final order");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-pages/moves-page-1-behind-page-2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `reorder-pages` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-pages/moves-page-1-behind-page-2: committed diff did not carry before to after");
}

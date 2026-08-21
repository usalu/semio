//! 🧪️ `delete-page` fixture — `removes-page-2`.
//!
//! Proves a page id leaves the collection and that undo re-materializes the full record.
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
    serde_json::from_str(BEFORE).expect("delete-page/removes-page-2: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("delete-page/removes-page-2: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("delete-page/removes-page-2: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("delete-page applies to its committed before-snapshot")
}

/// ▶️ `delete-page` removes only the addressed page; sibling pages are untouched.
#[semio_framework_async_macros::async_test]
async fn drops_page_2_and_keeps_page_1_intact() {
    let after = applied();
    assert_eq!(after.pages.iter().map(|page| page.id.as_str()).collect::<Vec<_>>(), vec!["page-1"], "delete-page must remove page-2 and only page-2");
    assert_eq!(after.pages[0].frames.len(), 2, "delete-page must not disturb the surviving page's frames");
    assert_eq!(after.stories.len(), 2, "delete-page does not cascade into the stories collection");
    assert_eq!(after, expected_after(), "delete-page/removes-page-2: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-page` carrying the ENTIRE removed page plus its original index.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_full_page_record() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "delete-page inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreatePage(step) => {
            assert_eq!(step.page.id, "page-2", "the inverse must recreate the removed page");
            assert_eq!(step.page.layers[0].id, "layer-2", "the inverse must carry the removed page's layers, not a stub");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed page's original index");
        }
        other => panic!("delete-page must invert to create-page, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("delete-page/removes-page-2: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-page/removes-page-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-page/removes-page-2: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-page/removes-page-2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-page`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-page/removes-page-2: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-page/removes-page-2: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().pages.as_ref().expect("delete-page fills the pages delta");
    assert_eq!(delta.removed, vec!["page-2".to_string()], "delete-page's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-page touches only the `removed` arm of the pages delta");
}

/// 🔺️ The sparse delta `delete-page` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `pages.removed` is populated, and it carries the bare id — the removed record itself lives in the INVERSE, never in the forward diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-page/removes-page-2: delete-page must emit a pages delta whose only populated arm is `removed`, carrying the bare id");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-page/removes-page-2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `delete-page` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-page/removes-page-2: committed diff did not carry before to after");
}

//! 🧪️ `update-page-margins` fixture — `📐️sets-asymmetric-margins-on-page-1`.
//!
//! Proves all four margin edges move in one atomic facet update.
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
    serde_json::from_str(BEFORE).expect("update-page-margins/sets-asymmetric-margins-on-page-1: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("update-page-margins/sets-asymmetric-margins-on-page-1: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("update-page-margins/sets-asymmetric-margins-on-page-1: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("update-page-margins applies to its committed before-snapshot")
}

/// ▶️ `update-page-margins` is an atomic four-field facet: every edge is written, none is inferred.
#[semio_framework_async_macros::async_test]
async fn rewrites_all_four_margin_edges_at_once() {
    let after = applied();
    let margins = &after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives").margins;
    assert_eq!((margins.top, margins.right, margins.bottom, margins.left), (12.0, 18.0, 24.0, 6.0), "update-page-margins must write all four edges from the payload");
    assert_eq!(after.pages[0].columns.count, 1, "update-page-margins must not touch the column facet");
    assert_eq!(after.pages[1].margins.top, 10.0, "update-page-margins must not touch sibling pages");
    assert_eq!(after, expected_after(), "update-page-margins/sets-asymmetric-margins-on-page-1: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse carries all four BASE edges, not a partial patch.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_uniform_ten_point_margins() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "update-page-margins inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::UpdatePageMargins(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!((step.top, step.right, step.bottom, step.left), (10.0, 10.0, 10.0, 10.0), "the inverse must carry all four pre-edit margin edges");
        }
        other => panic!("update-page-margins must invert to update-page-margins, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("update-page-margins/sets-asymmetric-margins-on-page-1: inverse step applies");
    }
    assert_eq!(snapshot, base, "update-page-margins/sets-asymmetric-margins-on-page-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-page-margins/sets-asymmetric-margins-on-page-1: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-page-margins/sets-asymmetric-margins-on-page-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `update-page-margins`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-page-margins/sets-asymmetric-margins-on-page-1: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "update-page-margins/sets-asymmetric-margins-on-page-1: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("update-page-margins fills the pages delta").patched[0].patch;
    assert_eq!((patch.margin_top, patch.margin_right, patch.margin_bottom, patch.margin_left), (Some(12.0), Some(18.0), Some(24.0), Some(6.0)), "update-page-margins fills all four margin fields of the patch");
    assert!(patch.columns_count.is_none() && patch.columns_gutter.is_none(), "update-page-margins must not emit a column patch");
}

/// 🔺️ The sparse delta `update-page-margins` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here all four `margin_*` fields are populated together and the two `columns_*` fields stay null — the atomic-facet boundary, visible in the diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-page-margins/sets-asymmetric-margins-on-page-1: update-page-margins must emit a page patch populating all four margin fields and no column field");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-page-margins/sets-asymmetric-margins-on-page-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `update-page-margins` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-page-margins/sets-asymmetric-margins-on-page-1: committed diff did not carry before to after");
}

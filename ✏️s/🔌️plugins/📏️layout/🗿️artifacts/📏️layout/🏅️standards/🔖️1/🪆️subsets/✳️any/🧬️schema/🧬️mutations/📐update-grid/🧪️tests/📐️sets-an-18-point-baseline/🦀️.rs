//! 🧪️ `update-grid` fixture — `📐️sets-an-18-point-baseline`.
//!
//! Proves the document baseline grid is replaced as one facet while pages, stories and links stay untouched.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐update-grid/📐️sets-an-18-point-baseline/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐update-grid/📐️sets-an-18-point-baseline/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐update-grid/📐️sets-an-18-point-baseline/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐update-grid/📐️sets-an-18-point-baseline/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📐update-grid/📐️sets-an-18-point-baseline/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(BEFORE).expect("update-grid/sets-an-18-point-baseline: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(AFTER).expect("update-grid/sets-an-18-point-baseline: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("update-grid/sets-an-18-point-baseline: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("update-grid applies to its committed before-snapshot")
}

/// ▶️ `update-grid` writes all three grid fields and nothing else.
#[semio_framework_async_macros::async_test]
async fn replaces_the_baseline_grid_only() {
    let base = before();
    let after = applied();
    assert_eq!((after.grid.baseline_grid, after.grid.baseline_offset, after.grid.snap_to_baseline), (18.0, 4.0, false), "update-grid must write all three grid fields from the payload");
    assert_eq!(after.pages, base.pages, "update-grid must not touch pages");
    assert_eq!(after.stories, base.stories, "update-grid must not touch stories");
    assert_eq!(after, expected_after(), "update-grid/sets-an-18-point-baseline: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is an `update-grid` carrying the grid captured from BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_twelve_point_grid() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "update-grid inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::UpdateGrid(step) => assert_eq!((step.baseline_grid, step.baseline_offset, step.snap_to_baseline), (12.0, 0.0, true), "the inverse must carry the pre-edit grid"),
        other => panic!("update-grid must invert to update-grid, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("update-grid/sets-an-18-point-baseline: inverse step applies");
    }
    assert_eq!(snapshot, base, "update-grid/sets-an-18-point-baseline: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = dsl::os_pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-grid/sets-an-18-point-baseline: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-grid/sets-an-18-point-baseline: committed mutation JSON is not canonical");
}

/// 🔺️ The produced diff is the committed `🔺️diff` (only `grid` populated), and the committed outcome is the produced one.
#[semio_framework_async_macros::async_test]
async fn produces_the_committed_diff_and_outcome() {
    let outcome = mutation().diff(&before());
    let produced: serde_json::Value = serde_json::from_str(&dsl::os_pack::to_json_string(outcome.diff())).expect("diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-grid/sets-an-18-point-baseline: produced diff differs from the committed 🔺️diff");
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"));
    assert!(outcome.messages().is_empty(), "update-grid/sets-an-18-point-baseline: an applied grid change raises no diagnostic");
}

/// 🚫️ A non-positive grid size never applies.
#[semio_framework_async_macros::async_test]
async fn rejects_a_non_positive_grid() {
    let base = before();
    let outcome = LayoutMutation::UpdateGrid(super::UpdateGrid { baseline_grid: 0.0, baseline_offset: 0.0, snap_to_baseline: true }).diff(&base);
    assert!(!outcome.messages().is_empty(), "update-grid must reject a zero baseline grid");
    assert_eq!(outcome.diff().grid, None, "a rejected update-grid carries no grid delta");
}

//! 🧪️ `update-page-columns` fixture — `🏛️splits-page-1-into-three-columns`.
//!
//! Proves count and gutter move together as one atomic facet.
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
    serde_json::from_str(BEFORE).expect("update-page-columns/splits-page-1-into-three-columns: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("update-page-columns/splits-page-1-into-three-columns: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("update-page-columns/splits-page-1-into-three-columns: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("update-page-columns applies to its committed before-snapshot")
}

/// ▶️ `update-page-columns` writes the count/gutter pair atomically and leaves the margin facet alone.
#[semio_framework_async_macros::async_test]
async fn rewrites_count_and_gutter_together() {
    let after = applied();
    let columns = &after.pages.iter().find(|page| page.id == "page-1").expect("page-1 survives").columns;
    assert_eq!((columns.count, columns.gutter), (3, 12.0), "update-page-columns must write both the count and the gutter");
    assert_eq!(after.pages[0].margins.left, 10.0, "update-page-columns must not touch the margin facet");
    assert_eq!(after.pages[1].columns.count, 1, "update-page-columns must not touch sibling pages");
    assert_eq!(after, expected_after(), "update-page-columns/splits-page-1-into-three-columns: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse carries BASE's count AND gutter together.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_single_column_grid() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "update-page-columns inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::UpdatePageColumns(step) => {
            assert_eq!(step.id, "page-1", "the inverse must address the same page");
            assert_eq!((step.count, step.gutter), (1, 0.0), "the inverse must carry the pre-edit count and gutter");
        }
        other => panic!("update-page-columns must invert to update-page-columns, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("update-page-columns/splits-page-1-into-three-columns: inverse step applies");
    }
    assert_eq!(snapshot, base, "update-page-columns/splits-page-1-into-three-columns: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-page-columns/splits-page-1-into-three-columns: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-page-columns/splits-page-1-into-three-columns: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `update-page-columns`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "update-page-columns/splits-page-1-into-three-columns: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "update-page-columns/splits-page-1-into-three-columns: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let patch = &produced.diff().pages.as_ref().expect("update-page-columns fills the pages delta").patched[0].patch;
    assert_eq!((patch.columns_count, patch.columns_gutter), (Some(3), Some(12.0)), "update-page-columns fills both column fields of the patch");
    assert!(patch.margin_top.is_none(), "update-page-columns must not emit a margin patch");
}

/// 🔺️ The sparse delta `update-page-columns` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here both `columns_*` fields are populated together and every `margin_*` field stays null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-page-columns/splits-page-1-into-three-columns: update-page-columns must emit a page patch populating both column fields and no margin field");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-page-columns/splits-page-1-into-three-columns: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `update-page-columns` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-page-columns/splits-page-1-into-three-columns: committed diff did not carry before to after");
}

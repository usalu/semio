//! 🧪️ `rename-layout` fixture — `renames-the-document`.
//!
//! Proves the document-root `name` scalar is the only thing `rename-layout` touches.
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
    serde_json::from_str(BEFORE).expect("rename-layout/renames-the-document: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("rename-layout/renames-the-document: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("rename-layout/renames-the-document: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("rename-layout applies to its committed before-snapshot")
}

/// ▶️ `rename-layout` replaces the root `name` and leaves every collection alone.
#[semio_framework_async_macros::async_test]
async fn rewrites_only_the_document_name() {
    let after = applied();
    assert_eq!(after.name, "Renamed Fixture", "rename-layout must set the document name to the payload's new_name");
    assert_eq!(after.pages.len(), 2, "rename-layout must not add or drop pages");
    assert_eq!(after.stories.len(), 2, "rename-layout must not touch the stories collection");
    assert!(after.print_target.is_none(), "rename-layout must not touch the print target");
    assert_eq!(after, expected_after(), "rename-layout/renames-the-document: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `rename-layout` carrying the BASE name captured before the edit.
#[semio_framework_async_macros::async_test]
async fn inverse_renames_back_to_fixture_layout() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "rename-layout inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::RenameLayout(step) => assert_eq!(step.new_name, "Fixture Layout", "the inverse must carry the pre-edit document name"),
        other => panic!("rename-layout must invert to rename-layout, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("rename-layout/renames-the-document: inverse step applies");
    }
    assert_eq!(snapshot, base, "rename-layout/renames-the-document: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-layout/renames-the-document: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-layout/renames-the-document: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `rename-layout`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "rename-layout/renames-the-document: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "rename-layout/renames-the-document: declared clean-applied but the diff builder reported {:?}", produced.messages());
    assert_eq!(produced.diff().name.as_deref(), Some("Renamed Fixture"), "rename-layout fills the diff's root `name` field");
    assert!(produced.diff().pages.is_none(), "rename-layout leaves the pages delta empty");
}

/// 🔺️ The sparse delta `rename-layout` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the ONLY populated field is the root `name` scalar — no collection delta at all.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-layout/renames-the-document: rename-layout must emit a diff whose sole populated field is the root `name` scalar");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-layout/renames-the-document: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `rename-layout` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-layout/renames-the-document: committed diff did not carry before to after");
}

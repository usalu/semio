//! 🧪️ `create-page` fixture — `➕️appends-page-3`.
//!
//! Proves a whole `Page` record (margins, columns, layers) enters the id-keyed collection.
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
    serde_json::from_str(BEFORE).expect("create-page/appends-page-3: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("create-page/appends-page-3: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("create-page/appends-page-3: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("create-page applies to its committed before-snapshot")
}

/// ▶️ `create-page` appends the payload's complete `Page`, margins and columns included.
#[semio_framework_async_macros::async_test]
async fn brings_a_whole_page_record_into_the_collection() {
    let after = applied();
    assert_eq!(after.pages.iter().map(|page| page.id.as_str()).collect::<Vec<_>>(), vec!["page-1", "page-2", "page-3"], "create-page appends the new page (the pages delta's `added` always pushes at the end)");
    let created = after.pages.iter().find(|page| page.id == "page-3").expect("create-page inserts page-3");
    assert_eq!(created.name, "Back", "create-page must carry the payload page's name");
    assert_eq!(created.columns.count, 2, "create-page must carry the payload page's column count");
    assert_eq!(created.margins.top, 5.0, "create-page must carry the payload page's margins");
    assert_eq!(after, expected_after(), "create-page/appends-page-3: applied state differs from the committed after-snapshot");
}

/// ↩️ `create-page` always inverts to `delete-page` of the id it minted — it never inspects BASE.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_page_it_created() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "create-page inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeletePage(step) => assert_eq!(step.id, "page-3", "the inverse must delete the page id create-page minted"),
        other => panic!("create-page must invert to delete-page, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("create-page/appends-page-3: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-page/appends-page-3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-page/appends-page-3: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-page/appends-page-3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-page`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-page/appends-page-3: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-page/appends-page-3: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().pages.as_ref().expect("create-page fills the pages delta");
    assert_eq!(delta.added.len(), 1, "create-page adds exactly one page");
    assert_eq!(delta.added[0].id, "page-3", "create-page's `added` entry is the payload page");
    assert!(delta.removed.is_empty() && delta.patched.is_empty() && delta.reordered.is_none(), "create-page touches only the `added` arm of the pages delta");
}

/// 🔺️ The sparse delta `create-page` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `pages.added` is populated — the whole `Page` record travels in the delta, and `removed`/`patched`/`reordered` stay empty.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-page/appends-page-3: create-page must emit a pages delta whose only populated arm is `added`");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-page/appends-page-3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-page` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-page/appends-page-3: committed diff did not carry before to after");
}

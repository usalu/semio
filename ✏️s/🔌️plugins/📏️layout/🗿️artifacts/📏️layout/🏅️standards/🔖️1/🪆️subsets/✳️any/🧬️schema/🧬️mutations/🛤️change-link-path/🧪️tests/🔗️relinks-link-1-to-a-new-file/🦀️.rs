//! 🧪️ `change-link-path` fixture — `🔗️relinks-link-1-to-a-new-file`.
//!
//! Proves relinking rewrites `path` only — hash and pixel size stay stale on purpose.
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
    serde_json::from_str(BEFORE).expect("change-link-path/relinks-link-1-to-a-new-file: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("change-link-path/relinks-link-1-to-a-new-file: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("change-link-path/relinks-link-1-to-a-new-file: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-link-path applies to its committed before-snapshot")
}

/// ▶️ `change-link-path` patches `path` alone; `hash`/`width`/`height`/`dpi` are NOT re-derived.
#[semio_framework_async_macros::async_test]
async fn repoints_the_link_path_but_keeps_the_stale_hash() {
    let after = applied();
    let link = after.links.iter().find(|link| link.id == "link-1").expect("link-1 survives");
    assert_eq!(link.path, "alpha-v2.png", "change-link-path must repoint the addressed link");
    assert_eq!(link.hash, "hash-alpha", "change-link-path must leave the hash untouched — it is not a re-import");
    assert_eq!((link.width, link.height, link.dpi), (800, 600, 300), "change-link-path must leave the pixel size and dpi untouched");
    assert_eq!(after.links[1].path, "spare.png", "change-link-path must not repoint sibling links");
    assert_eq!(after, expected_after(), "change-link-path/relinks-link-1-to-a-new-file: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `change-link-path` carrying BASE's path.
#[semio_framework_async_macros::async_test]
async fn inverse_repoints_link_1_at_the_original_file() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-link-path inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::ChangeLinkPath(step) => {
            assert_eq!(step.id, "link-1", "the inverse must address the same link");
            assert_eq!(step.new_path, "alpha.png", "the inverse must carry the pre-edit path");
        }
        other => panic!("change-link-path must invert to change-link-path, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-link-path/relinks-link-1-to-a-new-file: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-link-path/relinks-link-1-to-a-new-file: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-link-path/relinks-link-1-to-a-new-file: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-link-path/relinks-link-1-to-a-new-file: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-link-path`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-link-path/relinks-link-1-to-a-new-file: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-link-path/relinks-link-1-to-a-new-file: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().links.as_ref().expect("change-link-path fills the links delta");
    assert_eq!(delta.patched.len(), 1, "change-link-path patches exactly one link");
    assert_eq!(delta.patched[0].patch.path.as_deref(), Some("alpha-v2.png"), "change-link-path fills the patch's `path` field — the only field ImageLinkPatch has");
}

/// 🔺️ The sparse delta `change-link-path` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here `ImageLinkPatch` has exactly one field, so the diff structurally CANNOT re-derive the hash or pixel size.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-link-path/relinks-link-1-to-a-new-file: change-link-path must emit a link patch carrying the new path and nothing else");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-link-path/relinks-link-1-to-a-new-file: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-link-path` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-link-path/relinks-link-1-to-a-new-file: committed diff did not carry before to after");
}

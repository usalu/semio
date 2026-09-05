//! 🧪️ `create-link` fixture — `🔗️appends-link-3`.
//!
//! Proves a whole `ImageLink` record (path, hash, pixel size, dpi) enters the collection.
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
    serde_json::from_str(BEFORE).expect("create-link/appends-link-3: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("create-link/appends-link-3: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("create-link/appends-link-3: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("create-link applies to its committed before-snapshot")
}

/// ▶️ `create-link` appends the payload's complete `ImageLink`, hash and dpi included.
#[semio_framework_async_macros::async_test]
async fn brings_link_3_into_the_links_collection() {
    let after = applied();
    assert_eq!(after.links.iter().map(|link| link.id.as_str()).collect::<Vec<_>>(), vec!["link-1", "link-2", "link-3"], "create-link appends the new link");
    let created = after.links.iter().find(|link| link.id == "link-3").expect("create-link inserts link-3");
    assert_eq!(created.path, "caption.png", "create-link must carry the payload link's path");
    assert_eq!((created.width, created.height, created.dpi), (200, 150, 144), "create-link must carry the payload link's pixel size and dpi");
    assert_eq!(after.stories.len(), 2, "create-link must not touch the stories collection");
    assert_eq!(after, expected_after(), "create-link/appends-link-3: applied state differs from the committed after-snapshot");
}

/// ↩️ `create-link` always inverts to `delete-link` of the id it minted.
#[semio_framework_async_macros::async_test]
async fn inverse_deletes_the_link_it_created() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "create-link inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::DeleteLink(step) => assert_eq!(step.id, "link-3", "the inverse must delete the link id create-link minted"),
        other => panic!("create-link must invert to delete-link, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("create-link/appends-link-3: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-link/appends-link-3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-link/appends-link-3: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-link/appends-link-3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-link`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-link/appends-link-3: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-link/appends-link-3: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().links.as_ref().expect("create-link fills the links delta");
    assert_eq!(delta.added.len(), 1, "create-link adds exactly one link");
    assert_eq!(delta.added[0].hash, "hash-caption", "create-link's `added` entry carries the payload link's hash");
    assert!(produced.diff().stories.is_none(), "create-link must not emit a stories delta");
}

/// 🔺️ The sparse delta `create-link` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only the `links` delta is populated, carrying the whole `ImageLink` record including its hash and dpi.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-link/appends-link-3: create-link must emit a links delta whose only populated arm is `added`");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-link/appends-link-3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-link` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-link/appends-link-3: committed diff did not carry before to after");
}

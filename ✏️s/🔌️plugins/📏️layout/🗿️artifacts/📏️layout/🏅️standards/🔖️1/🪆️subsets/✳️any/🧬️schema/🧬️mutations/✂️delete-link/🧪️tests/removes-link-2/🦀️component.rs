//! 🧪️ `delete-link` fixture — `removes-link-2`.
//!
//! Proves a link leaves the collection with no cascade into image frames.
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
    serde_json::from_str(BEFORE).expect("delete-link/removes-link-2: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    serde_json::from_str(AFTER).expect("delete-link/removes-link-2: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("delete-link/removes-link-2: mutation decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("delete-link applies to its committed before-snapshot")
}

/// ▶️ `delete-link` removes the record only — there is no cascade into frames' `link_id`.
#[semio_framework_async_macros::async_test]
async fn drops_link_2_and_keeps_link_1_intact() {
    let after = applied();
    assert_eq!(after.links.iter().map(|link| link.id.as_str()).collect::<Vec<_>>(), vec!["link-1"], "delete-link must remove link-2 and only link-2");
    assert_eq!(after.links[0].path, "alpha.png", "delete-link must not rewrite the surviving link's path");
    assert_eq!(after.pages[0].frames.len(), 2, "delete-link does not cascade into the page's frames");
    assert_eq!(after, expected_after(), "delete-link/removes-link-2: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-link` carrying the removed link's full record and original index.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_link_2_with_its_hash() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "delete-link inverts to exactly one step");
    match &inverse[0] {
        LayoutMutation::CreateLink(step) => {
            assert_eq!(step.link.id, "link-2", "the inverse must recreate the removed link");
            assert_eq!(step.link.hash, "hash-spare", "the inverse must carry the removed link's hash, not a stub");
            assert_eq!(step.index, Some(1), "the inverse must capture the removed link's original index");
        }
        other => panic!("delete-link must invert to create-link, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("delete-link/removes-link-2: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-link/removes-link-2: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-link/removes-link-2: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-link/removes-link-2: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-link`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-link/removes-link-2: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-link/removes-link-2: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let delta = produced.diff().links.as_ref().expect("delete-link fills the links delta");
    assert_eq!(delta.removed, vec!["link-2".to_string()], "delete-link's diff carries the id in `removed`");
    assert!(delta.added.is_empty() && delta.patched.is_empty(), "delete-link touches only the `removed` arm of the links delta");
}

/// 🔺️ The sparse delta `delete-link` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here only `links.removed` is populated — the absent `pages` delta is what proves there is no cascade into image frames.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-link/removes-link-2: delete-link must emit a links delta only, proving no cascade into the frames that reference links");
}

/// 🔣️ The committed diff decodes into `LayoutDiff` and re-encodes byte-for-byte: `LayoutDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-link/removes-link-2: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `delete-link` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::layout::LayoutDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-link/removes-link-2: committed diff did not carry before to after");
}

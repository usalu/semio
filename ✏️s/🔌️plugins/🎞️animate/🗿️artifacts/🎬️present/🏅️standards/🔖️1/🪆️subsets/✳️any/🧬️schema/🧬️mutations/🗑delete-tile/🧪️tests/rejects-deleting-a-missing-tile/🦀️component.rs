//! 🧪️ `delete-tile` fixture — `rejects-deleting-a-missing-tile`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️component.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `PresentSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! 🗑️ The committed `presentation` handle is left unseeded on purpose: an unresolved child reads
//! back as the DEFAULT source with NO tiles (`present_working_scene_for_handle` falls soft, never
//! panics), which is exactly the state in which the singular delete's target guard fires. Note the
//! asymmetry with the plural `delete-tiles`, whose guard only rejects when EVERY id is missing.

use crate::artifacts::present::mutations::{apply_present_mutation, inverse_present_mutation, PresentMutation};
use crate::artifacts::present::{PresentDiff, PresentSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️component.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn mutation() -> PresentMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn before() -> PresentSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PresentSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// ▶️ A rejected `delete-tile` leaves the document byte-identical to the committed `after`.
#[semio_framework_async_macros::async_test]
async fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_present_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-tile/rejects-deleting-a-missing-tile: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a rejected delete must not mint a new presentation handle");
}

/// 🚫️ Deleting a tile that is not in the deck is an Error-level `mutation.target-missing` — an
/// Error, not the Fatal `create-tile` raises: a delete of something already gone breaks no
/// invariant, so a merge policy is allowed to absorb it.
#[semio_framework_async_macros::async_test]
async fn a_missing_tile_is_an_error_target_missing() {
    let produced = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentDiff::default(), "a rejecting delete-tile must carry the identity diff, never a half-built presentation handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a missing tile is reported as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a missing delete target is an Error, never Fatal");
    assert_eq!(messages[0].target, vec!["tiles".to_string(), "t-ghost".to_string()], "the diagnostic addresses the collection and then the one missing id");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[semio_framework_async_macros::async_test]
async fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️component.absent must be an empty marker, not a stand-in patch");
    let produced = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentDiff::default(), "delete-tile/rejects-deleting-a-missing-tile: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-tile/rejects-deleting-a-missing-tile: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-tile/rejects-deleting-a-missing-tile: committed mutation JSON is not canonical");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-tile/rejects-deleting-a-missing-tile declares a rejected outcome");
    let produced = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ `delete-tile`'s inverse is BASE-derived — it re-creates the captured tile at its captured
/// index — so a tile that was never in the deck yields NO undo step at all.
#[semio_framework_async_macros::async_test]
async fn inverse_of_a_missing_delete_is_empty() {
    let inverse = inverse_present_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "delete-tile has nothing to re-create when its target is absent, got {inverse:?}");
}

/// 🪪️ The fixture is bound to `delete-tile`'s own descriptor and its `tiles`-scoped address.
#[semio_framework_async_macros::async_test]
async fn semantics_bind_this_fixture_to_delete_tile() {
    let semantics = <PresentMutation as protocol::SemanticMutation<PresentSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "tile", "delete-tile", "DeletedTile"), "the fixture must be bound to delete-tile's own descriptor");
    assert_eq!(<PresentMutation as protocol::SemanticMutation<PresentSnapshot>>::target(&mutation()), vec!["tiles".to_string(), "t-ghost".to_string()], "delete-tile addresses the collection then exactly one tile id");
}

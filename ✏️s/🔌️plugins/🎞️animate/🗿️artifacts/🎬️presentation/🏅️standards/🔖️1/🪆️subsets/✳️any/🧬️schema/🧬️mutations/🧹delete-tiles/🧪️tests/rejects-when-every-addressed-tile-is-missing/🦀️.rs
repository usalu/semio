//! 🧪️ `delete-tiles` fixture — `rejects-when-every-addressed-tile-is-missing`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ `PresentationSnapshot` keeps its `(source, tiles)` in the composed `s.stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! 🧹 The plural delete has a THRESHOLD guard the singular one does not: it rejects only when
//! `missing.len() == ids.len()` — a partial miss still applies, downgraded to a Warning
//! `mutation.partial`. This case pins the total-miss end of that threshold with TWO addressed ids,
//! so the diagnostic's target is a genuinely variadic address, not a fixed pair.

use crate::artifacts::presentation::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};
use crate::artifacts::presentation::{PresentationDiff, PresentationSnapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn mutation() -> PresentationMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn before() -> PresentationSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> PresentationSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}

/// ▶️ A wholly unsatisfiable `delete-tiles` leaves the document byte-identical to the committed
/// `after`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_presentation_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "delete-tiles/rejects-when-every-addressed-tile-is-missing: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a wholly rejected multi-delete must not mint a new presentation handle");
}

/// 🚫️ Every addressed id is missing, so the threshold is crossed and the whole plural delete is one
/// Error `mutation.target-missing` whose target lists the collection followed by EVERY missing id,
/// in payload order — never the partial-success Warning.
#[test]
fn a_total_miss_is_one_error_listing_every_missing_id() {
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentationDiff::default(), "a wholly rejecting delete-tiles must carry the identity diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "a total miss collapses into exactly one diagnostic, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.target-missing", "a total miss is target-missing, not the partial-success code");
    assert_ne!(messages[0].code.0, "mutation.partial", "mutation.partial is reserved for a delete that still removed something");
    assert_eq!(messages[0].level, protocol::Severity::Error, "a total miss is an Error, never Fatal");
    assert_eq!(messages[0].target, vec!["tiles".to_string(), "t-alpha".to_string(), "t-omega".to_string()], "the diagnostic lists the collection and then every missing id in payload order");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[test]
fn the_committed_diff_is_declared_absent() {
    assert!(DIFF_ABSENT.is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentationDiff::default(), "delete-tiles/rejects-when-every-addressed-tile-is-missing: a rejection must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — the payload's
/// `ids` really carries two entries, which is what makes the threshold guard meaningful here.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: PresentationSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-tiles/rejects-when-every-addressed-tile-is-missing: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-tiles/rejects-when-every-addressed-tile-is-missing: committed mutation JSON is not canonical");
    assert_eq!(original.get("DeleteTiles").and_then(|payload| payload.get("ids")).and_then(serde_json::Value::as_array).map(Vec::len), Some(2), "the threshold guard needs more than one addressed id to be a real threshold");
}

/// 🎯️ The declared rejection — status, code and variadic path — is exactly what the diff builder
/// emits.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "delete-tiles/rejects-when-every-addressed-tile-is-missing declares a rejected outcome");
    let produced = <PresentationMutation as protocol::Mutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
    assert_eq!(declared_path.len(), 3, "the plural delete's address grows with the number of missing ids");
}

/// ↩️ `delete-tiles`' inverse is BASE-derived and emits ONE `create-tile` per removed tile, so a
/// total miss yields an EMPTY plan — not an empty `delete-tiles`, and not one step per requested id.
#[test]
fn inverse_of_a_total_miss_is_an_empty_plan() {
    let inverse = inverse_presentation_mutation(&before(), &mutation());
    assert!(inverse.is_empty(), "delete-tiles has nothing to re-create when every addressed tile is absent, got {inverse:?}");
}

/// 🪪️ The fixture is bound to `delete-tiles`' own PLURAL descriptor — a distinct verb row from the
/// singular `delete-tile`, not an overload of it — and to its variadic address.
#[test]
fn semantics_bind_this_fixture_to_delete_tiles() {
    let semantics = <PresentationMutation as protocol::SemanticMutation<PresentationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "tiles", "delete-tiles", "DeletedTiles"), "the fixture must be bound to the plural delete-tiles descriptor");
    assert_eq!(<PresentationMutation as protocol::SemanticMutation<PresentationSnapshot>>::target(&mutation()), vec!["tiles".to_string(), "t-alpha".to_string(), "t-omega".to_string()], "delete-tiles addresses the collection then every id it was given");
}

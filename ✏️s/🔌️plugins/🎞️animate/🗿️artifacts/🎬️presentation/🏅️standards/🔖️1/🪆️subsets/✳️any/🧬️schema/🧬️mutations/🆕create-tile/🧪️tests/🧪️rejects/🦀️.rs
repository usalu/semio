//! 🧪️ `create-tile` fixture — `🚫️rejects-a-duplicate-tile-id`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Per contract D6 a rejected case carries
//! `🔺️diff/🚫️.absent` and an `➡️after` byte-identical to `⬅️before`.
//!
//! ⚠️ Why every presentation case pins a GUARD branch: `PresentationSnapshot` keeps its `(source, tiles)` in
//! the composed `stdio.semio.presentation` CHILD (`🔖️WorkingScene`), so a committed snapshot
//! carries a handle, never a deck — and every content-changing diff routes through
//! `PresentationDiff::apply`, which derives the handle from the resulting `(source, tiles)`.
//! Hand-authoring such an `➡️after` would mean forging a value from `std`'s deliberately
//! unspecified default hasher, so this tree pins the branches that mint no handle at all.
//!
//! 🆕 `create-tile` is the ONE presentation verb whose guard is FATAL: an id collision breaks the
//! id-keyed collection's identity invariant, so no merge policy may absorb it. The seeded deck
//! holds exactly the tile the committed payload asks to create.
//!
//! 🎞️ `PresentationMutation` carries NO `#[serde(tag = ...)]`, so its wire shape is serde's EXTERNALLY
//! tagged default — `{"CreateTile": { … }}`, PascalCase variant name and all.

use crate::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};
use crate::{default_figure_tile_source, PresentationDiff, PresentationSnapshot};
use semio_framework_test_fixtures::fixture_text;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🆕create-tile/🧪️rejects/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🆕create-tile/🧪️rejects/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🆕create-tile/🧪️rejects/🦠️mutation/🔣️.json"));
static DIFF_ABSENT: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🆕create-tile/🧪️rejects/🔺️diff/🚫️.absent"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🆕create-tile/🧪️rejects/🎯️outcome/🔣️.json"));

fn mutation() -> PresentationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn expected_after() -> PresentationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to a deck holding
/// exactly the tile the committed payload carries — the collision the Fatal guards against.
fn before() -> PresentationSnapshot {
    let snapshot: PresentationSnapshot = semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let PresentationMutation::CreateTile(payload) = mutation() else {
        panic!("rejects-a-duplicate-tile-id's committed mutation must be a create-tile");
    };
    assert_eq!(snapshot.tiles, vec![payload.tile], "the committed before-snapshot must persist exactly the tile the payload re-creates");
    assert_eq!(snapshot.source, default_figure_tile_source(), "the committed before-snapshot's persisted source is the plugin's own default figure");
    snapshot
}

/// ▶️ A rejected `create-tile` leaves the document byte-identical to the committed `after`.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {
    let base = before();
    let snapshot = apply_presentation_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "create-tile/rejects-a-duplicate-tile-id: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "a rejected create must not mint a new presentation handle");
}

/// 🚨️ A colliding tile id is FATAL `mutation.duplicate-id` — the only Fatal-by-identity guard in
/// this vocabulary; every other missing/absent case here is a merge-absorbable Error.
#[test]
fn a_colliding_tile_id_is_fatal() {
    let produced = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentationDiff::default(), "a rejecting create-tile must carry the identity diff, never a half-built presentation handle");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.duplicate-id", "an id collision is reported as duplicate-id");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Fatal, "duplicate-id is Fatal — no merge policy may absorb it");
    assert_eq!(messages[0].target, vec!["tiles".to_string(), "t-hero".to_string()], "the diagnostic addresses the collection and then the colliding tile id");
}

/// 🚷 The diff is DECLARED absent, not an invented empty patch.
#[test]
fn the_committed_diff_is_declared_absent() {
    assert!((*DIFF_ABSENT).is_empty(), "🔺️diff/🚫️.absent must be an empty marker, not a stand-in patch");
    let produced = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &PresentationDiff::default(), "create-tile/rejects-a-duplicate-tile-id: a Fatal outcome must produce no delta at all");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical — including the
/// externally tagged `{"CreateTile": …}` envelope this enum's missing serde tag attribute implies.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: PresentationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-tile/rejects-a-duplicate-tile-id: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-tile/rejects-a-duplicate-tile-id: committed mutation JSON is not canonical");
    assert!(original.get("CreateTile").is_some(), "presentation mutations are externally tagged by their PascalCase variant name");
}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "create-tile/rejects-a-duplicate-tile-id declares a rejected outcome");
    let produced = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(serde_json::Value::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(serde_json::Value::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "the declared path must match the emitted target");
}

/// ↩️ A refused create leaves the existing tile outside undo history.
#[test]
fn a_duplicate_create_has_an_empty_inverse() {
    let inverse = inverse_presentation_mutation(&before(), &mutation()).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty(), "undo must preserve the existing tile");
}

/// 🪪️ The fixture is bound to `create-tile`'s own descriptor and its `tiles`-scoped address.
#[test]
fn semantics_bind_this_fixture_to_create_tile() {
    let semantics = <PresentationMutation as protocol::SemanticMutation<PresentationSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "tile", "create-tile", "CreatedTile"), "the fixture must be bound to create-tile's own descriptor");
    assert_eq!(<PresentationMutation as protocol::SemanticMutation<PresentationSnapshot>>::target(&mutation()), vec!["tiles".to_string(), "t-hero".to_string()], "create-tile addresses the collection then the new tile id");
}

#[semio_framework_async_macros::async_test]
async fn duplicate_create_obeys_the_fatal_outcome_law() {
    let outcome = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    assert_eq!(outcome.worst_level(), Some(semio_framework_diagnostic::Severity::Fatal));
    protocol::os_spr::protocol_laws::assert_fatal_never_applies(&outcome).await;
}

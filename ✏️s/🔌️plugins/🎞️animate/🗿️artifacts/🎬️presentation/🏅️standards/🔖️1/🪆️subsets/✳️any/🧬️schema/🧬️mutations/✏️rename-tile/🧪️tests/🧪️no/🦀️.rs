//! 🧪️ `rename-tile` fixture — `📖️no-ops-when-the-tile-already-has-that-name`.
//!
//! Source of truth is the committed JSON beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The derived encodings come from `fixtures generate`.
//!
//! ⚠️ `PresentationSnapshot` keeps its `(source, tiles)` in the composed `stdio.semio.presentation`
//! CHILD, and every content-changing diff mints a fresh `DefaultHasher`-digest handle no fixture can
//! hand-author — this tree pins the guard branches, which mint nothing.
//!
//! ✏️ `rename-tile` touches exactly one scalar (`tiles[id].name`) and its second guard is a plain
//! string equality on that scalar. The seeded deck's tile carries the committed payload's own
//! `new_name`, so the rename is a genuine identity — and the diff must stay empty rather than
//! re-minting a deck handle for a name that did not move.

use crate::mutations::{apply_presentation_mutation, inverse_presentation_mutation, PresentationMutation};
use crate::{default_figure_tile_source, FigureTileDraft, FigureTileFrame, PresentationDiff, PresentationSnapshot};
use semio_framework_test_fixtures::fixture_text;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/✏️rename-tile/🧪️no/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/✏️rename-tile/🧪️no/📸️snapshot/➡️after/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/✏️rename-tile/🧪️no/🦠️mutation/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/✏️rename-tile/🧪️no/🔺️diff/🔣️.json"));
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../🧪️tests/🧫️fixtures/🧬️mutations/✏️rename-tile/🧪️no/🎯️outcome/🔣️.json"));

fn mutation() -> PresentationMutation {
    semio_framework_pack_json::from_json_str((*MUTATION), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn expected_after() -> PresentationSnapshot {
    semio_framework_pack_json::from_json_str((*AFTER), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}

/// 🌱 The committed `⬅️before`, with its composed `presentation` child resolved to a deck whose one
/// tile already carries the committed payload's `new_name` — the identity the second guard catches.
fn before() -> PresentationSnapshot {
    let snapshot: PresentationSnapshot = semio_framework_pack_json::from_json_str((*BEFORE), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let PresentationMutation::RenameTile(payload) = mutation() else {
        panic!("no-ops-when-the-tile-already-has-that-name's committed mutation must be a rename-tile");
    };
    let tile = FigureTileDraft { id: payload.id, name: payload.new_name, crop: FigureTileFrame { x: 0.25, y: 0.25, width: 0.5, height: 0.5 } };
    assert_eq!(snapshot.tiles, vec![tile], "the committed before-snapshot must persist the tile already carrying the payload's new name");
    assert_eq!(snapshot.source, default_figure_tile_source(), "the committed before-snapshot's persisted source is the plugin's own default figure");
    snapshot
}

/// ▶️ Renaming a tile to the name it already has carries `before` to exactly the committed `after`,
/// leaving the composed deck handle untouched.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let snapshot = apply_presentation_mutation(&base, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "rename-tile/no-ops-when-the-tile-already-has-that-name: applied state differs from committed after-snapshot");
    assert_eq!(&snapshot.presentation.child_id, &base.presentation.child_id, "an identity rename must not re-mint the presentation handle");
}

/// 🔺️ The delta is exactly the committed all-null `PresentationDiff` — the guard returns after the target
/// lookup succeeded but before the name patch, so no deck is rebuilt for a non-change.
#[test]
fn produces_committed_diff() {
    let outcome = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff decodes");
    assert_eq!(produced, committed, "rename-tile/no-ops-when-the-tile-already-has-that-name: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(outcome.diff(), &PresentationDiff::default(), "an identity rename must carry the identity diff");
}

/// 🔣️ The committed diff is itself canonical and decodes to presentation's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: PresentationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str((*DIFF)).expect("committed diff reparses");
    assert_eq!(reencoded, original, "rename-tile/no-ops-when-the-tile-already-has-that-name: committed diff JSON is not canonical");
}

/// 🩹 The committed identity diff preserves the source, tile list and schema.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: PresentationDiff = semio_framework_pack_json::from_json_str((*DIFF), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(decoded, PresentationDiff::default(), "this operation must carry the identity diff");
    let produced = protocol::apply_diff_cold(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "rename-tile/no-ops-when-the-tile-already-has-that-name: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", (*BEFORE)), ("after", (*AFTER))] {
        let decoded: PresentationSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "rename-tile/no-ops-when-the-tile-already-has-that-name: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str((*MUTATION)).expect("mutation reparses");
    assert_eq!(reencoded, original, "rename-tile/no-ops-when-the-tile-already-has-that-name: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome holds: `no-op`, with one untargeted Warning `mutation.no-op` — a
/// presentation tile with an unchanged name is a warning, never the Error a missing id would raise.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str((*OUTCOME)).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("no-op"), "rename-tile/no-ops-when-the-tile-already-has-that-name declares a no-op outcome");
    let declared = outcome.get("messages").and_then(serde_json::Value::as_array).expect("the declared outcome carries messages");
    let produced = <PresentationMutation as protocol::ColdMutation<PresentationSnapshot>>::diff(&mutation(), &before());
    let messages = produced.messages();
    assert_eq!(messages.len(), declared.len(), "exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared[0].get("code").and_then(serde_json::Value::as_str), Some(messages[0].code.0.as_str()), "the declared code must match the emitted one");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning, "an unchanged name is a warning, not a missing-target error");
    assert!(messages[0].target.is_empty(), "rename-tile's no-op warning carries no target address");
}

/// ↩️ A no-op leaves undo history empty and preserves the full snapshot.
#[test]
fn unchanged_name_has_an_empty_inverse() {
    let base = before();
    let operation = mutation();
    let inverse = inverse_presentation_mutation(&base, &operation).expect("valid retained mutation inverse fixture");
    assert!(inverse.is_empty());
    assert_eq!(apply_presentation_mutation(&base, &operation).expect("no-op applies"), base);
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

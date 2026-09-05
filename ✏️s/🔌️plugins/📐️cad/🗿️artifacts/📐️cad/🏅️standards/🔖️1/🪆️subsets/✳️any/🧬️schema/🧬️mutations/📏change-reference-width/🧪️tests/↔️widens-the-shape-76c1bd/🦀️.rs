//! 🧪️ `change-reference-width` fixture — `↔️widens-the-shape-reference-plane`.
//!
//! Proves the world-width scalar is rewritten while the uniform `scale` factor is left alone.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> CadSnapshot {
    serde_json::from_str(BEFORE).expect("change-reference-width/widens-the-shape-reference-plane: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("change-reference-width/widens-the-shape-reference-plane: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("change-reference-width/widens-the-shape-reference-plane: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-reference-width applies to its committed before-snapshot")
}

/// ▶️ `change-reference-width` writes `width_world`; the uniform `scale` factor is a different field entirely.
#[semio_framework_async_macros::async_test]
async fn rewrites_the_world_width_without_touching_the_scale() {
    let after = applied();
    let rows = &after.references_by_model_definition_id["spatial.shape"];
    let reference = rows.iter().find(|reference| reference.id == "ref-1").expect("ref-1 survives");
    assert_eq!(reference.width_world, 12.0, "change-reference-width must set the addressed reference's world width");
    assert_eq!(reference.scale, Some(1.5), "change-reference-width must leave the uniform scale factor untouched");
    assert_eq!(reference.origin, [0.0, 0.0, 0.0], "change-reference-width must not move the reference");
    assert_eq!(after, expected_after(), "change-reference-width/widens-the-shape-reference-plane: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `change-reference-width` carrying BASE's width.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_original_world_width() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-reference-width inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ChangeReferenceWidth(step) => {
            assert_eq!((step.model_definition_id.as_str(), step.reference_id.as_str()), ("spatial.shape", "ref-1"), "the inverse must address the same reference in the same bucket");
            assert_eq!(step.new_width_world, 8.0, "the inverse must carry the pre-edit world width");
        }
        other => panic!("change-reference-width must invert to change-reference-width, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-reference-width/widens-the-shape-reference-plane: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-reference-width/widens-the-shape-reference-plane: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-reference-width/widens-the-shape-reference-plane: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-reference-width/widens-the-shape-reference-plane: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-reference-width`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-reference-width/widens-the-shape-reference-plane: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-reference-width/widens-the-shape-reference-plane: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let map = produced.diff().references_by_model_definition_id.as_ref().expect("change-reference-width fills the references map");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert_eq!(rows[0].width_world, 12.0, "the emitted bucket carries the whole post-patch reference row");
    assert_eq!(rows[0].scale, Some(1.5), "the emitted row keeps the scale factor the mutation did not address");
}

/// 🔺️ The sparse delta `change-reference-width` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the emitted row differs from BASE in `widthWorld` alone — the uniform `scale` factor is reproduced verbatim.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-reference-width/widens-the-shape-reference-plane: change-reference-width must emit one bucket whose row differs from BASE in `widthWorld` alone");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-reference-width/widens-the-shape-reference-plane: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-reference-width` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-reference-width/widens-the-shape-reference-plane: committed diff did not carry before to after");
}

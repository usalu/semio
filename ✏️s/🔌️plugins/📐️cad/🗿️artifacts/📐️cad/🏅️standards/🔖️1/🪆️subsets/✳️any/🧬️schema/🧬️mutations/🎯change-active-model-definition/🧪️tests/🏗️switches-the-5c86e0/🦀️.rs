//! 🧪️ `change-active-model-definition` fixture — `🏗️switches-the-active-pane-to-the-building-model`.
//!
//! Proves the active-pane selector is a plain persisted string that carries no content with it.
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
    serde_json::from_str(BEFORE).expect("change-active-model-definition/switches-the-active-pane-to-the-building-model: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("change-active-model-definition/switches-the-active-pane-to-the-building-model: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("change-active-model-definition/switches-the-active-pane-to-the-building-model: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("change-active-model-definition applies to its committed before-snapshot")
}

/// ▶️ `change-active-model-definition` writes one root string; no model slot, reference bucket or node moves with it.
#[semio_framework_async_macros::async_test]
async fn repoints_the_selector_without_moving_any_content() {
    let after = applied();
    assert_eq!(after.active_model_definition_id, "aec.building", "change-active-model-definition must repoint the selector");
    assert!(after.references_by_model_definition_id.contains_key("spatial.shape"), "change-active-model-definition must not migrate reference buckets between model definitions");
    assert!(!after.references_by_model_definition_id.contains_key("aec.building"), "change-active-model-definition must not conjure a bucket for the newly selected model definition");
    assert!(after.shape_model.is_some() && after.building_model.is_some(), "change-active-model-definition must not touch the fixed model slots");
    assert_eq!(after, expected_after(), "change-active-model-definition/switches-the-active-pane-to-the-building-model: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `change-active-model-definition` carrying BASE's selector.
#[semio_framework_async_macros::async_test]
async fn inverse_reselects_the_shape_model_definition() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "change-active-model-definition inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ChangeActiveModelDefinition(step) => assert_eq!(step.new_model_definition_id, "spatial.shape", "the inverse must carry the pre-edit selector"),
        other => panic!("change-active-model-definition must invert to change-active-model-definition, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("change-active-model-definition/switches-the-active-pane-to-the-building-model: inverse step applies");
    }
    assert_eq!(snapshot, base, "change-active-model-definition/switches-the-active-pane-to-the-building-model: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-active-model-definition/switches-the-active-pane-to-the-building-model: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-active-model-definition/switches-the-active-pane-to-the-building-model: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-active-model-definition`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "change-active-model-definition/switches-the-active-pane-to-the-building-model: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "change-active-model-definition/switches-the-active-pane-to-the-building-model: declared clean-applied but the diff builder reported {:?}", produced.messages());
    assert_eq!(produced.diff().active_model_definition_id.as_deref(), Some("aec.building"), "change-active-model-definition fills the root `active_model_definition_id` diff field");
    assert!(produced.diff().references_by_model_definition_id.is_none() && produced.diff().nodes.is_none(), "change-active-model-definition emits nothing but the selector");
}

/// 🔺️ The sparse delta `change-active-model-definition` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the ONLY populated field is the root selector string — no bucket, slot or node rides along.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-active-model-definition/switches-the-active-pane-to-the-building-model: change-active-model-definition must emit a diff whose sole populated field is `activeModelDefinitionId`");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-active-model-definition/switches-the-active-pane-to-the-building-model: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `change-active-model-definition` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-active-model-definition/switches-the-active-pane-to-the-building-model: committed diff did not carry before to after");
}

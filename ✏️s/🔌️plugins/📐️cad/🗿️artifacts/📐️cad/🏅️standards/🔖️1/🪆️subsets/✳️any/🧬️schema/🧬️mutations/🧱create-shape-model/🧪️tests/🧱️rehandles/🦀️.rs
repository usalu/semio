//! 🧪️ `create-shape-model` fixture — `🧱️rehandles`.
//!
//! Proves `create-shape-model` OVERWRITES an already-occupied fixed slot and that undo restores the displaced handle — it is not an insert-if-absent.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::CadMutation;
use crate::CadSnapshot;
use protocol::Mutation;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱create-shape-model/🧱️rehandles/🎯️outcome/🔣️.json");

fn before() -> CadSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("create-shape-model/rehandles-the-occupied-shape-slot: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("create-shape-model/rehandles-the-occupied-shape-slot: after snapshot decodes")
}
fn mutation() -> CadMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("create-shape-model/rehandles-the-occupied-shape-slot: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("create-shape-model applies to its committed before-snapshot")
}

/// ▶️ `create-shape-model` writes the fixed `shape_model` slot even when it is already occupied; the other three slots never move.
#[semio_framework_async_macros::async_test]
async fn replaces_the_shape_handle_in_place() {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let after = applied();
    let handle = after.shape_model.as_ref().expect("create-shape-model leaves the slot occupied");
    assert_eq!(handle.child_id, "cad-shape-2", "create-shape-model must install the payload's child id");
    assert_eq!(handle.target.to_uri(), "cad-shape-2!s.stdio.semio@v1/model", "create-shape-model must parse the payload target URI back into a real ArtifactRef");
    assert!(
        after.building_model.as_ref().map(|c| c.child_id.as_str()) == Some("cad-building-1")
            && after.energy_model.as_ref().map(|c| c.child_id.as_str()) == Some("cad-energy-1")
            && after.structure_classic_model.as_ref().map(|c| c.child_id.as_str()) == Some("cad-structure-1"),
        "create-shape-model must leave the other three fixed model slots untouched"
    );
    assert_eq!(after.drawings.len(), 1, "create-shape-model must not touch the drawings child collection");
    assert_eq!(after, expected_after(), "create-shape-model/rehandles-the-occupied-shape-slot: applied state differs from the committed after-snapshot");
}

/// ↩️ Because BASE's slot was occupied, the inverse is another `create-shape-model` carrying the DISPLACED handle — never a bare delete.
#[semio_framework_async_macros::async_test]
async fn inverse_reinstalls_the_displaced_shape_handle() {
    let base = before();
    let inverse = mutation().inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "create-shape-model inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateShapeModel(step) => {
            assert_eq!(step.child_id, "cad-shape-1", "the inverse must reinstall the handle create-shape-model displaced");
            assert_eq!(step.target, semio_framework_artifact_reference::ArtifactRef { artifact_id: "cad-shape-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "model".into() } }, "the inverse must carry the displaced handle's target URI");
        }
        other => panic!("create-shape-model over an OCCUPIED slot must invert to create-shape-model, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = protocol::apply_diff(step.diff(&snapshot).diff(), &snapshot).expect("create-shape-model/rehandles-the-occupied-shape-slot: inverse step applies");
    }
    assert_eq!(snapshot, base, "create-shape-model/rehandles-the-occupied-shape-slot: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-shape-model/rehandles-the-occupied-shape-slot: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-shape-model/rehandles-the-occupied-shape-slot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `create-shape-model`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "create-shape-model/rehandles-the-occupied-shape-slot: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "create-shape-model/rehandles-the-occupied-shape-slot: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let slot = produced.diff().shape_model.as_ref().expect("create-shape-model fills the `shape_model` slot diff");
    let handle = slot.as_ref().expect("create-shape-model's diff sets the slot to the occupied arm");
    assert_eq!(handle.child_id, "cad-shape-2", "the slot diff carries the payload child id");
    assert!(produced.diff().drawings.is_none() && produced.diff().nodes.is_none(), "create-shape-model emits nothing but its own slot field");
}

/// 🔺️ The sparse delta `create-shape-model` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the ONLY populated field is `shapeModel`, carrying the occupied arm — the other three fixed slots stay null even though this create overwrites one of them.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-shape-model/rehandles-the-occupied-shape-slot: create-shape-model must emit a diff whose sole populated field is `shapeModel`");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::diff::CadDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-shape-model/rehandles-the-occupied-shape-slot: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `create-shape-model` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::diff::CadDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes into the artifact's diff type");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-shape-model/rehandles-the-occupied-shape-slot: committed diff did not carry before to after");
}

/// ⚖️ The concrete inverse's diffs sum to exactly the negative of the forward diff, restoring the committed before-snapshot.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

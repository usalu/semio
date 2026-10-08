//! 🧪️ `delete-drawing` fixture — `🚫️removes-drawing-1`.
//!
//! Proves the addressed drawing handle is filtered out of the whole-list diff, leaving an empty collection.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::mutations::CadMutation;
use crate::CadSnapshot;
use protocol::Mutation;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧹delete-drawing/🚫️removes-drawing-1/🎯️outcome/🔣️.json");

fn before() -> CadSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-drawing/removes-drawing-1: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-drawing/removes-drawing-1: after snapshot decodes")
}
fn mutation() -> CadMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-drawing/removes-drawing-1: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("delete-drawing applies to its committed before-snapshot")
}

/// ▶️ `delete-drawing` drops one handle from the Vec-cardinality composition slot.
#[semio_framework_async_macros::async_test]
async fn filters_the_addressed_drawing_out_of_the_list() {
    let after = applied();
    assert!(after.drawings.is_empty(), "delete-drawing must remove the addressed handle, leaving the collection empty");
    assert!(after.shape_model.is_some() && after.building_model.is_some(), "delete-drawing must not touch the fixed model slots");
    assert_eq!(after.nodes.len(), 2, "delete-drawing must not cascade into the node tree");
    assert_eq!(after, expected_after(), "delete-drawing/removes-drawing-1: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `create-drawing` carrying the escrowed handle's id AND target URI.
#[semio_framework_async_macros::async_test]
async fn inverse_recreates_the_drawing_with_its_target() {
    let base = before();
    let inverse = mutation().inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "delete-drawing inverts to exactly one step");
    match &inverse[0] {
        CadMutation::CreateDrawing(step) => {
            assert_eq!(step.child_id, "cad-drawing-1", "the inverse must recreate the removed drawing id");
            assert_eq!(step.target, semio_framework_artifact_reference::ArtifactRef { artifact_id: "cad-drawing-1".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "drawing".into() } }, "the inverse must carry the removed handle's target URI, not a stub");
        }
        other => panic!("delete-drawing must invert to create-drawing, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = protocol::apply_diff(step.diff(&snapshot).diff(), &snapshot).expect("delete-drawing/removes-drawing-1: inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-drawing/removes-drawing-1: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-drawing/removes-drawing-1: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&(mutation()))).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-drawing/removes-drawing-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-drawing`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-drawing/removes-drawing-1: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "delete-drawing/removes-drawing-1: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let list = produced.diff().drawings.as_ref().expect("delete-drawing fills the drawings child list");
    assert!(list.values.is_empty(), "delete-drawing emits the WHOLE post-state list — empty here — rather than a removed-id delta");
    assert!(produced.diff().shape_model.is_none(), "delete-drawing must not emit a model-slot diff");
}

/// 🔺️ The sparse delta `delete-drawing` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here `drawings` carries the WHOLE post-state list, empty here — the removed id never appears in the diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-drawing/removes-drawing-1: delete-drawing must emit the whole post-state drawings list rather than a removed-id delta");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::diff::CadDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-drawing/removes-drawing-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `delete-drawing` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::diff::CadDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes into the artifact's diff type");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-drawing/removes-drawing-1: committed diff did not carry before to after");
}

/// ⚖️ The concrete inverse's diffs sum to exactly the negative of the forward diff, restoring the committed before-snapshot.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

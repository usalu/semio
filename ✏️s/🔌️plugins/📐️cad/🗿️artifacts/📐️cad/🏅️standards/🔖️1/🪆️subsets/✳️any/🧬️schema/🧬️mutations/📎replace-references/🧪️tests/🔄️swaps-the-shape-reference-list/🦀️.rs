//! 🧪️ `replace-references` fixture — `🔄️swaps-the-shape-reference-list`.
//!
//! Proves the whole per-model-definition list is substituted — ids present before but absent from the payload simply vanish.
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
    serde_json::from_str(BEFORE).expect("replace-references/swaps-the-shape-reference-list: before snapshot decodes")
}
fn expected_after() -> CadSnapshot {
    serde_json::from_str(AFTER).expect("replace-references/swaps-the-shape-reference-list: after snapshot decodes")
}
fn mutation() -> CadMutation {
    serde_json::from_str(MUTATION).expect("replace-references/swaps-the-shape-reference-list: mutation decodes")
}
fn applied() -> CadSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("replace-references applies to its committed before-snapshot")
}

/// ▶️ `replace-references` is a wholesale list substitution, not an id-keyed merge: ref-1 disappears because the payload omits it.
#[semio_framework_async_macros::async_test]
async fn substitutes_the_whole_bucket_rather_than_merging() {
    let after = applied();
    let rows = &after.references_by_model_definition_id["spatial.shape"];
    assert_eq!(rows.iter().map(|reference| reference.id.as_str()).collect::<Vec<_>>(), vec!["ref-2"], "replace-references must substitute the whole bucket — ref-1 is dropped because the payload omits it");
    assert_eq!(rows[0].width_world, 16.0, "replace-references must store the payload rows verbatim");
    assert!(rows[0].scale.is_none(), "replace-references must store the payload rows verbatim, absent optionals included");
    assert_eq!(after.references_by_model_definition_id.len(), 1, "replace-references must not create other model-definition buckets");
    assert_eq!(after, expected_after(), "replace-references/swaps-the-shape-reference-list: applied state differs from the committed after-snapshot");
}

/// ↩️ The inverse is a `replace-references` carrying BASE's entire list back.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_original_reference_list() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert_eq!(inverse.len(), 1, "replace-references inverts to exactly one step");
    match &inverse[0] {
        CadMutation::ReplaceReferences(step) => {
            assert_eq!(step.model_definition_id, "spatial.shape", "the inverse must address the same bucket");
            assert_eq!(step.references.iter().map(|reference| reference.id.as_str()).collect::<Vec<_>>(), vec!["ref-1"], "the inverse must carry BASE's entire list, not a diff of it");
            assert_eq!(step.references[0].width_world, 8.0, "the inverse must carry each restored row in full");
        }
        other => panic!("replace-references must invert to replace-references, got {other:?}"),
    }
    let mut snapshot = applied();
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("replace-references/swaps-the-shape-reference-list: inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-references/swaps-the-shape-reference-list: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: CadSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-references/swaps-the-shape-reference-list: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-references/swaps-the-shape-reference-list: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `replace-references`'s own diff builder actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "replace-references/swaps-the-shape-reference-list: this fixture declares an applied outcome");
    let base = before();
    let produced = mutation().diff(&base);
    assert!(produced.messages().is_empty(), "replace-references/swaps-the-shape-reference-list: declared clean-applied but the diff builder reported {:?}", produced.messages());
    let map = produced.diff().references_by_model_definition_id.as_ref().expect("replace-references fills the references map");
    assert_eq!(map.len(), 1, "replace-references emits only the addressed model-definition bucket");
    let rows = map.get("spatial.shape").expect("the addressed bucket is keyed by its model definition id");
    assert_eq!(rows.len(), 1, "replace-references emits the payload list verbatim as the bucket's new value");
    assert_eq!(rows[0].id, "ref-2", "the emitted bucket is the payload list, not a merge with BASE");
}

/// 🔺️ The sparse delta `replace-references` produces is exactly the committed diff — the most load-bearing
/// assertion in the fixture, because it pins WHICH fields the mutation may touch, not merely that the
/// end state matches. Here the emitted bucket IS the payload list verbatim — `ref-1` appears nowhere, which is exactly how a wholesale substitution differs from a merge.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-references/swaps-the-shape-reference-list: replace-references must emit the payload list verbatim as the bucket's new value");
}

/// 🔣️ The committed diff decodes into `CadDiff` and re-encodes byte-for-byte: `CadDiff` has
/// `#[serde(rename_all = "camelCase", default)]` with no `skip_serializing_if`, so EVERY field is on
/// the wire and the untouched ones must be committed as explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let reencoded = serde_json::to_value(&decoded).expect("committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-references/swaps-the-shape-reference-list: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields `after` — the diff is a complete
/// description of the change `replace-references` makes, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::cad::diff::CadDiff = serde_json::from_str(DIFF).expect("committed diff decodes into the artifact's diff type");
    let produced = decoded.apply(&before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-references/swaps-the-shape-reference-list: committed diff did not carry before to after");
}

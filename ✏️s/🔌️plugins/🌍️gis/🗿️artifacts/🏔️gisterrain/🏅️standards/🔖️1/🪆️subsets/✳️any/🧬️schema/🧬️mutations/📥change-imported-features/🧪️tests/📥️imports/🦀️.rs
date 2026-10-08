//! 🪪️ Committed Terrain mutation vectors preserve exact owned mesh identity.

use crate::diff::GisTerrainDiff;
use crate::mutations::{apply_gis_terrain_mutation, inverse_gis_terrain_mutation, GisTerrainMutation};
use crate::GisTerrainSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥change-imported-features/📥️imports/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥change-imported-features/📥️imports/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥change-imported-features/📥️imports/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥change-imported-features/📥️imports/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📥change-imported-features/📥️imports/🎯️outcome/🔣️.json");

/// 🪆️ Verifies the declared parent edit while retaining the committed mesh handle.
fn before() -> GisTerrainSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
/// 🪆️ Verifies the declared parent edit while retaining the committed mesh handle.
fn expected_after() -> GisTerrainSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> GisTerrainMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// 🪆️ Verifies the declared parent edit while retaining the committed mesh handle.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_gis_terrain_mutation(&mut snapshot, &mutation()).expect("change-imported-features applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-imported-features/imports-harbor-position-descriptor: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.mesh, base.mesh, "parent scalar edits preserve owned mesh identity");
    assert!(snapshot.mesh.is_some(), "change-imported-features/imports-harbor-position-descriptor: the terrain always owns a mesh child — the slot is never emptied by an edit");
}

/// 🪆️ Verifies the declared parent edit while retaining the committed mesh handle.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_gis_terrain_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    apply_gis_terrain_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_gis_terrain_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-imported-features/imports-harbor-position-descriptor: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GisTerrainSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-imported-features/imports-harbor-position-descriptor: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-imported-features/imports-harbor-position-descriptor: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `change-imported-features` actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "change-imported-features/imports-harbor-position-descriptor: this fixture pins an applied outcome");
    let produced = <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-imported-features/imports-harbor-position-descriptor: an applied outcome with no declared messages must emit none, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_gis_terrain_mutation(&mut snapshot, &mutation()).expect("change-imported-features/imports-harbor-position-descriptor: declared applied but the mutation was rejected");
    assert_ne!(snapshot, before(), "change-imported-features/imports-harbor-position-descriptor: an applied change-imported-features must actually change the document");
}

/// 🔺️ The sparse delta `change-imported-features` produces is exactly the committed diff — it pins WHICH of the
/// terrain's two artifact-lane fields the mutation is allowed to touch, not merely the end state.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-imported-features/imports-harbor-position-descriptor: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisTerrainDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-imported-features/imports-harbor-position-descriptor: committed diff JSON is not canonical");
}

/// 🪆️ Verifies the declared parent edit while retaining the committed mesh handle.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisTerrainDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-imported-features/imports-harbor-position-descriptor: committed diff did not carry before to after");
}

/// 🪆️ Verifies the declared parent edit while retaining the committed mesh handle.
#[semio_framework_async_macros::async_test]
async fn changes_only_the_imported_descriptor_string_and_inverts_to_the_base_string() {
    let base = before();
    let after = expected_after();
    let produced = <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::diff(&mutation(), &base);
    assert!(produced.messages().is_empty(), "change-imported-features/imports-harbor-position-descriptor: a genuinely different import must be diagnostic-free (the no-op warning is the other branch), got {:?}", produced.messages());
    assert_eq!(produced.diff().imported_map.as_ref().map(|change|&change.value), Some(&after.imported_map), "change-imported-features/imports-harbor-position-descriptor: the delta carries the payload's own descriptor string verbatim");
    assert!(produced.diff().exaggeration.is_none(), "change-imported-features/imports-harbor-position-descriptor: the terrain's OTHER artifact-lane field must stay out of the delta");
    assert_eq!(after.exaggeration, base.exaggeration, "change-imported-features/imports-harbor-position-descriptor: the untouched exaggeration must survive the edit verbatim");
    assert_eq!(base.imported_map, None, "change-imported-features/imports-harbor-position-descriptor: the pre-import state is the literal JSON string \"null\", not an empty field");
    let inverse = inverse_gis_terrain_mutation(&base, &mutation()).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "change-imported-features/imports-harbor-position-descriptor: an import undoes with exactly one step, got {inverse:?}");
    let GisTerrainMutation::ChangeImportedFeatures(undo) = &inverse[0] else {
        panic!("change-imported-features/imports-harbor-position-descriptor: the inverse must be another change-imported-features, got {:?}", inverse[0]);
    };
    assert_eq!(undo.new_imported_map, None, "change-imported-features/imports-harbor-position-descriptor: the inverse restores BASE's descriptor string, not the diff's");
    let semantics = <GisTerrainMutation as protocol::SemanticMutation<GisTerrainSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("change", "imported-features", "change-imported-features", "ChangedImportedFeatures"),
        "change-imported-features/imports-harbor-position-descriptor: the fixture must be bound to change-imported-features' own descriptor"
    );
}

/// ⚖️ The concrete inverse's diffs sum to exactly the negative of the forward diff, restoring the committed before-snapshot.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

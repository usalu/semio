//! 🧪️ `delete-region` fixture — `🚫️removes-old-town-region`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, never here.
//!
//! 🧹 `delete-region` writes ONE `removed` id into `regions`. The surviving region shares the removed one's
//! `kind`, so this case also pins that the delta addresses IDENTITY and never a payload predicate — a
//! `kind`-matching sibling must be left standing. Its inverse is BASE-derived.
//!
//! 🧩️ Committed snapshots preserve the stable drawing and value child identities across edits.
//! Feature collections and their sparse deltas are asserted directly against the neutral fixtures.

use crate::diff::GisMapDiff;
use crate::mutations::{apply_gis_map_mutation, inverse_gis_map_mutation, GisMapMutation};
use crate::GisMapSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

/// 🗺️ Decodes the committed snapshot with its stable child identities.
fn before() -> GisMapSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
/// 🎯️ Decodes the expected snapshot without normalizing its child identities.
fn expected_after() -> GisMapSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> GisMapMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ Applies the mutation while preserving child identities and matching the committed snapshot.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("delete-region applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-region/removes-old-town-region: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.drawing.child_id, base.drawing.child_id, "delete-region/removes-old-town-region: editing regions must preserve the stable drawing identity");
    assert_eq!(snapshot.value.child_id, base.value.child_id, "delete-region/removes-old-town-region: editing regions must preserve the stable value identity");
    assert!(snapshot.image.is_none(), "delete-region/removes-old-town-region: gis carries no raster basemap, so the image child stays absent");
}

/// ↩️ Restores the complete committed snapshot through the inverse mutations.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_gis_map_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_gis_map_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_gis_map_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-region/removes-old-town-region: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: GisMapSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-region/removes-old-town-region: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-region/removes-old-town-region: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what `delete-region` actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "applied", "delete-region/removes-old-town-region: this fixture pins an applied outcome");
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-region/removes-old-town-region: an applied outcome with no declared messages must emit none, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_gis_map_mutation(&mut snapshot, &mutation()).expect("delete-region/removes-old-town-region: declared applied but the mutation was rejected");
    assert_ne!(snapshot, before(), "delete-region/removes-old-town-region: an applied delete-region must actually change the document");
}

/// 🔺️ The sparse delta `delete-region` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH of `positions`/`routes`/`regions` the
/// mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let outcome = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-region/removes-old-town-region: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-region/removes-old-town-region: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is
/// a complete description of the change, not a summary of it. `GisMapDiff::apply` re-derives the
/// composed children itself, exactly as `apply_gis_map_mutation` does.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: GisMapDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-region/removes-old-town-region: committed diff did not carry before to after");
}

/// 🧹 `delete-region` writes ONE `removed` id into `regions`. The surviving region shares the removed one's
/// `kind`, so this case also pins that the delta addresses IDENTITY and never a payload predicate — a
/// `kind`-matching sibling must be left standing. Its inverse is BASE-derived.
#[semio_framework_async_macros::async_test]
async fn removes_exactly_the_addressed_region_and_leaves_its_kind_sibling() {
    let base = before();
    let after = expected_after();
    let produced = <GisMapMutation as protocol::Mutation<GisMapSnapshot>>::diff(&mutation(), &base);
    assert!(produced.messages().is_empty(), "delete-region/removes-old-town-region: deleting a present id must be diagnostic-free, got {:?}", produced.messages());
    let delta = produced.diff().regions.as_ref().expect("delete-region writes a regions delta");
    assert_eq!(delta.removed, vec!["region-old-town".to_string()], "delete-region/removes-old-town-region: exactly the payload's own id is removed");
    assert_eq!(
        after.regions.iter().map(|feature| feature.id.clone()).collect::<Vec<_>>(),
        vec!["region-harbor-district".to_string()],
        "delete-region/removes-old-town-region: the sibling region of the same kind must survive — deletion is by id, never by payload"
    );
    assert!(delta.added.is_empty() && delta.patched.is_empty() && delta.reordered.is_none(), "delete-region/removes-old-town-region: a delete must not add, patch or reorder anything, got {delta:?}");
    assert!(produced.diff().positions.is_none() && produced.diff().routes.is_none(), "delete-region/removes-old-town-region: delete-region must never touch the positions or routes collections");
    let inverse = inverse_gis_map_mutation(&base, &mutation());
    assert_eq!(inverse.len(), 1, "delete-region/removes-old-town-region: a delete undoes with exactly one step, got {inverse:?}");
    let GisMapMutation::CreateRegion(undo) = &inverse[0] else {
        panic!("delete-region/removes-old-town-region: the inverse must be a create-region, got {:?}", inverse[0]);
    };
    assert_eq!(undo.index, 1, "delete-region/removes-old-town-region: the inverse restores the region at the index BASE held it at");
    assert_eq!(undo.item, base.regions[1], "delete-region/removes-old-town-region: the inverse re-creates the whole BASE feature, payload included");
    let semantics = <GisMapMutation as protocol::SemanticMutation<GisMapSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("delete", "region", "delete-region", "DeletedRegion"), "delete-region/removes-old-town-region: the fixture must be bound to delete-region's own descriptor");
}

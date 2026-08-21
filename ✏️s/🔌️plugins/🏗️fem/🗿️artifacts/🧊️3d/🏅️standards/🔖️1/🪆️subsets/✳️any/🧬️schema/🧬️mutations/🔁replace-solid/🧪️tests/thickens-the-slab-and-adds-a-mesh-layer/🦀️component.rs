//! 🧪️ `replace-solid` fixture — `thickens-the-slab-and-adds-a-mesh-layer`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Height, layer count and mesh size all move together in one whole-value swap — the footprint must be re-stated unchanged.

use crate::artifacts::fem3d::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Fem3dSnapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `replace-solid` thickens the slab in place and carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("replace-solid applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.solids.len(), 1, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: a replacement must not change the solid count");
    assert_eq!(snapshot.solids[0].height, 0.375, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: the new thickness must survive the round trip exactly");
    assert_eq!(snapshot.solids[0].outline, before().solids[0].outline, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: the footprint must be re-stated unchanged by the whole-value swap");
}

/// ↩️ The inverse is a `replace-solid` carrying the thin two-layer slab recovered from `base`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = serde_json::to_value(&decoded_mutation).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-solid/thickens-the-slab-and-adds-a-mesh-layer: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `solids.patched` entry keyed by `slab`.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().solids.as_ref().expect("solids delta").patched.len(), 1, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: exactly one solid may be patched");
    assert!(outcome.diff().solids.as_ref().expect("solids delta").added.is_empty(), "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: a replacement is never an addition");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `solids.patched` entry on `before` must thicken the slab in its own slot.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before())
        .expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-solid/thickens-the-slab-and-adds-a-mesh-layer: committed diff did not carry before to after");
}

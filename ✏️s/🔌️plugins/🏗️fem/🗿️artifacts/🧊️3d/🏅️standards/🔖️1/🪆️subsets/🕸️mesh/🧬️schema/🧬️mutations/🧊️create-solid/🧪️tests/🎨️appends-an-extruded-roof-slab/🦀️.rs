//! 🧪️ `create-solid` fixture — `🎨️appends-an-extruded-roof-slab`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! A footprint is extruded from `baseZ` over two mesh layers — the extrusion parameters are stored, never re-derived at solve time.

use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `create-solid` appends the roof slab and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("create-solid applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-solid/appends-an-extruded-roof-slab: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.solids.len(), 1, "create-solid/appends-an-extruded-roof-slab: exactly one solid may be coined");
    assert_eq!(snapshot.solids[0].base_z, 3.5, "create-solid/appends-an-extruded-roof-slab: the extrusion base elevation must survive the round trip exactly");
    assert_eq!(snapshot.solids[0].layers, 2, "create-solid/appends-an-extruded-roof-slab: the layer count is an integer facet and must round-trip as one");
}

/// ↩️ The inverse is a `delete-solid` of `slab`, restoring the solid-free document.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-solid/appends-an-extruded-roof-slab: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-solid/appends-an-extruded-roof-slab: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-solid/appends-an-extruded-roof-slab: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "create-solid/appends-an-extruded-roof-slab: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "create-solid/appends-an-extruded-roof-slab: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "create-solid/appends-an-extruded-roof-slab: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("create-solid/appends-an-extruded-roof-slab: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `solids.added` entry carrying the footprint and the extrusion parameters.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().solids.is_some(), "create-solid/appends-an-extruded-roof-slab: the created solid must surface in the solids delta");
    assert!(outcome.diff().materials.is_none(), "create-solid/appends-an-extruded-roof-slab: the material the solid validates against is read-only");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-solid/appends-an-extruded-roof-slab: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-solid/appends-an-extruded-roof-slab: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `solids.added` entry on `before` must reproduce the slab with its geometry intact.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-solid/appends-an-extruded-roof-slab: committed diff did not carry before to after");
}

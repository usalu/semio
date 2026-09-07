//! 🧪️ `create-support` fixture — `🏗️hall-new-pin-c033f2`.
//!
//! Source of truth is the committed JSON bundle beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏭️ `⬅️before` is the GLULAM WORKSHOP HALL, the second real fem3d model this artifact carries
//! (ticket `26/09/06/FEM-PLUGIN-END-TO-END`): a 12 m span × 12 m long two-bay GL24h portal hall,
//! eaves at 4.2 m and ridge at 6.5 m, braced by M24 steel rods and standing on a 350 mm C25/30
//! raft slab with a machine pit. Every quantity is SI — Pa, m, m², m⁴, kg/m³, N, N/m.
//!
//! The free upper set-out point is pinned; the node it names already exists, which is the one thing this verb checks.

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

/// ▶️ The mutation carries the committed hall from `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("create-support applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-support/hall-new-pin-c033f2: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.supports.len(), 8, "create-support/hall-new-pin-c033f2: the new pin is appended behind the reserved clamp");
    assert_eq!(snapshot.supports[7].fixed.len(), 3, "create-support/hall-new-pin-c033f2: a pin restrains the three translations and nothing else");
}

/// 🔀️ This verb writes exactly ONE of the nine members; an implementation that re-derived a sibling
/// collection on every edit would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("forward applies");
    assert_eq!(snapshot.nodes, base.nodes, "create-support/hall-new-pin-c033f2: nodes must not move when this verb runs");
    assert_eq!(snapshot.elements, base.elements, "create-support/hall-new-pin-c033f2: elements must not move when this verb runs");
    assert_eq!(snapshot.materials, base.materials, "create-support/hall-new-pin-c033f2: materials must not move when this verb runs");
    assert_eq!(snapshot.sections, base.sections, "create-support/hall-new-pin-c033f2: sections must not move when this verb runs");
    assert_eq!(snapshot.solids, base.solids, "create-support/hall-new-pin-c033f2: solids must not move when this verb runs");
    assert_ne!(snapshot.supports, base.supports, "create-support/hall-new-pin-c033f2: supports is the one member this verb writes");
    assert_eq!(snapshot.load_cases, base.load_cases, "create-support/hall-new-pin-c033f2: load_cases must not move when this verb runs");
    assert_eq!(snapshot.combinations, base.combinations, "create-support/hall-new-pin-c033f2: combinations must not move when this verb runs");
    assert_eq!(snapshot.analysis, base.analysis, "create-support/hall-new-pin-c033f2: analysis must not move when this verb runs");
}

/// ↩️ Applying the computed inverse after the forward step lands back on the committed `before`.
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
    assert_eq!(snapshot, base, "create-support/hall-new-pin-c033f2: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome holds: applied, with no diagnostic at all.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "create-support/hall-new-pin-c033f2: this vector declares an applied outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "create-support/hall-new-pin-c033f2: a clean application raises no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The produced delta is exactly the committed sparse `🔺️diff/🔣️.json`.
#[test]
fn produces_committed_diff() {
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-support/hall-new-pin-c033f2: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-support/hall-new-pin-c033f2: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed delta on `before` reproduces the committed `after` on its own.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem3d::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem3d::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-support/hall-new-pin-c033f2: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-support/hall-new-pin-c033f2: committed {label} JSON is not canonical");
    }
    let reencoded = dsl::ToValue::to_value(&mutation());
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-support/hall-new-pin-c033f2: committed mutation JSON is not canonical");
}

/// 🪪️ The fixture is bound to this very kind's own semantic descriptor, never a sibling's.
#[test]
fn semantics_bind_the_declared_kind() {
    let semantics = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("create", "support", "create-support", "CreatedSupport"), "create-support/hall-new-pin-c033f2: the fixture must be bound to create-support's own descriptor");
}

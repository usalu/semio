//! 🧪️ `change-load-case-self-weight` fixture — `🏗️hall-crane-sw-978370`.
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
//! The crane case starts carrying the runway's own weight; its load list stays empty.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::Fem3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall-crane-sw-978370/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall-crane-sw-978370/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall-crane-sw-978370/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall-crane-sw-978370/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall-crane-sw-978370/🎯️outcome/🔣️.json");

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
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("change-load-case-self-weight applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-load-case-self-weight/hall-crane-sw-978370: applied state differs from committed after-snapshot");
    assert!(snapshot.load_cases[3].self_weight, "change-load-case-self-weight/hall-crane-sw-978370: the crane case must now carry self-weight");
}

/// 🔀️ This verb writes exactly ONE of the nine members; an implementation that re-derived a sibling
/// collection on every edit would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("forward applies");
    assert_eq!(snapshot.nodes, base.nodes, "change-load-case-self-weight/hall-crane-sw-978370: nodes must not move when this verb runs");
    assert_eq!(snapshot.elements, base.elements, "change-load-case-self-weight/hall-crane-sw-978370: elements must not move when this verb runs");
    assert_eq!(snapshot.materials, base.materials, "change-load-case-self-weight/hall-crane-sw-978370: materials must not move when this verb runs");
    assert_eq!(snapshot.sections, base.sections, "change-load-case-self-weight/hall-crane-sw-978370: sections must not move when this verb runs");
    assert_eq!(snapshot.solids, base.solids, "change-load-case-self-weight/hall-crane-sw-978370: solids must not move when this verb runs");
    assert_eq!(snapshot.supports, base.supports, "change-load-case-self-weight/hall-crane-sw-978370: supports must not move when this verb runs");
    assert_ne!(snapshot.load_cases, base.load_cases, "change-load-case-self-weight/hall-crane-sw-978370: load_cases is the one member this verb writes");
    assert_eq!(snapshot.combinations, base.combinations, "change-load-case-self-weight/hall-crane-sw-978370: combinations must not move when this verb runs");
    assert_eq!(snapshot.analysis, base.analysis, "change-load-case-self-weight/hall-crane-sw-978370: analysis must not move when this verb runs");
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
    assert_eq!(snapshot, base, "change-load-case-self-weight/hall-crane-sw-978370: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome holds: applied, with no diagnostic at all.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "change-load-case-self-weight/hall-crane-sw-978370: this vector declares an applied outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "change-load-case-self-weight/hall-crane-sw-978370: a clean application raises no diagnostic, got {:?}", produced.messages());
}

/// 🔺️ The produced delta is exactly the committed sparse `🔺️diff/🔣️.json`.
#[test]
fn produces_committed_diff() {
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-load-case-self-weight/hall-crane-sw-978370: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-load-case-self-weight/hall-crane-sw-978370: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed delta on `before` reproduces the committed `after` on its own.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-load-case-self-weight/hall-crane-sw-978370: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-load-case-self-weight/hall-crane-sw-978370: committed {label} JSON is not canonical");
    }
    let reencoded = dsl::ToValue::to_value(&mutation());
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-load-case-self-weight/hall-crane-sw-978370: committed mutation JSON is not canonical");
}

/// 🪪️ The fixture is bound to this very kind's own semantic descriptor, never a sibling's.
#[test]
fn semantics_bind_the_declared_kind() {
    let semantics = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("change", "load-case", "change-load-case-self-weight", "ChangedLoadCaseSelfWeight"),
        "change-load-case-self-weight/hall-crane-sw-978370: the fixture must be bound to change-load-case-self-weight's own descriptor"
    );
}

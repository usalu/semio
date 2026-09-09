//! 🧪️ `replace-material` fixture — `⏸️same-material-950f90`.
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
//! Re-declaring C24 with its own five properties is a no-op WARNING; only an uncatalogued grade is `target-missing`.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::Fem3dSnapshot;

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

/// ▶️ A no-op still APPLIES; it simply writes nothing, so `after` is the committed `before` again.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("replace-material's identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "replace-material/same-material-950f90: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "replace-material/same-material-950f90: a no-op must leave every one of the nine members exactly as it found them");
}

/// ⚠️ The branch this vector pins: a Warning `mutation.no-op` with an EMPTY diff and no target
/// address — `MutationOutcome::empty().warn(..)` is the 2-arg builder, which attaches none.
#[test]
fn a_redundant_write_is_a_warning_not_a_rejection() {
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "replace-material/same-material-950f90: a no-op must carry the identity diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "replace-material/same-material-950f90: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "replace-material/same-material-950f90: a redundant write is reported as no-op, never as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "replace-material/same-material-950f90: a no-op is a Warning — the mutation still applies");
    assert!(messages[0].target.is_empty(), "replace-material/same-material-950f90: the 2-arg warn builder attaches no target address");
}

/// ↩️ The inverse of a no-op is itself a no-op, so the round trip is the identity twice over.
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
    assert_eq!(snapshot, base, "replace-material/same-material-950f90: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome holds, code and level together.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "replace-material/same-material-950f90: a no-op declares an applied outcome");
    let declared = outcome.get("messages").and_then(dsl::DslValue::as_array).expect("the declared outcome carries messages");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(declared.len(), produced.messages().len(), "replace-material/same-material-950f90: the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("code").and_then(dsl::DslValue::as_str), Some(produced.messages()[0].code.0.as_str()), "replace-material/same-material-950f90: the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(dsl::DslValue::as_str), Some("warn"), "replace-material/same-material-950f90: the declared level must name the Warning the builder raises");
}

/// 🔺️ The produced delta is exactly the committed all-null `🔺️diff/🔣️.json`.
#[test]
fn produces_committed_diff() {
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-material/same-material-950f90: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed identity diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-material/same-material-950f90: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed identity delta on `before` reproduces the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-material/same-material-950f90: committed diff did not carry before to after");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-material/same-material-950f90: committed {label} JSON is not canonical");
    }
    let reencoded = dsl::ToValue::to_value(&mutation());
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-material/same-material-950f90: committed mutation JSON is not canonical");
}

/// 🪪️ The fixture is bound to this very kind's own semantic descriptor, never a sibling's.
#[test]
fn semantics_bind_the_declared_kind() {
    let semantics = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&mutation());
    assert_eq!(
        (semantics.verb, semantics.entity, semantics.kind, semantics.record),
        ("replace", "material", "replace-material", "ReplacedMaterial"),
        "replace-material/same-material-950f90: the fixture must be bound to replace-material's own descriptor"
    );
}

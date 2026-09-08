//! 🧪️ `delete-material` fixture — `🗑️drops-the-spare-00e964`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening), the
//! SECOND real-world fem2d model — the first is the timber portal frame the subset-level
//! differential cases share. Every value is in SI base units.
//!
//! 🗑️ The S235 spare is trailing and referenced by no element and no region, so this delete leaves nothing dangling and inverts exactly.

use crate::mutations::Fem2dMutation;
use crate::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `delete-material` carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-material applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-material/drops-the-spare-00e964: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, base, "delete-material/drops-the-spare-00e964: the forward mutation left the model untouched, so nothing was proved");
    assert_eq!(snapshot.materials.len(), 2, "delete-material/drops-the-spare-00e964: the materials collection must end up 2 records long");
    assert!(!snapshot.materials.iter().any(|item| item.id == "steel_s235_spare"), "delete-material/drops-the-spare-00e964: no record named steel_s235_spare may survive");
}

/// ↩️ Applying the computed inverse after the forward step lands back on `before`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "delete-material/drops-the-spare-00e964: delete-material undoes with exactly one step, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-material/drops-the-spare-00e964: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — applied, with no diagnostic at all — is what this kind really emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "delete-material/drops-the-spare-00e964 declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-material/drops-the-spare-00e964: a clean application raises no diagnostic, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-material/drops-the-spare-00e964: declared applied but the mutation was rejected");
}

/// 🔀️ Each verb writes exactly ONE of the nine members. An after-snapshot comparison cannot make
/// this check on its own: an implementation that re-derived a sibling collection on every edit
/// would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-material applies to its committed before-snapshot");
    assert_eq!(snapshot.nodes, base.nodes, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but nodes moved");
    assert_eq!(snapshot.elements, base.elements, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but elements moved");
    assert_eq!(snapshot.regions, base.regions, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but regions moved");
    assert_eq!(snapshot.sections, base.sections, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but sections moved");
    assert_eq!(snapshot.supports, base.supports, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but supports moved");
    assert_eq!(snapshot.load_cases, base.load_cases, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but loadCases moved");
    assert_eq!(snapshot.combinations, base.combinations, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but combinations moved");
    assert_eq!(snapshot.analysis, base.analysis, "delete-material/drops-the-spare-00e964: this verb writes materials and nothing else, but analysis moved");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-material/drops-the-spare-00e964: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-material/drops-the-spare-00e964: committed mutation JSON is not canonical");
}

/// 🔺️ The delta must be exactly the committed one, on exactly the `materials` slot.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    let delta = outcome.diff().materials.as_ref().expect("materials delta");
    assert_eq!((delta.added.len(), delta.removed.len(), delta.patched.len()), (0, 1, 0), "delete-material/drops-the-spare-00e964: the delta must be exactly one removed entry");
    assert!(delta.reordered.is_none(), "delete-material/drops-the-spare-00e964: no verb in this vocabulary re-orders a collection");
    assert!(outcome.diff().nodes.is_none(), "delete-material/drops-the-spare-00e964: no nodes delta may be opened by this verb");
    assert!(outcome.diff().elements.is_none(), "delete-material/drops-the-spare-00e964: no elements delta may be opened by this verb");
    assert!(outcome.diff().regions.is_none(), "delete-material/drops-the-spare-00e964: no regions delta may be opened by this verb");
    assert!(outcome.diff().sections.is_none(), "delete-material/drops-the-spare-00e964: no sections delta may be opened by this verb");
    assert!(outcome.diff().supports.is_none(), "delete-material/drops-the-spare-00e964: no supports delta may be opened by this verb");
    assert!(outcome.diff().load_cases.is_none(), "delete-material/drops-the-spare-00e964: no loadCases delta may be opened by this verb");
    assert!(outcome.diff().combinations.is_none(), "delete-material/drops-the-spare-00e964: no combinations delta may be opened by this verb");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-material/drops-the-spare-00e964: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-material/drops-the-spare-00e964: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed delta on `before` must reproduce the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-material/drops-the-spare-00e964: committed diff did not carry before to after");
}

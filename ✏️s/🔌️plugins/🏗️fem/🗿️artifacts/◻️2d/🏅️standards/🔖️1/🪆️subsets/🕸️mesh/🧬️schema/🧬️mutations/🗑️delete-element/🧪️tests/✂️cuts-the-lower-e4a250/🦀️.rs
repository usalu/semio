//! 🧪️ `delete-element` fixture — `✂️cuts-the-lower-e4a250`.
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
//! ✂️ `br1` is the TRAILING element, so the `create-element` inverse — which appends — lands it back in its own slot.

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::artifacts::fem2d::Fem2dSnapshot;

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

/// ▶️ `delete-element` carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-element applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-element/cuts-the-lower-e4a250: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, base, "delete-element/cuts-the-lower-e4a250: the forward mutation left the model untouched, so nothing was proved");
    assert_eq!(snapshot.elements.len(), 6, "delete-element/cuts-the-lower-e4a250: the elements collection must end up 6 records long");
    assert!(!snapshot.elements.iter().any(|item| crate::artifacts::fem2d::element_id(item) == "br1"), "delete-element/cuts-the-lower-e4a250: no record named br1 may survive");
}

/// ↩️ Applying the computed inverse after the forward step lands back on `before`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "delete-element/cuts-the-lower-e4a250: delete-element undoes with exactly one step, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-element/cuts-the-lower-e4a250: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — applied, with no diagnostic at all — is what this kind really emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "delete-element/cuts-the-lower-e4a250 declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "delete-element/cuts-the-lower-e4a250: a clean application raises no diagnostic, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-element/cuts-the-lower-e4a250: declared applied but the mutation was rejected");
}

/// 🔀️ Each verb writes exactly ONE of the nine members. An after-snapshot comparison cannot make
/// this check on its own: an implementation that re-derived a sibling collection on every edit
/// would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-element applies to its committed before-snapshot");
    assert_eq!(snapshot.nodes, base.nodes, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but nodes moved");
    assert_eq!(snapshot.regions, base.regions, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but regions moved");
    assert_eq!(snapshot.materials, base.materials, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but materials moved");
    assert_eq!(snapshot.sections, base.sections, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but sections moved");
    assert_eq!(snapshot.supports, base.supports, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but supports moved");
    assert_eq!(snapshot.load_cases, base.load_cases, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but loadCases moved");
    assert_eq!(snapshot.combinations, base.combinations, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but combinations moved");
    assert_eq!(snapshot.analysis, base.analysis, "delete-element/cuts-the-lower-e4a250: this verb writes elements and nothing else, but analysis moved");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-element/cuts-the-lower-e4a250: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-element/cuts-the-lower-e4a250: committed mutation JSON is not canonical");
}

/// 🔺️ The delta must be exactly the committed one, on exactly the `elements` slot.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    let delta = outcome.diff().elements.as_ref().expect("elements delta");
    assert_eq!((delta.added.len(), delta.removed.len(), delta.patched.len()), (0, 1, 0), "delete-element/cuts-the-lower-e4a250: the delta must be exactly one removed entry");
    assert!(delta.reordered.is_none(), "delete-element/cuts-the-lower-e4a250: no verb in this vocabulary re-orders a collection");
    assert!(outcome.diff().nodes.is_none(), "delete-element/cuts-the-lower-e4a250: no nodes delta may be opened by this verb");
    assert!(outcome.diff().regions.is_none(), "delete-element/cuts-the-lower-e4a250: no regions delta may be opened by this verb");
    assert!(outcome.diff().materials.is_none(), "delete-element/cuts-the-lower-e4a250: no materials delta may be opened by this verb");
    assert!(outcome.diff().sections.is_none(), "delete-element/cuts-the-lower-e4a250: no sections delta may be opened by this verb");
    assert!(outcome.diff().supports.is_none(), "delete-element/cuts-the-lower-e4a250: no supports delta may be opened by this verb");
    assert!(outcome.diff().load_cases.is_none(), "delete-element/cuts-the-lower-e4a250: no loadCases delta may be opened by this verb");
    assert!(outcome.diff().combinations.is_none(), "delete-element/cuts-the-lower-e4a250: no combinations delta may be opened by this verb");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-element/cuts-the-lower-e4a250: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-element/cuts-the-lower-e4a250: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed delta on `before` must reproduce the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-element/cuts-the-lower-e4a250: committed diff did not carry before to after");
}

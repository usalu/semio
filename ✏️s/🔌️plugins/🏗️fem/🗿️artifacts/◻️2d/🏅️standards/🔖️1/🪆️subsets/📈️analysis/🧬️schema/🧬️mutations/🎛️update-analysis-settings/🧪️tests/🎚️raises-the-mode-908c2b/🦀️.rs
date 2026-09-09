//! 🧪️ `update-analysis-settings` fixture — `🎚️raises-the-mode-908c2b`.
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
//! 🎚️ Analysis settings are one inseparable facet: the verb writes the whole record, so the diff carries a value rather than a collection delta.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation, inverse_fem2d_mutation};
use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🎚️raises-the-mode-908c2b/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🎚️raises-the-mode-908c2b/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🎚️raises-the-mode-908c2b/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🎚️raises-the-mode-908c2b/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎛️update-analysis-settings/🎚️raises-the-mode-908c2b/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `update-analysis-settings` carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("update-analysis-settings applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "update-analysis-settings/raises-the-mode-908c2b: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, base, "update-analysis-settings/raises-the-mode-908c2b: the forward mutation left the model untouched, so nothing was proved");
    assert_eq!(snapshot.analysis.modal_count, 10, "update-analysis-settings/raises-the-mode-908c2b: the modal count must be the one the payload states");
    assert_eq!(snapshot.analysis.buckling_count, 6, "update-analysis-settings/raises-the-mode-908c2b: the buckling count must be the one the payload states");
    assert_eq!(snapshot.analysis.deformation_scale, 80.0, "update-analysis-settings/raises-the-mode-908c2b: the deformation scale must be the one the payload states");
}

/// ↩️ Applying the computed inverse after the forward step lands back on `before`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "update-analysis-settings/raises-the-mode-908c2b: update-analysis-settings undoes with exactly one step, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-analysis-settings/raises-the-mode-908c2b: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — applied, with no diagnostic at all — is what this kind really emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "update-analysis-settings/raises-the-mode-908c2b declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "update-analysis-settings/raises-the-mode-908c2b: a clean application raises no diagnostic, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("update-analysis-settings/raises-the-mode-908c2b: declared applied but the mutation was rejected");
}

/// 🔀️ Each verb writes exactly ONE of the nine members. An after-snapshot comparison cannot make
/// this check on its own: an implementation that re-derived a sibling collection on every edit
/// would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("update-analysis-settings applies to its committed before-snapshot");
    assert_eq!(snapshot.nodes, base.nodes, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but nodes moved");
    assert_eq!(snapshot.elements, base.elements, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but elements moved");
    assert_eq!(snapshot.regions, base.regions, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but regions moved");
    assert_eq!(snapshot.materials, base.materials, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but materials moved");
    assert_eq!(snapshot.sections, base.sections, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but sections moved");
    assert_eq!(snapshot.supports, base.supports, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but supports moved");
    assert_eq!(snapshot.load_cases, base.load_cases, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but loadCases moved");
    assert_eq!(snapshot.combinations, base.combinations, "update-analysis-settings/raises-the-mode-908c2b: this verb writes analysis and nothing else, but combinations moved");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-analysis-settings/raises-the-mode-908c2b: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-analysis-settings/raises-the-mode-908c2b: committed mutation JSON is not canonical");
}

/// 🔺️ The delta must be exactly the committed one, on exactly the `analysis` slot.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().analysis.is_some(), "update-analysis-settings/raises-the-mode-908c2b: the analysis facet is written as a whole value, never as a collection delta");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-analysis-settings/raises-the-mode-908c2b: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-analysis-settings/raises-the-mode-908c2b: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed delta on `before` must reproduce the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-analysis-settings/raises-the-mode-908c2b: committed diff did not carry before to after");
}

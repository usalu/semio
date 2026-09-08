//! 🧪️ `create-combination` fixture — `➕️appends-the-6-10a-eefe01`.
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
//! ➕️ EN 1990 expression 6.10a over the dead and snow cases. Every term is validated against the load cases AND the existing combinations, so a nested combination is legal.

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

/// ▶️ `create-combination` carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("create-combination applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-combination/appends-the-6-10a-eefe01: applied state differs from committed after-snapshot");
    assert_ne!(snapshot, base, "create-combination/appends-the-6-10a-eefe01: the forward mutation left the model untouched, so nothing was proved");
    assert_eq!(snapshot.combinations.len(), 4, "create-combination/appends-the-6-10a-eefe01: the combinations collection must end up 4 records long");
    assert_eq!(snapshot.combinations.last().expect("the collection is not empty").id, "uls2", "create-combination/appends-the-6-10a-eefe01: the new record must be appended at the tail — no create- verb in this vocabulary carries an index");
}

/// ↩️ Applying the computed inverse after the forward step lands back on `before`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    assert_eq!(inverse.len(), 1, "create-combination/appends-the-6-10a-eefe01: create-combination undoes with exactly one step, got {inverse:?}");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "create-combination/appends-the-6-10a-eefe01: inverse did not restore the before-snapshot");
}

/// 🎯️ The declared outcome — applied, with no diagnostic at all — is what this kind really emits.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "create-combination/appends-the-6-10a-eefe01 declares an applied outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "create-combination/appends-the-6-10a-eefe01: a clean application raises no diagnostic, got {:?}", produced.messages());
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("create-combination/appends-the-6-10a-eefe01: declared applied but the mutation was rejected");
}

/// 🔀️ Each verb writes exactly ONE of the nine members. An after-snapshot comparison cannot make
/// this check on its own: an implementation that re-derived a sibling collection on every edit
/// would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("create-combination applies to its committed before-snapshot");
    assert_eq!(snapshot.nodes, base.nodes, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but nodes moved");
    assert_eq!(snapshot.elements, base.elements, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but elements moved");
    assert_eq!(snapshot.regions, base.regions, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but regions moved");
    assert_eq!(snapshot.materials, base.materials, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but materials moved");
    assert_eq!(snapshot.sections, base.sections, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but sections moved");
    assert_eq!(snapshot.supports, base.supports, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but supports moved");
    assert_eq!(snapshot.load_cases, base.load_cases, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but loadCases moved");
    assert_eq!(snapshot.analysis, base.analysis, "create-combination/appends-the-6-10a-eefe01: this verb writes combinations and nothing else, but analysis moved");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-combination/appends-the-6-10a-eefe01: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-combination/appends-the-6-10a-eefe01: committed mutation JSON is not canonical");
}

/// 🔺️ The delta must be exactly the committed one, on exactly the `combinations` slot.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    let delta = outcome.diff().combinations.as_ref().expect("combinations delta");
    assert_eq!((delta.added.len(), delta.removed.len(), delta.patched.len()), (1, 0, 0), "create-combination/appends-the-6-10a-eefe01: the delta must be exactly one added entry");
    assert!(delta.reordered.is_none(), "create-combination/appends-the-6-10a-eefe01: no verb in this vocabulary re-orders a collection");
    assert!(outcome.diff().nodes.is_none(), "create-combination/appends-the-6-10a-eefe01: no nodes delta may be opened by this verb");
    assert!(outcome.diff().elements.is_none(), "create-combination/appends-the-6-10a-eefe01: no elements delta may be opened by this verb");
    assert!(outcome.diff().regions.is_none(), "create-combination/appends-the-6-10a-eefe01: no regions delta may be opened by this verb");
    assert!(outcome.diff().materials.is_none(), "create-combination/appends-the-6-10a-eefe01: no materials delta may be opened by this verb");
    assert!(outcome.diff().sections.is_none(), "create-combination/appends-the-6-10a-eefe01: no sections delta may be opened by this verb");
    assert!(outcome.diff().supports.is_none(), "create-combination/appends-the-6-10a-eefe01: no supports delta may be opened by this verb");
    assert!(outcome.diff().load_cases.is_none(), "create-combination/appends-the-6-10a-eefe01: no loadCases delta may be opened by this verb");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-combination/appends-the-6-10a-eefe01: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-combination/appends-the-6-10a-eefe01: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed delta on `before` must reproduce the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-combination/appends-the-6-10a-eefe01: committed diff did not carry before to after");
}

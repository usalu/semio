//! 🧪️ `replace-section` fixture — `stiffens-ipe200-with-a-reinforced-profile`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Area and inertia both grow under the same section id, so every element already pointing at it is silently stiffened.

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

/// ▶️ `replace-section` restates `ipe200` as the reinforced profile and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("replace-section applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "replace-section/stiffens-ipe200-with-a-reinforced-profile: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.sections.len(), 1, "replace-section/stiffens-ipe200-with-a-reinforced-profile: a replacement must not change the profile count");
    assert_eq!(snapshot.sections[0].area, 0.0035, "replace-section/stiffens-ipe200-with-a-reinforced-profile: the enlarged area must survive the round trip exactly");
    assert_eq!(snapshot.elements, before().elements, "replace-section/stiffens-ipe200-with-a-reinforced-profile: e1 keeps the same section id and must not be rewritten");
}

/// ↩️ The inverse is a `replace-section` carrying the slender profile recovered from `base`.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-section/stiffens-ipe200-with-a-reinforced-profile: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "replace-section/stiffens-ipe200-with-a-reinforced-profile: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "replace-section/stiffens-ipe200-with-a-reinforced-profile: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(dsl::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "replace-section/stiffens-ipe200-with-a-reinforced-profile: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "replace-section/stiffens-ipe200-with-a-reinforced-profile: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "replace-section/stiffens-ipe200-with-a-reinforced-profile: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("replace-section/stiffens-ipe200-with-a-reinforced-profile: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `sections.patched` entry keyed by `ipe200`.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().sections.as_ref().expect("sections delta").patched.len(), 1, "replace-section/stiffens-ipe200-with-a-reinforced-profile: exactly one profile may be patched");
    assert!(outcome.diff().sections.as_ref().expect("sections delta").removed.is_empty(), "replace-section/stiffens-ipe200-with-a-reinforced-profile: a replacement is never a removal");
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "replace-section/stiffens-ipe200-with-a-reinforced-profile: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "replace-section/stiffens-ipe200-with-a-reinforced-profile: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `sections.patched` entry on `before` must stiffen the profile in place.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::artifacts::fem2d::diff::Fem2dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::artifacts::fem2d::diff::Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "replace-section/stiffens-ipe200-with-a-reinforced-profile: committed diff did not carry before to after");
}

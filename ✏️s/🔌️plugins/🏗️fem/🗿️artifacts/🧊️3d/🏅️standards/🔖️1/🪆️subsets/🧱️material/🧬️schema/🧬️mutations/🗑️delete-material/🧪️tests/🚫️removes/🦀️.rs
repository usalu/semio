//! 🧪️ `delete-material` fixture — `🚫️removes`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Only the trailing, unreferenced alloy is dropped; frame `f1` keeps its steel reference untouched.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation,inverse_fem3d_mutation};

use crate::Fem3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-material/🚫️removes/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-material/🚫️removes/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-material/🚫️removes/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-material/🚫️removes/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-material/🚫️removes/🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `delete-material` drops `alu` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("delete-material applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-material/removes-the-b7b56a: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.materials.len(), 1, "delete-material/removes-the-b7b56a: only steel may remain");
    assert_eq!(snapshot.materials[0].id, "steel", "delete-material/removes-the-b7b56a: the surviving row must be the one f1 references");
    assert_eq!(snapshot.elements, before().elements, "delete-material/removes-the-b7b56a: element material references are never rewritten by a material deletion");
}

/// ↩️ The inverse is a `create-material` rebuilt from `base`, re-appending the alloy with all five properties.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-material/removes-the-b7b56a: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-material/removes-the-b7b56a: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-material/removes-the-b7b56a: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
///
/// 🚦️ Refusal is read off the OUTCOME, never off the `Result`. `vcs::apply_mutation` is
/// policy-agnostic: a refused mutation carries the empty diff, so it still applies cleanly and
/// still returns `Ok` — asserting `is_err()` here would be a branch that can never fire.
#[test]
fn declared_outcome_holds() {
    let outcome: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    let status = outcome.get("status").and_then(semio_framework_value::DslValue::as_str).expect("outcome carries a status");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let refused = produced.messages().iter().any(|message| message.level >= semio_framework_diagnostic::Severity::Error);
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("the produced diff applies to its own before-snapshot");
    match status {
        "applied" => assert!(!refused, "delete-material/removes-the-b7b56a: declared applied but the diff builder refused with {:?}", produced.messages()),
        "rejected" => {
            assert!(refused, "delete-material/removes-the-b7b56a: declared rejected but the diff builder raised no Error or Fatal, only {:?}", produced.messages());
            assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "delete-material/removes-the-b7b56a: a refused mutation must carry the empty diff");
            assert_eq!(snapshot, before(), "delete-material/removes-the-b7b56a: a refused mutation must leave the snapshot untouched");
        }
        other => panic!("delete-material/removes-the-b7b56a: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `materials.removed` id — element references are not rewritten.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().materials.as_ref().expect("materials delta").removed, vec!["alu".to_string()], "delete-material/removes-the-b7b56a: exactly alu may be removed");
    assert!(outcome.diff().elements.is_none(), "delete-material/removes-the-b7b56a: no element delta may be opened");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-material/removes-the-b7b56a: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-material/removes-the-b7b56a: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `materials.removed` id on `before` must leave the steel-only catalogue.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-material/removes-the-b7b56a: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

//! 🧪️ `delete-load-case` fixture — `🚫️removes`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! Loads are members of their case, so removing the case removes them with it — no separate load delta appears.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem2d_mutation,inverse_fem2d_mutation};

use crate::Fem2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-load-case/🚫️removes/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-load-case/🚫️removes/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-load-case/🚫️removes/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-load-case/🚫️removes/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🗑️delete-load-case/🚫️removes/🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem2dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem2dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `delete-load-case` drops the live case and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("delete-load-case applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "delete-load-case/removes-the-live-06415d: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.load_cases.len(), 1, "delete-load-case/removes-the-live-06415d: only the dead case may remain");
    assert_eq!(snapshot.load_cases[0].id, "dead", "delete-load-case/removes-the-live-06415d: the dead case is the survivor");
    assert_eq!(snapshot.nodes, before().nodes, "delete-load-case/removes-the-live-06415d: the node the removed load pointed at must stay");
}

/// ↩️ The inverse is a `create-load-case` rebuilt from `base`, restoring the case *and* its nodal load.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem2d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_fem2d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "delete-load-case/removes-the-live-06415d: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem2dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-load-case/removes-the-live-06415d: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "delete-load-case/removes-the-live-06415d: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    let status = outcome.get("status").and_then(semio_framework_value::DslValue::as_str).expect("outcome carries a status");
    let mut snapshot = before();
    let applied = apply_fem2d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => assert!(applied, "delete-load-case/removes-the-live-06415d: declared applied but the mutation was rejected"),
        "rejected" => {
            assert!(!applied, "delete-load-case/removes-the-live-06415d: declared rejected but the mutation applied");
            assert_eq!(snapshot, before(), "delete-load-case/removes-the-live-06415d: rejected mutation must leave the snapshot untouched");
        }
        other => panic!("delete-load-case/removes-the-live-06415d: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `loadCases.removed` id; the member loads are implied, never itemised.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &base);
    assert_eq!(outcome.diff().load_cases.as_ref().expect("loadCases delta").removed, vec!["live".to_string()], "delete-load-case/removes-the-live-06415d: exactly the live case may be removed");
    assert!(outcome.diff().load_cases.as_ref().expect("loadCases delta").patched.is_empty(), "delete-load-case/removes-the-live-06415d: member loads are never itemised as patches");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-load-case/removes-the-live-06415d: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "delete-load-case/removes-the-live-06415d: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `loadCases.removed` id on `before` must leave only the dead case.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-load-case/removes-the-live-06415d: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

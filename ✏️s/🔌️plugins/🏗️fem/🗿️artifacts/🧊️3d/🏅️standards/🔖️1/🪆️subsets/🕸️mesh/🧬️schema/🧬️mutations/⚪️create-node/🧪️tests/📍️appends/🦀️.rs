//! 🧪️ `create-node` fixture — `📍️appends`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! A column head is coined 3.5 m above the origin — the third coordinate is what separates this from the fem2d node.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation,inverse_fem3d_mutation};

use crate::Fem3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚪️create-node/📍️appends/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚪️create-node/📍️appends/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚪️create-node/📍️appends/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚪️create-node/📍️appends/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/⚪️create-node/📍️appends/🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ `create-node` appends the elevated node `n3` and carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("create-node applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "create-node/appends-the-column-head-node-n3: applied state differs from committed after-snapshot");
    assert_eq!(snapshot.nodes.len(), 3, "create-node/appends-the-column-head-node-n3: the created node must be appended, never merged into an existing one");
    assert_eq!(snapshot.nodes[2].z, 3.5, "create-node/appends-the-column-head-node-n3: the elevation must survive the round trip exactly");
    assert_eq!(snapshot.nodes[1], before().nodes[1], "create-node/appends-the-column-head-node-n3: n2 must stay byte-identical");
}

/// ↩️ The inverse is a `delete-node` of `n3`, restoring the two-node ground line.
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
    assert_eq!(snapshot, base, "create-node/appends-the-column-head-node-n3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Fem3dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = semio_framework_value::ToValue::to_value(&decoded);
        let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot reparses");
        assert_eq!(reencoded, original, "create-node/appends-the-column-head-node-n3: committed {label} JSON is not canonical");
    }
    let decoded_mutation = mutation();
    let reencoded = semio_framework_value::ToValue::to_value(&decoded_mutation);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation reparses");
    assert_eq!(reencoded, original, "create-node/appends-the-column-head-node-n3: committed mutation JSON is not canonical");
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
        "applied" => assert!(!refused, "create-node/appends-the-column-head-node-n3: declared applied but the diff builder refused with {:?}", produced.messages()),
        "rejected" => {
            assert!(refused, "create-node/appends-the-column-head-node-n3: declared rejected but the diff builder raised no Error or Fatal, only {:?}", produced.messages());
            assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "create-node/appends-the-column-head-node-n3: a refused mutation must carry the empty diff");
            assert_eq!(snapshot, before(), "create-node/appends-the-column-head-node-n3: a refused mutation must leave the snapshot untouched");
        }
        other => panic!("create-node/appends-the-column-head-node-n3: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The delta must be a single `nodes.added` entry — a created node never patches or removes.
#[test]
fn produces_committed_diff() {
    let base = before();
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &base);
    assert!(outcome.diff().nodes.is_some(), "create-node/appends-the-column-head-node-n3: the created node must surface in the nodes delta");
    assert!(outcome.diff().solids.is_none() && outcome.diff().supports.is_none(), "create-node/appends-the-column-head-node-n3: creating a node must open no other collection delta");
    let produced = semio_framework_value::ToValue::to_value(outcome.diff());
    let committed: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "create-node/appends-the-column-head-node-n3: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_value::ToValue::to_value(&decoded);
    let original: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "create-node/appends-the-column-head-node-n3: committed diff JSON is not canonical");
}

/// 🩹 Replaying the committed `nodes.added` entry on `before` must reproduce the three-node frame.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "create-node/appends-the-column-head-node-n3: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

//! 🧪️ `delete-user-profile` fixture — `🗑️deletes`.
//!
//! Hand-authored source of truth is the JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). Every expectation below is transcribed from THIS
//! leaf's own `🔺️diff/🦀️.rs`, which removes the middle row of a three-row collection (`removed = [id]`); the inverse recreates it at its original index, so the summed inverse diffs carry the row order too.
//!
//! That leaf's own contract line reads: 🗑️ Error `mutation.target-missing` if the id is absent (empty diff), else `removed = [id]`.
//!
//! The `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio`/`.patch.semio` encodings are derived
//! from this JSON by `fixtures generate` and are asserted by the shared codec-matrix harness.

use crate::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::Mutation;

const BEFORE: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🧑️user/🗑️delete/🗑️deletes-middle/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🧑️user/🗑️delete/🗑️deletes-middle/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🧑️user/🗑️delete/🗑️deletes-middle/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🧑️user/🗑️delete/🗑️deletes-middle/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../../🧫️fixtures/🧬️mutations/🧑️user/🗑️delete/🗑️deletes-middle/🎯️outcome/🔣️.json");

fn before() -> ProgramSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-user-profile/deletes-user-profile-a: before snapshot decodes")
}

fn expected_after() -> ProgramSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-user-profile/deletes-user-profile-a: after snapshot decodes")
}

fn mutation() -> ProgramMutation {
    serde_json::from_str(MUTATION).expect("delete-user-profile/deletes-user-profile-a: mutation decodes")
}

/// ▶️ delete-user-profile carries the committed before-snapshot to exactly the committed after-snapshot.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_applies_to_committed_after() {
    let base = before();
    let outcome = mutation().diff(&base);
    let applied = protocol::apply_diff(outcome.diff(), &base).expect("delete-user-profile/deletes-user-profile-a: delete-user-profile applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "delete-user-profile/deletes-user-profile-a: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying delete-user-profile and then its own recorded inverse restores the before-snapshot exactly.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_inverse_restores_before() {
    let base = before();
    let forward = mutation();
    let mut undo = forward.inverse(&base).expect("valid retained mutation inverse fixture");
    undo.reverse();
    let mut state = protocol::apply_diff(forward.diff(&base).diff(), &base).expect("delete-user-profile/deletes-user-profile-a: forward diff applies");
    for step in &undo {
        state = protocol::apply_diff(step.diff(&state).diff(), &state).expect("delete-user-profile/deletes-user-profile-a: inverse step applies");
    }
    assert_eq!(state, base, "delete-user-profile/deletes-user-profile-a: create-user-profile (this leaf's recorded inverse) did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed delete-user-profile payload are canonical: decode then encode
/// is a fixed point.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: ProgramSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-user-profile/deletes-user-profile-a: snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("delete-user-profile/deletes-user-profile-a: snapshot re-encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("delete-user-profile/deletes-user-profile-a: snapshot reparses");
        assert_eq!(reencoded, original, "delete-user-profile/deletes-user-profile-a: committed {side} snapshot JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("delete-user-profile/deletes-user-profile-a: delete-user-profile payload re-encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-user-profile/deletes-user-profile-a: delete-user-profile payload reparses");
    assert_eq!(reencoded, original, "delete-user-profile/deletes-user-profile-a: committed delete-user-profile payload JSON is not canonical");
}

/// 🎯️ The declared outcome holds: delete-user-profile applies cleanly here and raises no diagnostic at all.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_declared_outcome_holds() {
    let declared: serde_json::Value = serde_json::from_str(OUTCOME).expect("delete-user-profile/deletes-user-profile-a: outcome decodes");
    assert_eq!(declared.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-user-profile/deletes-user-profile-a: this fixture declares an applied outcome");
    let base = before();
    let outcome = mutation().diff(&base);
    assert!(outcome.messages().is_empty(), "delete-user-profile/deletes-user-profile-a: delete-user-profile raised a diagnostic on a fixture that declares a clean apply");
    assert!(protocol::apply_diff(outcome.diff(), &base).is_ok(), "delete-user-profile/deletes-user-profile-a: delete-user-profile was rejected by apply on its own before-snapshot");
}

/// 🔺️ The sparse delta delete-user-profile produces is exactly the committed diff — this pins WHICH collection
/// and which fields the mutation is allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("delete-user-profile/deletes-user-profile-a: produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("delete-user-profile/deletes-user-profile-a: committed diff decodes");
    assert_eq!(produced, committed, "delete-user-profile/deletes-user-profile-a: the diff delete-user-profile builds differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to ProgramDiff.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_committed_diff_is_canonical() {
    let decoded: ProgramDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-user-profile/deletes-user-profile-a: committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("delete-user-profile/deletes-user-profile-a: committed diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("delete-user-profile/deletes-user-profile-a: committed diff reparses");
    assert_eq!(reencoded, original, "delete-user-profile/deletes-user-profile-a: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff straight to the before-snapshot yields the committed
/// after-snapshot — the diff is a complete description of what delete-user-profile does, not a summary.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_committed_diff_applies_to_after() {
    let decoded: ProgramDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("delete-user-profile/deletes-user-profile-a: committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("delete-user-profile/deletes-user-profile-a: committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-user-profile/deletes-user-profile-a: the committed diff did not carry before to after");
}

/// 🧮️ Law L3: the diffs of delete-user-profile's inverse mutations, summed with `absorb`, equal the negative of its forward diff and carry the committed after-snapshot back to the before-snapshot.
#[semio_framework_async_macros::async_test]
async fn delete_user_profile_inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

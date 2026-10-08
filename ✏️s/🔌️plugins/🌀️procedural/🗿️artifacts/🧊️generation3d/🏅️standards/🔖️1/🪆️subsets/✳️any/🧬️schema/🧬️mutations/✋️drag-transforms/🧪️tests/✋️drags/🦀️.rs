//! 🧪️ `drag-transforms` fixture — `✋️drags`.
//!
//! `drag-transforms` adds the gesture offset to the offset the addressed translate operator holds, read from its BASE params.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), authored from the independent Python oracle
//! (`🧪️tests/🧊️mutate-procedural-3d-1/🐍️.py`), never from this implementation.

use crate::standards::v1::subsets::any::schema::diff::{Generation3dDiff, Generation3dDiffRead};
use crate::standards::v1::subsets::any::schema::mutations::{inverse_generation3d_mutation, Generation3dMutation};

use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
use crate::Generation3dSnapshot;
use crate::central_apply::{apply_generation3d_mutation};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/✋️drags/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/✋️drags/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/✋️drags/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/✋️drags/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-transforms/✋️drags/🎯️outcome/🔣️.json");

fn before() -> Generation3dSnapshotRead {
    Generation3dSnapshotRead::new(semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes"))
}
fn expected_after() -> Generation3dSnapshotRead {
    Generation3dSnapshotRead::new(semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes"))
}
fn mutation() -> Generation3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}
fn raised_diff(base: &Generation3dSnapshot) -> (Generation3dDiffRead, Vec<protocol::MutationMessage>) {
    let (diff, messages) = <Generation3dMutation as protocol::Mutation<Generation3dSnapshot>>::diff(&mutation(), base).into_parts();
    (Generation3dDiffRead::new(diff), messages)
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    apply_generation3d_mutation(&mut snapshot, &mutation()).expect("drag-transforms applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_generation3d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = Generation3dSnapshotRead::new((*base).clone());
    apply_generation3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {
        apply_generation3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = Generation3dSnapshotRead::new(semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes"));
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&*decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let base = before();
    let (_delta, messages) = raised_diff(&base);
    let produced: Vec<(String, String)> = messages
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = Generation3dSnapshotRead::new((*base).clone());
    let applied = apply_generation3d_mutation(&mut snapshot, &mutation()).is_ok();
    match status {
        "applied" => {
            assert!(applied, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: declared applied but the mutation was rejected");
            assert_ne!(snapshot, base, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, base, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("drag-transforms/drags-the-translate-operator-by-the-gesture-offset: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH collections and fields `drag-transforms` is
/// allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let (delta, _messages) = raised_diff(&base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&*delta)).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded = Generation3dDiffRead::new(semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes"));
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&*decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `drag-transforms` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded = Generation3dDiffRead::new(semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes"));
    let produced = Generation3dSnapshotRead::new(
        protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot"),
    );
    assert_eq!(produced, expected_after(), "drag-transforms/drags-the-translate-operator-by-the-gesture-offset: committed diff did not carry before to after");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff and carry the after-state back to `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

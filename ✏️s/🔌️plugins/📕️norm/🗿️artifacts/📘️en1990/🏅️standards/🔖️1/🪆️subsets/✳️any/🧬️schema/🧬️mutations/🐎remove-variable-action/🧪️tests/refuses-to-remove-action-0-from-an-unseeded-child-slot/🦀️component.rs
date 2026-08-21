//! 🧪️ `remove-variable-action` fixture — `refuses-to-remove-action-0-from-an-unseeded-child-slot`.
//!
//! `remove-variable-action` addresses the `q_k` table BY INDEX, and `q_k` is a composed `s.stdio.semio.table` child slot whose live entries live only in the session-side `EN1990_QK_SCRATCH` working-scene cache. A snapshot decoded from committed JSON can never have seeded that cache, so `en1990_qk` fails soft to an EMPTY list and `0 >= 0` trips the `mutation.target-missing` guard. That is not an accident of this fixture — it is the documented cache-miss behaviour of the composed-child design, and it is exactly what this case pins.
//!
//! Source of truth is the committed JSON quartet beside this file plus contract D6's empty
//! `🔺️diff/🚫️component.absent` marker (ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The
//! `.op.semio`/`.spr.semio`/`.dsl.semio`/`.pack.semio` encodings are derived from it by
//! `fixtures generate` and are asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF_ABSENT: &str = include_str!("🔺️diff/🚫️component.absent");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> En1990Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> En1990Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> En1990Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn applied() -> En1990Snapshot {
    let base = before();
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation(), &base);
    <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &base).expect("an empty rejection diff still applies cleanly")
}

/// ▶️ The rejected mutation carries `before` to exactly the committed `after` — which, for a
/// rejection, is `before` verbatim.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.q_k.child_id, before().q_k.child_id, "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: a refused removal must not re-mint the q_k handle");
    assert!(
        crate::artifacts::en1990::en1990_qk(&before()).is_empty(),
        "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: the unseeded working-scene cache must read back an empty entry list — the reason index 0 is missing"
    );
    assert!(<En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation(), &before()).is_empty(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: removing an absent index has nothing to undo");
    assert_eq!(snapshot, expected_after(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: applied state differs from committed after-snapshot");
    assert_eq!(expected_after(), before(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: a rejected case's after-snapshot must be its before-snapshot verbatim");
}

/// ↩️ A rejection changes nothing, and replaying its inverse on top still lands on `before`.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let mut snapshot = applied();
    for step in &<En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation, &base) {
        let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(step, &snapshot);
        snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: replaying the inverse of a rejection must still leave the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1990Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — the `rejected` status AND the exact diagnostic `remove-variable-action`'s own
/// diff builder raises — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    assert_eq!(status, "rejected", "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: this case exists to pin a rejection");
    let code = outcome.get("code").and_then(serde_json::Value::as_str).expect("a rejected outcome carries a machine-readable code");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: raised diagnostics differ from the committed 🎯️outcome messages");
    assert!(produced.iter().any(|(_, raised_code)| raised_code == code), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: the outcome's declared code must be one the builder actually raised");
    assert_eq!(applied(), before(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: a rejected mutation must leave the snapshot untouched");
}

/// 🔺️ A rejection publishes NO delta: the raised diff must be the diff type's own `default()`, and
/// the case must carry contract D6's empty `🚫️component.absent` marker instead of an invented
/// empty patch.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation(), &before());
    assert_eq!(raised.diff(), &En1990Diff::default(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: a fatal rejection must publish the default (all-null) diff");
    assert!(DIFF_ABSENT.is_empty(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: 🔺️diff/🚫️component.absent must be a zero-byte marker");
}

/// 🔣️ There is no committed diff JSON to be canonical — the absent marker is the committed form,
/// and it must stay empty.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    assert_eq!(DIFF_ABSENT.len(), 0, "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: the absence marker must carry no bytes at all");
    let produced = serde_json::to_value(En1990Diff::default()).expect("default diff encodes");
    assert!(
        produced.as_object().is_some_and(|fields| fields.values().all(serde_json::Value::is_null)),
        "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: the default diff must serialize as an all-null object, never as an omitted-field one"
    );
}

/// 🩹 Applying the rejection's own (default) diff to `before` yields the committed `after`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let produced = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&En1990Diff::default(), &before()).expect("the default diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "remove-variable-action/refuses-to-remove-action-0-from-an-unseeded-child-slot: the rejection's empty diff must leave before exactly as committed in after");
}

//! 🧪️ `insert-variable-action` fixture — `seeds-the-first-variable-action-q-snow-at-12-5-kn`.
//!
//! The one index-addressed variable-action mutation that succeeds against a freshly decoded snapshot: `en1990_qk` reads an EMPTY list out of the unseeded working-scene cache, index 0 clamps to 0 (so no `mutation.clamped` warning fires), and the resulting one-entry list is re-minted as a CONTENT-ADDRESSED child handle — `en1990-qk-69c0017661d2372c` is `DefaultHasher` over the JSON text of a one-entry list whose category is Q_snow and whose value is 12.5. The diff therefore touches the `q_k` child slot and nothing else; the twelve-and-a-half kilonewtons themselves never appear in the diff, only their address does.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

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
    <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &base).expect("insert-variable-action applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.q_k.child_id, "en1990-qk-69c0017661d2372c", "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the q_k handle must be the content address of the one-entry list");
    assert_ne!(snapshot.q_k.child_id, before().q_k.child_id, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: inserting must re-mint the handle, never reuse the empty-list address");
    assert_eq!(snapshot.q_k.target, before().q_k.target, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: only the content address moves — the child slot still targets the same table artifact");
    assert_eq!(crate::artifacts::en1990::en1990_qk(&snapshot).len(), 1, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the working-scene cache seeded by the diff builder must read back exactly one entry");
    assert_eq!(snapshot, expected_after(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(step, &snapshot);
        snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1990Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `insert-variable-action`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
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
    assert_eq!(produced, declared, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: a rejected mutation must leave the snapshot untouched"),
        other => panic!("insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `insert-variable-action` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert!(raised_diff.q_k.is_some(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the diff must publish the q_k child slot");
    assert_eq!(raised_diff.q_k.as_ref().map(|child| child.child_id.as_str()), Some("en1990-qk-69c0017661d2372c"), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: the published handle must be the one-entry content address");
    assert!(raised_diff.g_k.is_none() && raised_diff.resistance_kn.is_none(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: adding a variable action must not restate the permanent action or the resistance");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `En1990Diff` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: En1990Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `insert-variable-action` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: En1990Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-variable-action/seeds-the-first-variable-action-q-snow-at-12-5-kn: committed diff did not carry before to after");
}

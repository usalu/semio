//! 🧪️ `change-seismic-action` fixture — `enables-the-seismic-situation-with-an-85-kn-a-ed`.
//!
//! The seismic accidental action A_Ed goes 0 → 85 kN. Zero is this field's documented `seismic situation disabled` sentinel, so this case is what turns Eq. 6.12b on; it also pins that a 0.0 -> 85.0 move clears the no-op equality guard.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
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
    <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &base).expect("change-seismic-action applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.seismic_a_ed_kn, 85.0, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: A_Ed must be 85 kN");
    assert_eq!(before().seismic_a_ed_kn, 0.0, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: the committed before-snapshot must start with the seismic situation disabled");
    assert_eq!(snapshot, expected_after(), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(step, &snapshot);
        snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1990Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `change-seismic-action`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: a rejected mutation must leave the snapshot untouched"),
        other => panic!("change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `change-seismic-action` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert_eq!(raised_diff.seismic_a_ed_kn, Some(85.0), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: the diff must publish seismicAEdKn = 85");
    assert!(raised_diff.consequence_class.is_none(), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: the consequence class must stay null");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `En1990Diff` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: En1990Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-seismic-action` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1990Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-seismic-action/enables-the-seismic-situation-with-an-85-kn-a-ed: committed diff did not carry before to after");
}

//! 🧪️ `change-consequence-class` fixture — `escalates-the-building-from-cc2-to-cc3`.
//!
//! The consequence class goes CC2 → CC3, raising K_FI. This builder is the only one in EN 1990 with a RANGE invariant rather than a finiteness one — `!(1..=3).contains(&new)` is `mutation.invariant` (fatal); 3 is inside the range, so the change is published.
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
    <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &base).expect("change-consequence-class applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.consequence_class, 3, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the consequence class must be CC3");
    assert_eq!(before().consequence_class, 2, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the committed before-snapshot must start at CC2");
    assert_eq!(snapshot, expected_after(), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(step, &snapshot);
        snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: En1990Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `change-consequence-class`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
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
    assert_eq!(produced, declared, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: a rejected mutation must leave the snapshot untouched"),
        other => panic!("change-consequence-class/escalates-the-building-from-cc2-to-cc3: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `change-consequence-class` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let raised = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert_eq!(raised_diff.consequence_class, Some(3u8), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the diff must publish consequenceClass as the u8 3");
    assert!(raised_diff.annex.is_none(), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: the national annex choice must stay null");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: produced diff differs from the committed 🔺️diff/🔣️component.json");
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
    assert_eq!(reencoded, original, "change-consequence-class/escalates-the-building-from-cc2-to-cc3: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-consequence-class` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: En1990Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-consequence-class/escalates-the-building-from-cc2-to-cc3: committed diff did not carry before to after");
}

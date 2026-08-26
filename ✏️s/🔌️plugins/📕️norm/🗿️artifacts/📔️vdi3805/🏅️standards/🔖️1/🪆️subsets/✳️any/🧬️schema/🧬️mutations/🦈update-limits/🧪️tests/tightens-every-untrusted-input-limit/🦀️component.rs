//! 🧪️ `update-limits` fixture — `tightens-every-untrusted-input-limit`.
//!
//! All four untrusted-input security limits are one policy and move together: 16 MiB→1 MiB, 100000→4096 records, 8192→512 field bytes, depth 32→8.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Vdi3805Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Vdi3805Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Vdi3805Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}
fn applied() -> Vdi3805Snapshot {
    let base = before();
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &base);
    <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(raised.diff(), &base).expect("update-limits applies to its committed before-snapshot")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let snapshot = applied();
    assert_eq!(snapshot.limits.max_file_bytes, 1048576, "update-limits/tightens-every-untrusted-input-limit: max_file_bytes must be 1 MiB");
    assert_eq!(snapshot.limits.max_records, 4096, "update-limits/tightens-every-untrusted-input-limit: max_records must be 4096");
    assert_eq!(snapshot.limits.max_field_length, 512, "update-limits/tightens-every-untrusted-input-limit: max_field_length must be 512");
    assert_eq!(snapshot.limits.max_nesting_depth, 8, "update-limits/tightens-every-untrusted-input-limit: max_nesting_depth must be 8");
    assert_eq!(snapshot, expected_after(), "update-limits/tightens-every-untrusted-input-limit: applied state differs from committed after-snapshot");
}

/// ↩️ Applying the mutation then every step of its inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::inverse(&mutation, &base);
    assert!(!inverse.is_empty(), "update-limits/tightens-every-untrusted-input-limit: this mutation changes state, so its inverse must not be empty");
    let mut snapshot = applied();
    for step in &inverse {
        let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(step, &snapshot);
        snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(raised.diff(), &snapshot).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "update-limits/tightens-every-untrusted-input-limit: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is
/// a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Vdi3805Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "update-limits/tightens-every-untrusted-input-limit: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "update-limits/tightens-every-untrusted-input-limit: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `update-limits`'s own diff builder raises —
/// matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "update-limits/tightens-every-untrusted-input-limit: raised diagnostics differ from the committed 🎯️outcome messages");
    let snapshot = applied();
    match status {
        "applied" => assert_ne!(snapshot, before(), "update-limits/tightens-every-untrusted-input-limit: declared applied but the snapshot came back unchanged"),
        "rejected" => assert_eq!(snapshot, before(), "update-limits/tightens-every-untrusted-input-limit: a rejected mutation must leave the snapshot untouched"),
        other => panic!("update-limits/tightens-every-untrusted-input-limit: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields `update-limits` is allowed to
/// touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let raised = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation(), &before());
    let raised_diff = raised.diff();
    assert_eq!(raised_diff.limits.map(|limits| limits.max_nesting_depth), Some(8), "update-limits/tightens-every-untrusted-input-limit: the diff must publish the whole limits facet with depth 8");
    assert!(raised_diff.strict_mode.is_none(), "update-limits/tightens-every-untrusted-input-limit: changing limits must not also flip strict mode");
    let produced = serde_json::to_value(raised_diff).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "update-limits/tightens-every-untrusted-input-limit: produced diff differs from the committed 🔺️diff/🔣️component.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type. Note
/// `Vdi3805Diff` carries no `skip_serializing_if`, so every untouched field must be present as an
/// explicit `null` — and its `Option<Option<u32>>` presence field cannot distinguish "cleared" from
/// "untouched" across a JSON round trip, which is why no case here writes it.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Vdi3805Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "update-limits/tightens-every-untrusted-input-limit: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `update-limits` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Vdi3805Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "update-limits/tightens-every-untrusted-input-limit: committed diff did not carry before to after");
}

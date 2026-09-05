//! 🧪️ `insert-layer` fixture — `➕️inserts-an-interior-plaster-layer-at-index-1`.
//!
//! `insert-layer` rejects a non-positive or non-finite thickness/lambda with `mutation.invariant`, then clamps the index with `payload.index.min(layers.len())` and warns `mutation.clamped` only when the clamp actually moved it. Index 1 of a two-layer build-up needs no clamp, so this case applies with no diagnostics at all.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::din4108::diff::Din4108Diff;
use crate::artifacts::din4108::Din4108Mutation;
use crate::artifacts::din4108::Din4108Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Din4108Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Din4108Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Din4108Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `insert-layer` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("insert-layer applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "insert-layer/inserts-an-interior-plaster-layer-at-index-1: applied state differs from committed after-snapshot");
    let base = before();
    assert_eq!(snapshot.layers.len(), 3, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the build-up must grow from two layers to three");
    assert_eq!(snapshot.layers[1].thickness_m, 0.05, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the inserted plaster layer must sit at index 1 with thickness 0.05 m");
    assert_eq!(snapshot.layers[1].lambda_w_mk, 0.25, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the inserted plaster layer must carry lambda 0.25 W/mK");
    assert_eq!(snapshot.layers[0], base.layers[0], "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the masonry layer at index 0 must be untouched");
    assert_eq!(snapshot.layers[2], base.layers[1], "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the former index-1 insulation layer must have shifted to index 2 unchanged");
    assert_eq!(snapshot.category, base.category, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: insert-layer owns `layers` alone and must not touch `category`");
}

/// ↩️ Applying `insert-layer` and then its own inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the undo of one insert is exactly one remove-layer step");
    assert_eq!(snapshot.layers.len(), 2, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the undo must bring the build-up back to two layers");
    assert_eq!(snapshot.layers[1].thickness_m, 0.14, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the insulation layer must be back at index 1 with its original 0.14 m");
}

/// 🔣️ Both committed snapshots and this leaf's committed mutation payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din4108Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `insert-layer`'s own diff builder
/// raises for this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> =
        outcome.get("messages").and_then(serde_json::Value::as_array).map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect()).unwrap_or_default();
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::to_value(message.level).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: raised diagnostics differ from the committed 🎯️outcome messages");
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "insert-layer/inserts-an-interior-plaster-layer-at-index-1: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "insert-layer/inserts-an-interior-plaster-layer-at-index-1: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "insert-layer/inserts-an-interior-plaster-layer-at-index-1: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("insert-layer/inserts-an-interior-plaster-layer-at-index-1: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `insert-layer` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields of `Din4108Snapshot` this leaf is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: produced diff differs from the committed 🔺️diff/🔣️.json");
    let layers = raised.diff().layers.as_ref().expect("insert-layer writes the whole rebuilt layer list");
    assert_eq!(layers.values.len(), 3, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the sparse delta must carry the rebuilt three-layer build-up, not a single element");
    assert_eq!(layers.values[1].thickness_m, 0.05, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the delta's index-1 entry must be the inserted 0.05 m plaster layer");
    assert!(raised.diff().category.is_none(), "insert-layer/inserts-an-interior-plaster-layer-at-index-1: the sparse delta must leave `category` unset");
}

/// 🔣️ The committed diff is itself canonical and decodes to `Din4108Diff`. Its
/// `selectedCheckIndex` is an `Option<Option<u32>>` and so cannot distinguish `None` from
/// `Some(None)` across a JSON round trip — `insert-layer` never writes it, so the committed
/// `null` is unambiguously `None` here and the fixed point holds.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.selected_check_index.is_none(), "insert-layer/inserts-an-interior-plaster-layer-at-index-1: insert-layer is an artifact-lane edit and must never carry the presence-lane selectedCheckIndex");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "insert-layer/inserts-an-interior-plaster-layer-at-index-1: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `insert-layer` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "insert-layer/inserts-an-interior-plaster-layer-at-index-1: committed diff did not carry before to after");
}

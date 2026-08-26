//! 🧪️ `change-layer-thickness` fixture — `thickens-the-insulation-layer-to-0-point-2-m`.
//!
//! `change-layer-thickness` rejects a non-positive thickness with `mutation.invariant` targeted at the index, then a missing index with `mutation.target-missing`, then the `==` no-op — and only then rebuilds the list with that one layer's `thickness_m` replaced, leaving its lambda alone.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`), every value of which was transcribed from this
//! leaf's own `🔺️diff/🦀️component.rs` oracle. The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.

use crate::artifacts::din4108::diff::Din4108Diff;
use crate::artifacts::din4108::Din4108Mutation;
use crate::artifacts::din4108::Din4108Snapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> Din4108Snapshot {
    serde_json::from_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Din4108Snapshot {
    serde_json::from_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Din4108Mutation {
    serde_json::from_str(MUTATION).expect("mutation decodes")
}

/// ▶️ `change-layer-thickness` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("change-layer-thickness applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: applied state differs from committed after-snapshot");
    let base = before();
    assert_eq!(snapshot.layers[1].thickness_m, 0.2, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: layer #1's thickness must land on 0.2 m");
    assert_eq!(snapshot.layers[1].lambda_w_mk, base.layers[1].lambda_w_mk, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: the same layer's lambda must be left exactly as it was — thickness and lambda are separate leaves");
    assert_eq!(snapshot.layers[0], base.layers[0], "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: layer #0 must be untouched");
}

/// ↩️ Applying `change-layer-thickness` and then its own inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: the undo is exactly one change-layer-thickness step");
    assert_eq!(snapshot.layers[1].thickness_m, 0.14, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: the undo must put layer #1 back to 0.14 m");
}

/// 🔣️ Both committed snapshots and this leaf's committed mutation payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din4108Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `change-layer-thickness`'s own diff builder
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
    assert_eq!(produced, declared, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: raised diagnostics differ from the committed 🎯️outcome messages");
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `change-layer-thickness` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields of `Din4108Snapshot` this leaf is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: produced diff differs from the committed 🔺️diff/🔣️component.json");
    let layers = raised.diff().layers.as_ref().expect("change-layer-thickness writes the whole rebuilt layer list");
    assert_eq!(layers.values[1].thickness_m, 0.2, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: the delta's index-1 entry must carry the new 0.2 m thickness");
    assert_eq!(layers.values[1].lambda_w_mk, 0.04, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: the delta must re-state the untouched lambda 0.04 W/mK, because the whole list is the delta");
    assert!(raised.diff().catalog_id.is_none(), "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: the sparse delta must leave `catalog_id` unset");
}

/// 🔣️ The committed diff is itself canonical and decodes to `Din4108Diff`. Its
/// `selectedCheckIndex` is an `Option<Option<u32>>` and so cannot distinguish `None` from
/// `Some(None)` across a JSON round trip — `change-layer-thickness` never writes it, so the committed
/// `null` is unambiguously `None` here and the fixed point holds.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.selected_check_index.is_none(), "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: change-layer-thickness is an artifact-lane edit and must never carry the presence-lane selectedCheckIndex");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-layer-thickness` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-layer-thickness/thickens-the-insulation-layer-to-0-point-2-m: committed diff did not carry before to after");
}

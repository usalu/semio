//! 🧪️ `reorder-layers` fixture — `moves-the-insulation-in-front-of-the-masonry`.
//!
//! `reorder-layers` rejects an out-of-range `from` with `mutation.target-missing`, removes the layer, clamps `to` against the SHORTENED list, and warns `mutation.no-op` when the landing index equals `from`. Moving index 0 to index 1 of a two-layer build-up lands at 1 ≠ 0, so it applies cleanly.
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

/// ▶️ `reorder-layers` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("reorder-layers applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: applied state differs from committed after-snapshot");
    let base = before();
    assert_eq!(snapshot.layers.len(), base.layers.len(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: a reorder must never change how many layers there are");
    assert_eq!(snapshot.layers[0], base.layers[1], "reorder-layers/moves-the-insulation-in-front-of-the-masonry: the insulation layer must now be the outermost entry");
    assert_eq!(snapshot.layers[1], base.layers[0], "reorder-layers/moves-the-insulation-in-front-of-the-masonry: the masonry layer must have landed at index 1");
    assert_eq!(snapshot.envelope_area_m2, base.envelope_area_m2, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: reordering the build-up must not touch `envelope_area_m2`");
}

/// ↩️ Applying `reorder-layers` and then its own inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: the undo of one reorder is exactly one reorder back");
    assert_eq!(snapshot.layers[0].thickness_m, 0.24, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: the undo must return the 0.24 m masonry layer to index 0");
}

/// 🔣️ Both committed snapshots and this leaf's committed mutation payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din4108Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `reorder-layers`'s own diff builder
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
    assert_eq!(produced, declared, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: raised diagnostics differ from the committed 🎯️outcome messages");
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("reorder-layers/moves-the-insulation-in-front-of-the-masonry: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `reorder-layers` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields of `Din4108Snapshot` this leaf is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: produced diff differs from the committed 🔺️diff/🔣️.json");
    let layers = raised.diff().layers.as_ref().expect("reorder-layers writes the whole rebuilt layer list");
    assert_eq!(layers.values.len(), 2, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: the sparse delta must carry both layers in their new order");
    assert_eq!(layers.values[0].lambda_w_mk, 0.04, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: the delta's index-0 entry must be the insulation layer, lambda 0.04 W/mK");
    assert!(raised.diff().envelope_area_m2.is_none(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: the sparse delta must leave `envelope_area_m2` unset");
}

/// 🔣️ The committed diff is itself canonical and decodes to `Din4108Diff`. Its
/// `selectedCheckIndex` is an `Option<Option<u32>>` and so cannot distinguish `None` from
/// `Some(None)` across a JSON round trip — `reorder-layers` never writes it, so the committed
/// `null` is unambiguously `None` here and the fixed point holds.
#[semio_framework_async_macros::async_test]
fn committed_diff_is_canonical() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.selected_check_index.is_none(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: reorder-layers is an artifact-lane edit and must never carry the presence-lane selectedCheckIndex");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "reorder-layers/moves-the-insulation-in-front-of-the-masonry: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `reorder-layers` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
fn committed_diff_applies_to_after() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "reorder-layers/moves-the-insulation-in-front-of-the-masonry: committed diff did not carry before to after");
}

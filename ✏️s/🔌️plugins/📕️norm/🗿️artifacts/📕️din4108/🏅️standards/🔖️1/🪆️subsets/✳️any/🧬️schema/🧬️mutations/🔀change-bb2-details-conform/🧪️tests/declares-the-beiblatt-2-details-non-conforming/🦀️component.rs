//! 🧪️ `change-bb2-details-conform` fixture — `declares-the-beiblatt-2-details-non-conforming`.
//!
//! `change-bb2-details-conform` flips the DIN 4108 Beiblatt 2 conformity flag; the builder's only guard is the `==` no-op, so switching `true` to `false` is a plain single-field write.
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

/// ▶️ `change-bb2-details-conform` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let (snapshot, _) = protocol::apply_mutation(&before(), &mutation()).expect("change-bb2-details-conform applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: applied state differs from committed after-snapshot");
    assert!(!snapshot.bb2_details_conform, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: bb2DetailsConform did not land on false");
    assert_eq!(
        snapshot.airtightness_class,
        before().airtightness_class,
        "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: airtightnessClass must stay exactly as the before-snapshot had it — change-bb2-details-conform owns bb2DetailsConform and nothing else"
    );
}

/// ↩️ Applying `change-bb2-details-conform` and then its own inverse restores `before` exactly.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::inverse(&mutation, &base);
    let (mut snapshot, _) = protocol::apply_mutation(&base, &mutation).expect("forward applies");
    for step in &inverse {
        snapshot = protocol::apply_mutation(&snapshot, step).expect("inverse step applies").0;
    }
    assert_eq!(snapshot, base, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: inverse did not restore the before-snapshot");
    assert_eq!(inverse.len(), 1, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: undoing one bb2DetailsConform edit is exactly one step");
    assert!(snapshot.bb2_details_conform, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: the undo step must put bb2DetailsConform back to true");
}

/// 🔣️ Both committed snapshots and this leaf's committed mutation payload are already canonical:
/// decode→encode is a fixed point.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Din4108Snapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::to_value(mutation()).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic `change-bb2-details-conform`'s own diff builder
/// raises for this payload — matches what the mutation actually produces.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
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
    assert_eq!(produced, declared, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: raised diagnostics differ from the committed 🎯️outcome messages");
    let attempt = protocol::apply_mutation(&before(), &mutation());
    let applied = attempt.is_ok();
    let snapshot = attempt.map(|(next, _)| next).unwrap_or_else(|_| before());
    match status {
        "applied" if declared.iter().any(|(_, code)| code == "mutation.no-op") => {
            assert!(applied, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: declared applied but the mutation was rejected");
            assert_eq!(snapshot, before(), "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: a no-op outcome is applied with an EMPTY diff — the snapshot must come back untouched");
        }
        "applied" => {
            assert!(applied, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta `change-bb2-details-conform` produces is exactly the committed diff — the single most
/// load-bearing assertion in the fixture: it pins WHICH fields of `Din4108Snapshot` this leaf is
/// allowed to touch, not merely that the end state matches.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation(), &base);
    let produced = serde_json::to_value(raised.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: produced diff differs from the committed 🔺️diff/🔣️component.json");
    assert_eq!(raised.diff().bb2_details_conform, Some(false), "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: the sparse delta must carry bb2DetailsConform = false");
    assert!(
        raised.diff().airtightness_class.is_none(),
        "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: the sparse delta must leave airtightnessClass unset — a delta that rewrote it would be a bug this assertion exists to catch"
    );
}

/// 🔣️ The committed diff is itself canonical and decodes to `Din4108Diff`. Its
/// `selectedCheckIndex` is an `Option<Option<u32>>` and so cannot distinguish `None` from
/// `Some(None)` across a JSON round trip — `change-bb2-details-conform` never writes it, so the committed
/// `null` is unambiguously `None` here and the fixed point holds.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert!(decoded.selected_check_index.is_none(), "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: change-bb2-details-conform is an artifact-lane edit and must never carry the presence-lane selectedCheckIndex");
    let reencoded = serde_json::to_value(&decoded).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `change-bb2-details-conform` changed, not a summary of it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: Din4108Diff = serde_json::from_str(DIFF).expect("committed diff decodes");
    let produced = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-bb2-details-conform/declares-the-beiblatt-2-details-non-conforming: committed diff did not carry before to after");
}

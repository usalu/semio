//! 🧪️ `change-tile-media` fixture — `🎨️repaints`.
//!
//! Source of truth is the committed JSON quintet beside this file: the mutation is decoded from it,
//! applied, inverted, and its produced diff compared field for field against the committed delta.

use crate::diff::Wfc2dDiff;
use crate::mutations::{inverse_wfc2d_mutation, Wfc2dMutation};
use crate::schema::snapshot::Wfc2dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎨️change-tile-media/🎨️repaints/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎨️change-tile-media/🎨️repaints/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎨️change-tile-media/🎨️repaints/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎨️change-tile-media/🎨️repaints/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🎨️change-tile-media/🎨️repaints/🎯️outcome/🔣️.json");

fn before() -> Wfc2dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}
fn expected_after() -> Wfc2dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}
fn mutation() -> Wfc2dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    vcs::apply_mutation(&snapshot, &mutation()).map(|(applied_state, _)| { snapshot = applied_state; }).expect("mutation applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "change-tile-media/🎨️repaints: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — VALUE and POSITION.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_wfc2d_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    vcs::apply_mutation(&snapshot, &mutation).map(|(applied_state, _)| { snapshot = applied_state; }).expect("forward applies");
    for step in &inverse {
        vcs::apply_mutation(&snapshot, step).map(|(applied_state, _)| { snapshot = applied_state; }).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "change-tile-media/🎨️repaints: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots and the mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: Wfc2dSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "change-tile-media/🎨️repaints: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "change-tile-media/🎨️repaints: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic the diff builder raises — matches reality.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <Wfc2dMutation as protocol::Mutation<Wfc2dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "change-tile-media/🎨️repaints: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = vcs::apply_mutation(&snapshot, &mutation()).map(|(applied_state, _)| { snapshot = applied_state; }).is_ok();
    match status {
        "applied" => {
            assert!(applied, "change-tile-media/🎨️repaints: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "change-tile-media/🎨️repaints: declared applied but the snapshot came back unchanged");
        }
        "rejected" => {
            assert_eq!(snapshot, before(), "change-tile-media/🎨️repaints: a rejected mutation must leave the snapshot untouched");
        }
        other => panic!("change-tile-media/🎨️repaints: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — it pins WHICH
/// collections and fields the kind is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Wfc2dMutation as protocol::Mutation<Wfc2dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-tile-media/🎨️repaints: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to this artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Wfc2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-tile-media/🎨️repaints: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after`.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Wfc2dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "change-tile-media/🎨️repaints: committed diff did not carry before to after");
}

/// ➕️ The concrete inverse operations' diffs sum to exactly the negative of the forward diff (law L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

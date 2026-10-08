//! 🧪️ `pin-pixel` fixture — `📌️pins`.
//!
//! one id-keyed upsert into `pinned`, at the canonical row-major position rather than appended.
//!
//! Source of truth is the committed JSON quintet beside this file, itself emitted by the artifact
//! root's own `emit_committed_fixtures` generator — a hand-edited fixture is a bug by construction.

use crate::diff::BitmapDiff;
use crate::mutations::{inverse_bitmap_mutation, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️pin-pixel/📌️pins/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️pin-pixel/📌️pins/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️pin-pixel/📌️pins/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️pin-pixel/📌️pins/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📌️pin-pixel/📌️pins/🎯️outcome/🔣️.json");

fn before() -> BitmapSnapshot {
    crate::standards::v1::subsets::any::io::text::bitmap_json_decode(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> BitmapSnapshot {
    crate::standards::v1::subsets::any::io::text::bitmap_json_decode(AFTER).expect("after snapshot decodes")
}
fn mutation() -> BitmapMutation {
    crate::standards::v1::subsets::any::io::text::bitmap_json_decode(MUTATION).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    vcs::apply_mutation(&snapshot, &mutation()).map(|(applied_state, _)| { snapshot = applied_state; }).expect("pin-pixel applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "pin-pixel/📌️pins: applied state differs from the committed after-snapshot");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly — value AND position.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mutation = mutation();
    let inverse = inverse_bitmap_mutation(&base, &mutation).expect("valid retained mutation inverse fixture");
    let mut snapshot = base.clone();
    vcs::apply_mutation(&snapshot, &mutation).map(|(applied_state, _)| { snapshot = applied_state; }).expect("forward applies");
    for step in &inverse {
        vcs::apply_mutation(&snapshot, step).map(|(applied_state, _)| { snapshot = applied_state; }).expect("inverse step applies");
    }
    assert_eq!(snapshot, base, "pin-pixel/📌️pins: inverse did not restore the before-snapshot");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (side, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: BitmapSnapshot = crate::standards::v1::subsets::any::io::text::bitmap_json_decode(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::bitmap_json_encode(&decoded)).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "pin-pixel/📌️pins: committed {side} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::bitmap_json_encode(&mutation())).expect("mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "pin-pixel/📌️pins: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status AND every diagnostic this mutation's own diff builder raises —
/// matches what the mutation actually produces.
#[test]
fn declared_outcome_holds() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    let status = outcome.get("status").and_then(serde_json::Value::as_str).expect("outcome carries a status");
    let declared: Vec<(String, String)> = outcome
        .get("messages")
        .and_then(serde_json::Value::as_array)
        .map(|rows| rows.iter().map(|row| (row["level"].as_str().unwrap_or_default().to_string(), row["code"].as_str().unwrap_or_default().to_string())).collect())
        .unwrap_or_default();
    let raised = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::bitmap_json_encode(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "pin-pixel/📌️pins: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = vcs::apply_mutation(&snapshot, &mutation()).map(|(applied_state, _)| { snapshot = applied_state; }).is_ok();
    match status {
        "applied" => {
            assert!(applied, "pin-pixel/📌️pins: declared applied but the mutation was rejected");
            assert_ne!(snapshot, before(), "pin-pixel/📌️pins: declared applied but the snapshot came back unchanged");
        }
        "rejected" => assert_eq!(snapshot, before(), "pin-pixel/📌️pins: a rejected mutation must leave the snapshot untouched"),
        other => panic!("pin-pixel/📌️pins: unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the assertion that
/// pins WHICH lanes `pin-pixel` is allowed to touch, not merely that the end state matches.
#[test]
fn produces_committed_diff() {
    let raised = <BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation(), &before());
    let produced = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::bitmap_json_encode(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "pin-pixel/📌️pins: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: BitmapDiff = crate::standards::v1::subsets::any::io::text::bitmap_json_decode(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&crate::standards::v1::subsets::any::io::text::bitmap_json_encode(&decoded)).expect("diff re-encodes");
    let original: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "pin-pixel/📌️pins: committed diff JSON is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// complete description of what `pin-pixel` changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: BitmapDiff = crate::standards::v1::subsets::any::io::text::bitmap_json_decode(DIFF).expect("committed diff decodes");
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "pin-pixel/📌️pins: committed diff did not carry before to after");
}

/// ➕️ The concrete inverse operations' diffs sum to exactly the negative of the forward diff (law L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

//! 🧪️ `delete-slot` fixture — `🕳️removes`.
//!
//! `delete-slot` removes the slot AND every edge incident to it, in one atomic delta.
//!
//! Source of truth is the committed JSON quintet beside this file; the Rust builder that produced it
//! lives in the mutation aggregate's `fixture_cases()` table, so a case can never exist here and be
//! missing from the round-trip law.

use crate::diff::Wfc3dDiff;
use crate::mutations::Wfc3dMutation;
use crate::schema::snapshot::Wfc3dSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️delete-slot/🕳️removes/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️delete-slot/🕳️removes/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️delete-slot/🕳️removes/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️delete-slot/🕳️removes/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🕳️delete-slot/🕳️removes/🎯️outcome/🔣️.json");

fn before() -> Wfc3dSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("before snapshot decodes")
}

fn expected_after() -> Wfc3dSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("after snapshot decodes")
}

fn mutation() -> Wfc3dMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mutation decodes")
}

/// ▶️ The mutation carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    let mut snapshot = before();
    vcs::apply_mutation(&snapshot, &mutation()).map(|(applied_state, _)| { snapshot = applied_state; }).expect("the committed mutation applies");
    assert_eq!(snapshot, expected_after(), "🕳️removes: applying the mutation must reach the committed ➡️after");
}

/// ↩️ Applying the mutation then its inverse restores `before` exactly.
#[test]
fn inverse_restores_before() {
    let base = before();
    let mut snapshot = base.clone();
    vcs::apply_mutation(&snapshot, &mutation()).map(|(applied_state, _)| { snapshot = applied_state; }).expect("the committed mutation applies");
    for back in <Wfc3dMutation as protocol::Mutation<Wfc3dSnapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture") {
        vcs::apply_mutation(&snapshot, &back).map(|(applied_state, _)| { snapshot = applied_state; }).expect("the inverse applies");
    }
    assert_eq!(snapshot, base, "🕳️removes: inverse() must restore the pre-mutation document");
}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text, snapshot) in [("⬅️before", BEFORE, before()), ("➡️after", AFTER, expected_after())] {
        let committed: serde_json::Value = serde_json::from_str(text).expect("committed snapshot decodes");
        let printed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).expect("snapshot encodes");
        assert_eq!(printed, committed, "🕳️removes: committed {label} is not canonical");
    }
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
    let raised = <Wfc3dMutation as protocol::Mutation<Wfc3dSnapshot>>::diff(&mutation(), &before());
    let produced: Vec<(String, String)> = raised
        .messages()
        .iter()
        .map(|message| {
            let level = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&message.level)).expect("severity encodes");
            (level.as_str().unwrap_or_default().to_string(), message.code.0.clone())
        })
        .collect();
    assert_eq!(produced, declared, "🕳️removes: raised diagnostics differ from the committed 🎯️outcome messages");
    let mut snapshot = before();
    let applied = vcs::apply_mutation(&snapshot, &mutation()).map(|(applied_state, _)| { snapshot = applied_state; }).is_ok();
    match status {
        "applied" => {
            assert!(applied, "🕳️removes: an applied outcome must apply");
            assert_ne!(snapshot, before(), "🕳️removes: an applied outcome must change the document");
        }
        "rejected" => assert_eq!(snapshot, before(), "a rejected mutation must leave the snapshot untouched"),
        other => panic!("unknown outcome status {other:?}"),
    }
}

/// 🔺️ The sparse delta this mutation produces is exactly the committed diff — the load-bearing
/// assertion: it pins WHICH collections and fields this kind is allowed to touch, not merely that
/// the end state matches.
#[test]
fn produces_committed_diff() {
    let base = before();
    let raised = <Wfc3dMutation as protocol::Mutation<Wfc3dSnapshot>>::diff(&mutation(), &base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(raised.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "🕳️removes: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {
    let decoded: Wfc3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes to Wfc3dDiff");
    let printed: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&decoded)).expect("diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(printed, committed, "🕳️removes: the committed diff is not canonical");
}

/// 🩹 Applying the committed diff directly to `before` yields the committed `after` — the diff is a
/// COMPLETE description of what this mutation changed, not a summary of it.
#[test]
fn committed_diff_applies_to_after() {
    let decoded: Wfc3dDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let applied = protocol::apply_diff(&decoded, &before()).expect("the committed diff applies");
    assert_eq!(applied, expected_after(), "🕳️removes: the committed diff must be complete");
}

/// ➕️ The concrete inverse operations' diffs sum to exactly the negative of the forward diff (law L3).
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

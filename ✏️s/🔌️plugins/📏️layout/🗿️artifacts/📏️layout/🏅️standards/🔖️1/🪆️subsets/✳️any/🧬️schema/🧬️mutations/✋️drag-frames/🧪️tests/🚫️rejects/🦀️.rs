//! 🧪️ `drag-frames` fixture — `🚫️rejects`.
//!
//! No target is a frame of page 1: Error-level `mutation.target-missing`, nothing moves.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/` (contract D1), authored by
//! the independent implementation `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-layout-author-vectors.py`.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/🚫️rejects/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(BEFORE).expect("drag-frames/rejects-missing-frames: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(AFTER).expect("drag-frames/rejects-missing-frames: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    dsl::os_pack::from_json_str(MUTATION).expect("drag-frames/rejects-missing-frames: mutation decodes")
}
fn outcome() -> serde_json::Value {
    serde_json::from_str(OUTCOME).expect("drag-frames/rejects-missing-frames: outcome decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("drag-frames/rejects-missing-frames: the diff applies to its committed before-snapshot")
}

/// 🗣️ `(level, code, target)` of every message `drag-frames` raises on the committed base.
fn produced_messages() -> Vec<(protocol::Severity, String, Vec<String>)> {
    mutation().diff(&before()).messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(protocol::Severity, String, Vec<String>)> {
    let level = |text: &str| match text {
        "info" => protocol::Severity::Info,
        "warning" => protocol::Severity::Warning,
        "error" => protocol::Severity::Error,
        "fatal" => protocol::Severity::Fatal,
        other => panic!("drag-frames/rejects-missing-frames: unknown message level {other:?}"),
    };
    let strings = |value: &serde_json::Value| value.as_array().expect("an array of strings").iter().map(|entry| entry.as_str().expect("a string").to_string()).collect::<Vec<_>>();
    let outcome = outcome();
    if outcome["status"].as_str() == Some("rejected") {
        let code = outcome["code"].as_str().expect("a code");
        return vec![(protocol::outcome_code_level(code).expect("a vocabulary code"), code.to_string(), strings(&outcome["path"]))];
    }
    outcome.get("messages").and_then(serde_json::Value::as_array).map_or_else(Vec::new, |messages| {
        messages.iter().map(|message| (level(message["level"].as_str().expect("a level")), message["code"].as_str().expect("a code").to_string(), strings(&message["target"]))).collect()
    })
}

/// 🔣️ Both committed snapshots and the committed payload are canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: LayoutSnapshot = dsl::os_pack::from_json_str(text).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "drag-frames/rejects-missing-frames: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "drag-frames/rejects-missing-frames: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status, codes, levels and addressed targets — is exactly what `drag-frames`'s diff raises.
#[test]
fn declared_outcome_holds() {
    assert_eq!(produced_messages(), declared_messages(), "drag-frames/rejects-missing-frames: the produced messages differ from the declared outcome");
}

/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    assert_eq!(applied(), expected_after(), "drag-frames/rejects-missing-frames: applied state differs from the committed after-snapshot");
}

/// 🧾️ A vector that moves nothing leaves the committed `after` equal to `before` and has nothing to undo, and its D6 sentinel `🔺️diff/🚫️.absent` stays empty while its diff is the default one.
#[test]
fn moves_nothing_and_has_nothing_to_undo() {
    assert_eq!(expected_after(), before(), "drag-frames/rejects-missing-frames: a vector that moves nothing commits two equal snapshots");
    assert!(mutation().inverse(&before()).is_empty(), "drag-frames/rejects-missing-frames: nothing moved, so nothing is undone");
    assert!(DIFF_ABSENT.is_empty(), "drag-frames/rejects-missing-frames: the D6 sentinel must stay empty");
    assert_eq!(mutation().diff(&before()).diff(), &crate::LayoutDiff::default(), "drag-frames/rejects-missing-frames: a refusal carries the default diff");
}

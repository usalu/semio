//! 🧪️ `drag-frames` fixture — `⏸️keeps-a-zero-offset`.
//!
//! A zero offset moves nothing: Warning-level `mutation.no-op` and the default diff.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/` (contract D1), authored by
//! the independent implementation `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-layout-author-vectors.py`.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/✋️drag-frames/⏸️keeps-a-zero-offset/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("drag-frames/keeps-a-zero-offset: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("drag-frames/keeps-a-zero-offset: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    semio_framework_pack_json::from_json_str(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("drag-frames/keeps-a-zero-offset: mutation decodes")
}
fn outcome() -> serde_json::Value {
    serde_json::from_str(OUTCOME).expect("drag-frames/keeps-a-zero-offset: outcome decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("drag-frames/keeps-a-zero-offset: the diff applies to its committed before-snapshot")
}

/// 🗣️ `(level, code, target)` of every message `drag-frames` raises on the committed base.
fn produced_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    mutation().diff(&before()).messages().iter().map(|message| (message.level, message.code.0.clone(), message.target.clone())).collect()
}

/// 📜️ `(level, code, target)` of every message the committed outcome declares.
fn declared_messages() -> Vec<(semio_framework_diagnostic::Severity, String, Vec<String>)> {
    let level = |text: &str| match text {
        "info" => semio_framework_diagnostic::Severity::Info,
        "warning" => semio_framework_diagnostic::Severity::Warning,
        "error" => semio_framework_diagnostic::Severity::Error,
        "fatal" => semio_framework_diagnostic::Severity::Fatal,
        other => panic!("drag-frames/keeps-a-zero-offset: unknown message level {other:?}"),
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
        let decoded: LayoutSnapshot = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("snapshot decodes");
        let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("snapshot encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "drag-frames/keeps-a-zero-offset: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "drag-frames/keeps-a-zero-offset: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status, codes, levels and addressed targets — is exactly what `drag-frames`'s diff raises.
#[test]
fn declared_outcome_holds() {
    assert_eq!(produced_messages(), declared_messages(), "drag-frames/keeps-a-zero-offset: the produced messages differ from the declared outcome");
}

/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    assert_eq!(applied(), expected_after(), "drag-frames/keeps-a-zero-offset: applied state differs from the committed after-snapshot");
}

/// 🔺️ The sparse delta `drag-frames` produces is exactly the committed diff: WHICH frames of the page it patches, in page
/// order, and which bounds fields of each.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "drag-frames/keeps-a-zero-offset: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff decodes to `LayoutDiff`, re-encodes byte for byte, and carries `before` to `after` on its own.
#[test]
fn committed_diff_is_canonical_and_complete() {
    let decoded: crate::LayoutDiff = semio_framework_pack_json::from_json_str(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&decoded)).expect("committed diff re-encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff reparses"), "drag-frames/keeps-a-zero-offset: committed diff JSON is not canonical");
    assert_eq!(protocol::apply_diff(&decoded, &before()).expect("committed diff applies"), expected_after(), "drag-frames/keeps-a-zero-offset: committed diff did not carry before to after");
}

/// 🧾️ A vector that moves nothing leaves the committed `after` equal to `before` and has nothing to undo.
#[test]
fn moves_nothing_and_has_nothing_to_undo() {
    assert_eq!(expected_after(), before(), "drag-frames/keeps-a-zero-offset: a vector that moves nothing commits two equal snapshots");
    assert!(mutation().inverse(&before()).expect("valid retained mutation inverse fixture").is_empty(), "drag-frames/keeps-a-zero-offset: nothing moved, so nothing is undone");
}

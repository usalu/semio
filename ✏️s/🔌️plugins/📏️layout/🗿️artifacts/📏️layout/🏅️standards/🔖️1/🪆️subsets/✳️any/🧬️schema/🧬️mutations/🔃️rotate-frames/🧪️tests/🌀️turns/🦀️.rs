//! 🧪️ `rotate-frames` fixture — `🌀️turns`.
//!
//! A turn about the rect's own centre leaves its origin where it is and only grows its rotation.
//!
//! Source of truth is the committed JSON quintet under `🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/` (contract D1), authored by
//! the independent implementation `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-layout-author-vectors.py`.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔃️rotate-frames/🌀️turns/🎯️outcome/🔣️.json");

fn before() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(BEFORE).expect("rotate-frames/turns-the-rect-about-its-centre: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    dsl::os_pack::from_json_str(AFTER).expect("rotate-frames/turns-the-rect-about-its-centre: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    dsl::os_pack::from_json_str(MUTATION).expect("rotate-frames/turns-the-rect-about-its-centre: mutation decodes")
}
fn outcome() -> serde_json::Value {
    serde_json::from_str(OUTCOME).expect("rotate-frames/turns-the-rect-about-its-centre: outcome decodes")
}
fn applied() -> LayoutSnapshot {
    let base = before();
    mutation().diff(&base).diff().apply(&base).expect("rotate-frames/turns-the-rect-about-its-centre: the diff applies to its committed before-snapshot")
}

/// 🗣️ `(level, code, target)` of every message `rotate-frames` raises on the committed base.
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
        other => panic!("rotate-frames/turns-the-rect-about-its-centre: unknown message level {other:?}"),
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
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("snapshot reparses"), "rotate-frames/turns-the-rect-about-its-centre: committed {label} JSON is not canonical");
    }
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&mutation())).expect("mutation encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(MUTATION).expect("mutation reparses"), "rotate-frames/turns-the-rect-about-its-centre: committed mutation JSON is not canonical");
}

/// 🎯️ The declared outcome — status, codes, levels and addressed targets — is exactly what `rotate-frames`'s diff raises.
#[test]
fn declared_outcome_holds() {
    assert_eq!(produced_messages(), declared_messages(), "rotate-frames/turns-the-rect-about-its-centre: the produced messages differ from the declared outcome");
}

/// ▶️ The committed payload carries `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {
    assert_eq!(applied(), expected_after(), "rotate-frames/turns-the-rect-about-its-centre: applied state differs from the committed after-snapshot");
}

/// 🔺️ The sparse delta `rotate-frames` produces is exactly the committed diff: WHICH frames of the page it patches, in page
/// order, and which bounds fields of each.
#[test]
fn produces_committed_diff() {
    let produced = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(mutation().diff(&before()).diff())).expect("produced diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff decodes"), "rotate-frames/turns-the-rect-about-its-centre: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🩹 The committed diff decodes to `LayoutDiff`, re-encodes byte for byte, and carries `before` to `after` on its own.
#[test]
fn committed_diff_is_canonical_and_complete() {
    let decoded: crate::LayoutDiff = dsl::os_pack::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = serde_json::from_str::<serde_json::Value>(&dsl::os_pack::to_json_string(&decoded)).expect("committed diff re-encodes");
    assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed diff reparses"), "rotate-frames/turns-the-rect-about-its-centre: committed diff JSON is not canonical");
    assert_eq!(decoded.apply(&before()).expect("committed diff applies"), expected_after(), "rotate-frames/turns-the-rect-about-its-centre: committed diff did not carry before to after");
}

/// ↩️ Applying the payload then every step of the inverse it derives from `before` restores `before` EXACTLY — the
/// inverse is the absolute setters of the base bounds, never a negated parameter.
#[test]
fn inverse_restores_before() {
    let base = before();
    let inverse = mutation().inverse(&base);
    assert!(!inverse.is_empty(), "rotate-frames/turns-the-rect-about-its-centre: a moving vector must have something to undo");
    let mut snapshot = applied();
    assert_ne!(snapshot, base, "rotate-frames/turns-the-rect-about-its-centre: an applied vector must move a frame");
    for step in &inverse {
        snapshot = step.diff(&snapshot).diff().apply(&snapshot).expect("an inverse step applies");
    }
    assert_eq!(snapshot, base, "rotate-frames/turns-the-rect-about-its-centre: the inverse did not restore the before-snapshot");
}

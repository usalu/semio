//! 🧪️ `move-selection` fixture — `📌️pins`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 half-edge mesh in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-spatial-lowpoly-selection.py`.
//!
//! Two corners of the plane slide one unit along x; the other two stay pinned.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-selection/📌️pins/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-selection/📌️pins/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-selection/📌️pins/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-selection/📌️pins/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🚚️move-selection/📌️pins/🎯️outcome/🔣️.json");

/// ▶️ The motion carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}

/// ↩️ The computed inverse — the prior mesh handle and content written back as one edit — restores `before` exactly.
#[test]
fn inverse_restores_before() {
    laws::inverse_restores(BEFORE, MUTATION);
}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
#[test]
fn declared_outcome_holds() {
    laws::declared_outcome(BEFORE, MUTATION, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, Some(DIFF));
}

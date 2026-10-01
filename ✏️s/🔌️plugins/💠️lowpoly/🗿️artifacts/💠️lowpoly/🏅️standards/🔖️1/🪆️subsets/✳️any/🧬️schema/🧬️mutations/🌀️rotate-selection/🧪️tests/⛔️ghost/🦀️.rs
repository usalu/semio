//! 🧪️ `rotate-selection` fixture — `⛔️ghost`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 half-edge mesh in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-spatial-lowpoly-selection.py`.
//!
//! A turn of object obj-ghost, which the document does not hold, is refused.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀️rotate-selection/⛔️ghost/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀️rotate-selection/⛔️ghost/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀️rotate-selection/⛔️ghost/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀️rotate-selection/⛔️ghost/🎯️outcome/🔣️.json");

/// ⛔️ The refused or no-op motion leaves the document byte-identical and emits the declared diagnostic.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(BEFORE, MUTATION, AFTER, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, None);
}

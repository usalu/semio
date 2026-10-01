//! 🧪️ `apply-paint-stroke` fixture — `⛔️paints-a-missing-layer-27a048`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 brush in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-lowpoly-paint.py`.
//!
//! ⛔️ A stroke on paint layer 5 of obj-hull, whose stack holds two, addresses nothing and is refused.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints-a-missing-layer-27a048/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints-a-missing-layer-27a048/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints-a-missing-layer-27a048/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⛔️paints-a-missing-layer-27a048/🎯️outcome/🔣️.json");

/// ⛔️ The refused or no-op stroke leaves the document byte-identical and emits the declared diagnostic.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(BEFORE, MUTATION, AFTER, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, None);
}

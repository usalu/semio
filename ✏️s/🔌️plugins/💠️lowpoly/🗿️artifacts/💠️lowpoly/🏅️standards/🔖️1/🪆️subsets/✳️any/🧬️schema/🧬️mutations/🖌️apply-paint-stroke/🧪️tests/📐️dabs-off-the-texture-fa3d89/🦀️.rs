//! 🧪️ `apply-paint-stroke` fixture — `📐️dabs-off-the-texture-fa3d89`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 brush in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-lowpoly-paint.py`.
//!
//! 📐️ A dab at u = 1.5 lies off the texture and is refused as an invariant violation.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs-off-the-texture-fa3d89/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs-off-the-texture-fa3d89/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs-off-the-texture-fa3d89/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs-off-the-texture-fa3d89/🎯️outcome/🔣️.json");

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

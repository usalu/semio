//! 🧪️ `apply-paint-stroke` fixture — `⏸️erases`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 brush in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-lowpoly-paint.py`.
//!
//! ⏸️ An eraser dab over the already transparent second layer has no alpha left to remove: the stroke is a declared no-op.

use crate::laws;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/⬅️before/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/⬅️before/🔣️.json").leak());
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/➡️after/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/📸️snapshot/➡️after/🔣️.json").leak());
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/🦠️mutation/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/🦠️mutation/🔣️.json").leak());
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/🎯️outcome/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/⏸️erases/🎯️outcome/🔣️.json").leak());

/// ⛔️ The refused or no-op stroke leaves the document byte-identical and emits the declared diagnostic.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(*BEFORE, *MUTATION, *AFTER, *OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(*BEFORE, *AFTER, *MUTATION, None);
}

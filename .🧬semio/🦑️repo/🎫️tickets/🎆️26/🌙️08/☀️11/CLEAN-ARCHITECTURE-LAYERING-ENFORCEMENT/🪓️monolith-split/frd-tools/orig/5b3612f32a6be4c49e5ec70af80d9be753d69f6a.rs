//! 🧪️ `apply-paint-stroke` fixture — `📐️dabs`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 brush in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-lowpoly-paint.py`.
//!
//! 📐️ A dab at u = 1.5 lies off the texture and is refused as an invariant violation.

use crate::laws;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/⬅️before/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/⬅️before/🔣️.json").leak());
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/➡️after/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/📸️snapshot/➡️after/🔣️.json").leak());
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/🦠️mutation/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/🦠️mutation/🔣️.json").leak());
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/🎯️outcome/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/📐️dabs/🎯️outcome/🔣️.json").leak());

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

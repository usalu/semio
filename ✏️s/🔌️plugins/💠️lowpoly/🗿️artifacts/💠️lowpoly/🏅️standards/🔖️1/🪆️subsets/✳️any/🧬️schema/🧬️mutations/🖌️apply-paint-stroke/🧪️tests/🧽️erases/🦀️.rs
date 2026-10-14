//! 🧪️ `apply-paint-stroke` fixture — `🧽️erases`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 brush in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-lowpoly-paint.py`.
//!
//! 🧽️ One hard eraser dab at half opacity thins the alpha of the base layer's centre, leaving its colour untouched.

use crate::laws;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/⬅️before/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/⬅️before/🔣️.json").leak());
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/➡️after/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/📸️snapshot/➡️after/🔣️.json").leak());
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/🦠️mutation/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/🦠️mutation/🔣️.json").leak());
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/🔺️diff/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/🔺️diff/🔣️.json").leak());
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/🎯️outcome/🔣️.json")).expect("../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🧽️erases/🎯️outcome/🔣️.json").leak());

/// ▶️ The stroke carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(*BEFORE, *MUTATION, *AFTER, *DIFF);
}

/// ↩️ The computed inverse — the overwritten pixels written back as one edit — restores `before` exactly.
#[test]
fn inverse_restores_before() {
    laws::inverse_restores(*BEFORE, *MUTATION);
}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
#[test]
fn declared_outcome_holds() {
    laws::declared_outcome(*BEFORE, *MUTATION, *OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(*BEFORE, *AFTER, *MUTATION, Some(*DIFF));
}

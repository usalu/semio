//! 🧪️ `apply-paint-stroke` fixture — `🫓️paints`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 brush in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-lowpoly-paint.py`.
//!
//! 🫓️ A brush of radius 0 can reach no pixel and is refused as an invariant violation.

use crate::laws;
use semio_framework_test_fixtures::fixture_path;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/⬅️before/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/⬅️before/🔣️.json").leak());
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/➡️after/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/📸️snapshot/➡️after/🔣️.json").leak());
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/🦠️mutation/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/🦠️mutation/🔣️.json").leak());
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/🎯️outcome/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🖌️apply-paint-stroke/🫓️paints/🎯️outcome/🔣️.json").leak());

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

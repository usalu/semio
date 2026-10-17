//! 🧪️ `scale-selection` fixture — `🫓️flat`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 half-edge mesh in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-spatial-lowpoly-selection.py`.
//!
//! A zero factor would flatten the plane and is an invariant violation the schema already refuses.

use crate::laws;
use semio_framework_test_fixtures::fixture_path;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/📸️snapshot/⬅️before/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/📸️snapshot/⬅️before/🔣️.json").leak());
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/📸️snapshot/➡️after/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/📸️snapshot/➡️after/🔣️.json").leak());
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/🦠️mutation/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/🦠️mutation/🔣️.json").leak());
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/🎯️outcome/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🫓️flat/🎯️outcome/🔣️.json").leak());

/// ⛔️ The refused or no-op motion leaves the document byte-identical and emits the declared diagnostic.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(*BEFORE, *MUTATION, *AFTER, *OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(*BEFORE, *AFTER, *MUTATION, None);
}

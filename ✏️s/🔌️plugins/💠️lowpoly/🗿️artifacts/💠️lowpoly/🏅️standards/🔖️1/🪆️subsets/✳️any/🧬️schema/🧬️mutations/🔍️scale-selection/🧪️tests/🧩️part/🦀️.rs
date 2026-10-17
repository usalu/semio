//! 🧪️ `scale-selection` fixture — `🧩️part`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 half-edge mesh in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-spatial-lowpoly-selection.py`.
//!
//! Vertices 3 and the absent 7 halve along z: vertex 3 moves, vertex 7 is skipped.

use crate::laws;
use semio_framework_test_fixtures::fixture_path;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/📸️snapshot/⬅️before/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/📸️snapshot/⬅️before/🔣️.json").leak());
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/📸️snapshot/➡️after/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/📸️snapshot/➡️after/🔣️.json").leak());
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/🦠️mutation/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/🦠️mutation/🔣️.json").leak());
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/🔺️diff/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/🔺️diff/🔣️.json").leak());
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/🎯️outcome/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🔍️scale-selection/🧩️part/🎯️outcome/🔣️.json").leak());

/// ▶️ The motion carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(*BEFORE, *MUTATION, *AFTER, *DIFF);
}

/// ↩️ The computed inverse — the prior mesh handle and content written back as one edit — restores `before` exactly.
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

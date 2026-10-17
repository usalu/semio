//! 🧪️ `move-selection` fixture — `🚚️moves`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent float32 half-edge mesh in
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️s2-spatial-lowpoly-selection.py`.
//!
//! The whole plane slides by (0.5, 0, -0.25): every corner moves, the normals stay up.

use crate::laws;
use semio_framework_test_fixtures::fixture_path;

static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/📸️snapshot/⬅️before/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/📸️snapshot/⬅️before/🔣️.json").leak());
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/📸️snapshot/➡️after/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/📸️snapshot/➡️after/🔣️.json").leak());
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/🦠️mutation/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/🦠️mutation/🔣️.json").leak());
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/🔺️diff/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/🔺️diff/🔣️.json").leak());
static OUTCOME: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| std::fs::read_to_string(fixture_path!("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/🎯️outcome/🔣️.json")).expect("../../../../../🧪️tests/🧫️fixtures/🧬️mutations/🚚️move-selection/🚚️moves/🎯️outcome/🔣️.json").leak());

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

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, mutation) = laws::decode_case(*BEFORE, *MUTATION);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

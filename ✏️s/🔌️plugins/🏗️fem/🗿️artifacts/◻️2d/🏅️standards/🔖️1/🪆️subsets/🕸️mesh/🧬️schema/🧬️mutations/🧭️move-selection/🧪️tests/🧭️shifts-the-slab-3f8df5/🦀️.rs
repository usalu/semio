//! 🧪️ `move-selection` fixture — `🧭️shifts-the-slab-3f8df5`.
//!
//! Source of truth is the committed JSON quintet, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem2d-transform.py`.
//!
//! 🪵️ The model is the timber portal frame every fem2d mutation subset case shares (8.0 m span, eaves at 5.6 m,
//! ridge at 7.6 m, an RC first-floor slab on four corner nodes and one spare region), in SI base units.
//!
//! 🧭️ Shifting the first-floor slab 0.5 m right and 0.25 m up carries its outline and its four corner nodes together; every element on those nodes travels along.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🧭️shifts-the-slab-3f8df5/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🧭️shifts-the-slab-3f8df5/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🧭️shifts-the-slab-3f8df5/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🧭️shifts-the-slab-3f8df5/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🧭️shifts-the-slab-3f8df5/🎯️outcome/🔣️.json");

/// ▶️ The leaf carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}

/// ↩️ The computed inverse — whole-record replacements of every moved node and region — restores `before` exactly.
#[test]
fn inverse_restores_before() {
    laws::inverse_restores(BEFORE, MUTATION);
}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
#[test]
fn declared_outcome_holds() {
    laws::declared_outcome(BEFORE, MUTATION, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, Some(DIFF));
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    laws::inverse_sum(BEFORE, MUTATION).await;
}

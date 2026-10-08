//! 🧪️ `move-selection` fixture — `📏️stretches`.
//!
//! Source of truth is the committed JSON quintet, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem2d-transform.py`.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening), the
//! SECOND real-world fem2d model — the first is the timber portal frame the subset-level
//! differential cases share. Every value is in SI base units.
//!
//! 📏️ Stretching to twice the width about the right column line doubles the spare side panel and the canopy cantilever; the node the frame never had is skipped with a partial warning.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/📏️stretches/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/📏️stretches/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/📏️stretches/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/📏️stretches/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/📏️stretches/🎯️outcome/🔣️.json");

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

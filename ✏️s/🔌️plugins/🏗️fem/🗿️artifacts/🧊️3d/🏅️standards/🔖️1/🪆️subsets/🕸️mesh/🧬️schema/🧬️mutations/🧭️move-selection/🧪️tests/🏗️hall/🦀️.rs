//! 🧪️ `move-selection` fixture — `🏗️hall`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem3d-transform.py`.
//!
//! 🏗️ Stretching the portal hall's apron slab to twice its width about the right column line doubles its footprint in x; the gate node the hall never had is skipped with a partial warning.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🏗️hall/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🏗️hall/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🏗️hall/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🏗️hall/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🏗️hall/🎯️outcome/🔣️.json");

/// ▶️ The leaf carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}

/// ↩️ The computed inverse — whole-record replacements of every moved node and solid — restores `before` exactly.
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

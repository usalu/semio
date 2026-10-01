//! 🧪️ `move-selection` fixture — `🫓️denies`.
//!
//! Source of truth is the committed JSON quintet, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem2d-transform.py`.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening), the
//! SECOND real-world fem2d model — the first is the timber portal frame the subset-level
//! differential cases share. Every value is in SI base units.
//!
//! 🫓️ A zero factor would collapse the wall to a line no mesh can fill; the payload schema bounds the factor away from zero and the diff refuses it as an invariant.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️denies/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️denies/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️denies/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️denies/🎯️outcome/🔣️.json");

/// ⛔️ The refused or no-op leaf leaves the document byte-identical, emits the declared diagnostic and inverts to nothing.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(BEFORE, MUTATION, AFTER, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, None);
}

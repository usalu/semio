//! 🧪️ `move-selection` fixture — `🫓️flattens`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem3d-transform.py`.
//!
//! 🫓️ A zero factor would flatten the raft to a plane no mesh can fill; the payload schema bounds every factor away from zero and the diff refuses it as an invariant.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️flattens/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️flattens/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️flattens/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🧭️move-selection/🫓️flattens/🎯️outcome/🔣️.json");

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

//! 🧪️ `move-selection` fixture — `⏸️moves-nothing-f724d6`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem3d-transform.py`.
//!
//! ⏸️ The identity transform — no offset, no angle, unit factors — is a no-op on the targets it names.

use crate::laws;

static BEFORE: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧭️move-selection/⏸️moves-nothing-f724d6/📸️snapshot/⬅️before/🔣️.json");
static AFTER: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧭️move-selection/⏸️moves-nothing-f724d6/📸️snapshot/➡️after/🔣️.json");
static MUTATION: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧭️move-selection/⏸️moves-nothing-f724d6/🦠️mutation/🔣️.json");
static OUTCOME: std::sync::LazyLock<String> = semio_s_fem_test_support::fixture!("../../../../../🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🧭️move-selection/⏸️moves-nothing-f724d6/🎯️outcome/🔣️.json");

/// ⛔️ The refused or no-op leaf leaves the document byte-identical, emits the declared diagnostic and inverts to nothing.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(BEFORE.as_str(), MUTATION.as_str(), AFTER.as_str(), OUTCOME.as_str());
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE.as_str(), AFTER.as_str(), MUTATION.as_str(), None);
}

//! 🧪️ `reorder-pages` fixture — `📍️moves-a-middle-row`.
//!
//! 📍️ The addressed row sits in the MIDDLE of its ordered collection, so the concrete inverse must put it back at its original index, and the inverse steps' diffs must sum to the negative of the forward diff.

use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use protocol::Mutation;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔀reorder-pages/📍️moves-a-middle-row/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔀reorder-pages/📍️moves-a-middle-row/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔀reorder-pages/📍️moves-a-middle-row/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔀reorder-pages/📍️moves-a-middle-row/🔺️diff/🔣️.json");

fn before() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(BEFORE, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("reorder-pages/moves-a-middle-row: before snapshot decodes")
}
fn expected_after() -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(AFTER, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("reorder-pages/moves-a-middle-row: after snapshot decodes")
}
fn mutation() -> LayoutMutation {
    serde_json::from_str(MUTATION).expect("reorder-pages/moves-a-middle-row: mutation decodes")
}

/// ▶️ `reorder-pages` carries `before` to exactly the committed `after`.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let base = before();
    let applied = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("reorder-pages/moves-a-middle-row: applies to its committed before-snapshot");
    assert_eq!(applied, expected_after(), "reorder-pages/moves-a-middle-row: applied state differs from the committed after-snapshot");
    assert_ne!(applied, base, "reorder-pages/moves-a-middle-row: the forward mutation left the model untouched, so nothing was proved");
}

/// ↩️ The concrete inverse puts the middle row back at its original index.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_the_middle_position() {
    let base = before();
    let inverse = mutation().inverse(&base).expect("valid retained mutation inverse fixture");
    assert!(!inverse.is_empty(), "reorder-pages/moves-a-middle-row: a changing mutation must have a non-empty inverse");
    let mut snapshot = protocol::apply_diff(mutation().diff(&base).diff(), &base).expect("reorder-pages/moves-a-middle-row: forward applies");
    for step in &inverse {
        snapshot = protocol::apply_diff(step.diff(&snapshot).diff(), &snapshot).expect("reorder-pages/moves-a-middle-row: inverse step applies");
    }
    assert_eq!(snapshot, base, "reorder-pages/moves-a-middle-row: inverse did not restore the before-snapshot, row order included");
}

/// 🔺️ The sparse delta this kind produces is exactly the committed diff.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = mutation().diff(&base);
    let produced = serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(outcome.diff())).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "reorder-pages/moves-a-middle-row: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// ⚖️ The inverse steps' diffs sum, by `absorb`, to the negative of the forward diff.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}

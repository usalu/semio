//! 🧪️ `set-vertex-positions` fixture — `📍️sets`.
//!
//! The committed bundle is the `move-selection` `📌️pins` bundle seen from the other side: the two corners that slid one unit along x
//! are written straight to their absolute positions, so the same before-snapshot reaches the same after-snapshot.

use super::laws;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-vertex-positions/📍️sets/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-vertex-positions/📍️sets/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-vertex-positions/📍️sets/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-vertex-positions/📍️sets/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📍️set-vertex-positions/📍️sets/🎯️outcome/🔣️.json");

/// ▶️ The positions carry `before` to exactly the committed `after` and produce exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}

/// ↩️ The computed inverse — the base positions of the moved vertices as one `set-vertex-positions` — restores `before` exactly.
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

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    let (before, mutation) = laws::decode_case(BEFORE, MUTATION);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

/// ⚖️ A middle vertex row alone: only the vertex that moves is written and its base position is the whole undo.
#[semio_framework_async_macros::async_test]
async fn a_middle_vertex_row_inverts_to_its_base_position() {
    let (before, _) = laws::decode_case(BEFORE, MUTATION);
    let mutation = crate::LowpolyMutation::SetVertexPositions(SetVertexPositions { object_id: "obj-plane".into(), positions: vec![crate::diff::LowpolyVertexPosition { vertex: 1, position: [3.0, 0.0, -1.0], channels: Vec::new() }, crate::diff::LowpolyVertexPosition { vertex: 3, position: [-1.0, 0.0, 1.0] }] });
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

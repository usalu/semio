//! ➕ `insert-roofs` — adds a monopitch annex roof with a parapet behind the main roof.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply — the committed vector.

/// ➕ The committed `insert-roofs` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "insert-roofs",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-roofs/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn insert_roofs_annex_roof() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `insert-roofs` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn insert_roofs_annex_roof_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

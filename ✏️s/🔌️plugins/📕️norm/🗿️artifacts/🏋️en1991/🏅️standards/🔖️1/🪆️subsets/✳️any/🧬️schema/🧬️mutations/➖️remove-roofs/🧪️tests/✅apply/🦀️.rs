//! ➖ `remove-roofs` — removes the main duopitch roof.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/✅apply — the committed vector.

/// ➖ The committed `remove-roofs` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "remove-roofs",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-roofs/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn remove_roofs_main_roof() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `remove-roofs` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn remove_roofs_main_roof_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

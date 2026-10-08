//! ➖ `remove-self-weight-elements` — removes the reinforced-concrete slab layer.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/✅apply — the committed vector.

/// ➖ The committed `remove-self-weight-elements` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "remove-self-weight-elements",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn remove_self_weight_elements_slab() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `remove-self-weight-elements` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn remove_self_weight_elements_slab_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

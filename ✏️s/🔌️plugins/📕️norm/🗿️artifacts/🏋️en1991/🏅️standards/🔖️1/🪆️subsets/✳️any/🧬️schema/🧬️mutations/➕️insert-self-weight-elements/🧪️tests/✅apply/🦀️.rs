//! ➕ `insert-self-weight-elements` — adds a 50 mm cement screed layer behind the slab.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/✅apply — the committed vector.

/// ➕ The committed `insert-self-weight-elements` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "insert-self-weight-elements",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn insert_self_weight_elements_screed() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `insert-self-weight-elements` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn insert_self_weight_elements_screed_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

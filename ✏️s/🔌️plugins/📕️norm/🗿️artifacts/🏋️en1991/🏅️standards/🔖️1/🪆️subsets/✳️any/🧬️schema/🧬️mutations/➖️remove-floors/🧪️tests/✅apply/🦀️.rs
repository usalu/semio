//! ➖ `remove-floors` — removes the office floor.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply — the committed vector.

/// ➖ The committed `remove-floors` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "remove-floors",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-floors/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn remove_floors_office() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `remove-floors` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn remove_floors_office_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

//! ➕ `insert-floors` — adds an archive floor (category E1) behind the office floor.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/✅apply — the committed vector.

/// ➕ The committed `insert-floors` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "insert-floors",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-floors/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn insert_floors_archive() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `insert-floors` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn insert_floors_archive_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

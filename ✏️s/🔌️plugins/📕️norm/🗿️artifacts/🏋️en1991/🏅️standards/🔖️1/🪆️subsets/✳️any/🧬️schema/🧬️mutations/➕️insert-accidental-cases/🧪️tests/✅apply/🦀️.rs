//! ➕ `insert-accidental-cases` — adds an internal explosion case behind the vehicle impact case.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/✅apply — the committed vector.

/// ➕ The committed `insert-accidental-cases` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "insert-accidental-cases",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-accidental-cases/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn insert_accidental_cases_explosion() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `insert-accidental-cases` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn insert_accidental_cases_explosion_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

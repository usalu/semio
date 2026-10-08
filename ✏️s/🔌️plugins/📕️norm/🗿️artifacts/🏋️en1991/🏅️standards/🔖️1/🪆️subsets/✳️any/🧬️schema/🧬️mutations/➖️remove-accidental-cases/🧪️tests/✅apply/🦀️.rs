//! ➖ `remove-accidental-cases` — removes the vehicle impact case.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/✅apply — the committed vector.

/// ➖ The committed `remove-accidental-cases` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "remove-accidental-cases",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-accidental-cases/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn remove_accidental_cases_impact() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `remove-accidental-cases` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn remove_accidental_cases_impact_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

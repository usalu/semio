//! 🌊 `change-coast-or-island` — places the site on the coast.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply — the committed vector.

/// 🌊 The committed `change-coast-or-island` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-coast-or-island",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏝️change-coast-or-island/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_coast_or_island_coast() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-coast-or-island` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_coast_or_island_coast_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

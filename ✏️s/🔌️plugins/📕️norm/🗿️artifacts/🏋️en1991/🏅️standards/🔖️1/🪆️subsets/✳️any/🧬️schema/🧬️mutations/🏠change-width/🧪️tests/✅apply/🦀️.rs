//! 🏠 `change-width` — widens the building b from 15 m to 18.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply — the committed vector.

/// 🏠 The committed `change-width` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-width",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_width_18_5_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-width` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_width_18_5_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

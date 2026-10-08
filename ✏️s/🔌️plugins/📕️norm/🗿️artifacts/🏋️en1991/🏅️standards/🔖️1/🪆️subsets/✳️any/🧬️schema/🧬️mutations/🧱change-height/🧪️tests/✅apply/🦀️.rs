//! 🧱 `change-height` — raises the building height h from 20 m to 24 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply — the committed vector.

/// 🧱 The committed `change-height` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-height",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_height_24_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-height` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_height_24_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

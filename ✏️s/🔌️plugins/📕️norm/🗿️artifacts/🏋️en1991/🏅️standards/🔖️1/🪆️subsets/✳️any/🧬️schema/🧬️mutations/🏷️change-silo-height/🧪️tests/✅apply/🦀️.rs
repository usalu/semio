//! 📏 `change-silo-height` — raises the silo from 12 m to 18 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏷️change-silo-height/✅apply — the committed vector.

/// 📏 The committed `change-silo-height` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-silo-height",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-silo-height/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-silo-height/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-silo-height/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-silo-height/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-silo-height/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_silo_height_18_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-silo-height` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_silo_height_18_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

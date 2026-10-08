//! ⏫ `change-hoisting-speed` — speeds hoisting from 0.5 m/s to 1.25 m/s.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply — the committed vector.

/// ⏫ The committed `change-hoisting-speed` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-hoisting-speed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_hoisting_speed_1_25_m_s() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-hoisting-speed` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_hoisting_speed_1_25_m_s_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

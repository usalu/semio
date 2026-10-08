//! 💨 `change-en-vb` — raises the EN basic wind velocity v_b from 25 m/s to 27.5 m/s.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌬️change-en-vb/✅apply — the committed vector.

/// 💨 The committed `change-en-vb` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-en-vb",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬️change-en-vb/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬️change-en-vb/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬️change-en-vb/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬️change-en-vb/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬️change-en-vb/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_en_vb_27_5_m_s() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-en-vb` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_en_vb_27_5_m_s_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

//! 🔋 `change-assumed-qf-d` — raises the assumed design fire load density q_f,d from 420 MJ/m² to 511 MJ/m².
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/✅apply — the committed vector.

/// 🔋 The committed `change-assumed-qf-d` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-qf-d",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_qf_d_511_mj() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-qf-d` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_qf_d_511_mj_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

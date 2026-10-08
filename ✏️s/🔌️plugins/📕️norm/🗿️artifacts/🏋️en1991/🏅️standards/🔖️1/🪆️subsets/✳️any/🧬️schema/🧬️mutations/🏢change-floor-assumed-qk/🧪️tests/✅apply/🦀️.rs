//! 🏢 `change-floor-assumed-qk` — raises the assumed imposed load q_k of floor 0 from 1 kPa to 3 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/✅apply — the committed vector.

/// 🏢 The committed `change-floor-assumed-qk` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-floor-assumed-qk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-floor-assumed-qk/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_floor_assumed_qk_3_kpa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-floor-assumed-qk` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_floor_assumed_qk_3_kpa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

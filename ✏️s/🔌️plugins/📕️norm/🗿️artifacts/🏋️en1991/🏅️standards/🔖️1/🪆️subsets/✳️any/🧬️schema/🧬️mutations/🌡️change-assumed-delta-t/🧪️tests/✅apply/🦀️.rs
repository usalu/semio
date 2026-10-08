//! 📈 `change-assumed-delta-t` — raises the assumed uniform temperature difference from 10 K to 15 K.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌡️change-assumed-delta-t/✅apply — the committed vector.

/// 📈 The committed `change-assumed-delta-t` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-delta-t",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-assumed-delta-t/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-assumed-delta-t/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-assumed-delta-t/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-assumed-delta-t/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-assumed-delta-t/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_delta_t_15_k() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-delta-t` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_delta_t_15_k_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

//! 🚧 `change-self-weight-assumed-gk` — raises the assumed self-weight g_k of element 0 from 2 kPa to 5 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/✅apply — the committed vector.

/// 🚧 The committed `change-self-weight-assumed-gk` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-self-weight-assumed-gk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚧change-self-weight-assumed-gk/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_self_weight_assumed_gk_5_kpa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-self-weight-assumed-gk` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_self_weight_assumed_gk_5_kpa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

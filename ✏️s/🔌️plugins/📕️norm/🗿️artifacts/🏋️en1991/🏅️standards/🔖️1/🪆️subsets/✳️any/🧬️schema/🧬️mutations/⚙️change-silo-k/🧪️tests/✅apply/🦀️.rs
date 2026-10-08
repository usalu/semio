//! 🔩 `change-silo-k` — raises the lateral pressure ratio K from 0.4 to 0.55.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply — the committed vector.

/// 🔩 The committed `change-silo-k` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-silo-k",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚙️change-silo-k/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_silo_k_0_55() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-silo-k` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_silo_k_0_55_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

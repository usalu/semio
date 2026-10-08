//! 📐 `change-orography-factor` — raises the orography factor c_o from 1.0 to 1.15.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply — the committed vector.

/// 📐 The committed `change-orography-factor` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-orography-factor",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_orography_factor_1_15() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-orography-factor` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_orography_factor_1_15_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

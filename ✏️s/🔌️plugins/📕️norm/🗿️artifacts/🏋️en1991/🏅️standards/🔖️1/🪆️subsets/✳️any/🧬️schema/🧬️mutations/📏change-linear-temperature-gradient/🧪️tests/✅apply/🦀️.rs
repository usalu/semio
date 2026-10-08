//! 📈 `change-linear-temperature-gradient` — applies a 5 K linear temperature difference ΔT_M.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/✅apply — the committed vector.

/// 📈 The committed `change-linear-temperature-gradient` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-linear-temperature-gradient",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_linear_temperature_gradient_5_k() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-linear-temperature-gradient` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_linear_temperature_gradient_5_k_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

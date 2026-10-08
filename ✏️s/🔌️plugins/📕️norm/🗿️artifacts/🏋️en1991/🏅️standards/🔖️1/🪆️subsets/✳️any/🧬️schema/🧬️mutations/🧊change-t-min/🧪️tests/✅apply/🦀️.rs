//! 🧊 `change-t-min` — lowers the minimum shade air temperature T_min from -24 °C to -28 °C.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply — the committed vector.

/// 🧊 The committed `change-t-min` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-t-min",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_t_min_minus_28_c() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-t-min` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_t_min_minus_28_c_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

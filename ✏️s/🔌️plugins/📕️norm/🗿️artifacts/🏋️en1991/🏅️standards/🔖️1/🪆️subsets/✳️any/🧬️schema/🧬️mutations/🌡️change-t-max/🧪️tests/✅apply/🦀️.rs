//! 🌞 `change-t-max` — raises the maximum shade air temperature T_max from 37 °C to 39 °C.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌡️change-t-max/✅apply — the committed vector.

/// 🌞 The committed `change-t-max` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-t-max",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-t-max/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-t-max/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-t-max/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-t-max/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡️change-t-max/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_t_max_39_c() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-t-max` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_t_max_39_c_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

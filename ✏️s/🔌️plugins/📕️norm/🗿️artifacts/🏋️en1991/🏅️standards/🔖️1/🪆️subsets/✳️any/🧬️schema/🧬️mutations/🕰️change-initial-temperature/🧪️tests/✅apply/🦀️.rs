//! ⏰ `change-initial-temperature` — raises the initial temperature T_0 from 10 °C to 15 °C.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply — the committed vector.

/// ⏰ The committed `change-initial-temperature` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-initial-temperature",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_initial_temperature_15_c() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-initial-temperature` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_initial_temperature_15_c_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

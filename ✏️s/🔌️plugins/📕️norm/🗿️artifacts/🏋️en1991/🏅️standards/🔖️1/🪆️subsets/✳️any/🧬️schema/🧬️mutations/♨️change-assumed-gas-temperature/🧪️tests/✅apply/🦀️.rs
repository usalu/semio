//! 🔥 `change-assumed-gas-temperature` — raises the assumed gas temperature from 1200 K to 1300 K.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/♨️change-assumed-gas-temperature/✅apply — the committed vector.

/// 🔥 The committed `change-assumed-gas-temperature` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-gas-temperature",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️change-assumed-gas-temperature/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️change-assumed-gas-temperature/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️change-assumed-gas-temperature/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️change-assumed-gas-temperature/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨️change-assumed-gas-temperature/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_gas_temperature_1300_k() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-gas-temperature` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_gas_temperature_1300_k_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

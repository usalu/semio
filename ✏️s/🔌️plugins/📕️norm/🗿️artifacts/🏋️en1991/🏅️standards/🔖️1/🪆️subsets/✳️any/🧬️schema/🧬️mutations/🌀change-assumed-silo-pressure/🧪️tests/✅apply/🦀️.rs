//! 🌀 `change-assumed-silo-pressure` — raises the assumed horizontal wall pressure from 5 kPa to 8 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/✅apply — the committed vector.

/// 🌀 The committed `change-assumed-silo-pressure` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-silo-pressure",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_silo_pressure_8_kpa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-silo-pressure` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_silo_pressure_8_kpa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

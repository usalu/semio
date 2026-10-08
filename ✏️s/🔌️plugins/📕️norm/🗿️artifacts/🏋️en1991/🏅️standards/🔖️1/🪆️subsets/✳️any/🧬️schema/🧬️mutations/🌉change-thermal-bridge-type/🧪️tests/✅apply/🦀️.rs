//! 🌁 `change-thermal-bridge-type` — switches the thermal bridge deck type from 1 to 2.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/✅apply — the committed vector.

/// 🌁 The committed `change-thermal-bridge-type` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-thermal-bridge-type",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-thermal-bridge-type/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_thermal_bridge_type_type_2() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-thermal-bridge-type` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_thermal_bridge_type_type_2_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

//! 🌉 `change-thermal-element-type` — switches the thermal element from a building element to bridge deck type 2.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗️change-thermal-element-type/✅apply — the committed vector.

/// 🌉 The committed `change-thermal-element-type` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-thermal-element-type",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-thermal-element-type/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-thermal-element-type/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-thermal-element-type/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-thermal-element-type/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-thermal-element-type/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_thermal_element_type_bridge2() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-thermal-element-type` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_thermal_element_type_bridge2_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

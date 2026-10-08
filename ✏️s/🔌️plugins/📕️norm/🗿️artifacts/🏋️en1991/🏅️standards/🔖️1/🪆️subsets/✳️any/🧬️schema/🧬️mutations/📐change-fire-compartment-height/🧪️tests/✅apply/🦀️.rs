//! 📐 `change-fire-compartment-height` — raises the fire compartment height from 3 m to 3.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply — the committed vector.

/// 📐 The committed `change-fire-compartment-height` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-compartment-height",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-fire-compartment-height/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_compartment_height_3_5_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-compartment-height` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_compartment_height_3_5_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

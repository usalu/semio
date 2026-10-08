//! 📐 `change-fire-compartment-area` — enlarges the fire compartment floor area from 100 m² to 150 m².
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🗺️change-fire-compartment-area/✅apply — the committed vector.

/// 📐 The committed `change-fire-compartment-area` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-compartment-area",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-fire-compartment-area/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-fire-compartment-area/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-fire-compartment-area/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-fire-compartment-area/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-fire-compartment-area/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_compartment_area_150_m2() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-compartment-area` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_compartment_area_150_m2_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

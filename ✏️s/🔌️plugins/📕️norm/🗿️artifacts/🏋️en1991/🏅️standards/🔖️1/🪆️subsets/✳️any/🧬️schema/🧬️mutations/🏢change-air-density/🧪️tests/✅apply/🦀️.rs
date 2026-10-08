//! 💨 `change-air-density` — adopts the standard air density of 1.225 kg/m³.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply — the committed vector.

/// 💨 The committed `change-air-density` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-air-density",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏢change-air-density/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_air_density_1_225() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-air-density` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_air_density_1_225_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

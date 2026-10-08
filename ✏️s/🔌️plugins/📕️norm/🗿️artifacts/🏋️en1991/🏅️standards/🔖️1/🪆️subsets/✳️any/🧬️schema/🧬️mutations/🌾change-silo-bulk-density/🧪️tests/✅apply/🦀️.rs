//! 🌾 `change-silo-bulk-density` — raises the bulk unit weight γ from 8 kN/m³ to 9 kN/m³.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply — the committed vector.

/// 🌾 The committed `change-silo-bulk-density` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-silo-bulk-density",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_silo_bulk_density_9_kn_m3() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-silo-bulk-density` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_silo_bulk_density_9_kn_m3_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

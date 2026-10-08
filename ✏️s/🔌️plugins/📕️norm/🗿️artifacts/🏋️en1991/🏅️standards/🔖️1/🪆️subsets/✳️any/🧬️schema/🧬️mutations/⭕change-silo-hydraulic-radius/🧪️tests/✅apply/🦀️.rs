//! ⭕ `change-silo-hydraulic-radius` — widens the hydraulic radius from 1.5 m to 2.25 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/✅apply — the committed vector.

/// ⭕ The committed `change-silo-hydraulic-radius` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-silo-hydraulic-radius",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_silo_hydraulic_radius_2_25_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-silo-hydraulic-radius` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_silo_hydraulic_radius_2_25_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

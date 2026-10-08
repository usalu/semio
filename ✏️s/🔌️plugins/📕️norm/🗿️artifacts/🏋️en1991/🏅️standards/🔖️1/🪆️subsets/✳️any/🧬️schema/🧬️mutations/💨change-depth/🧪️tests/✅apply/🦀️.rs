//! 🏢 `change-depth` — deepens the building d from 25 m to 30 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply — the committed vector.

/// 🏢 The committed `change-depth` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-depth",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_depth_30_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-depth` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_depth_30_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

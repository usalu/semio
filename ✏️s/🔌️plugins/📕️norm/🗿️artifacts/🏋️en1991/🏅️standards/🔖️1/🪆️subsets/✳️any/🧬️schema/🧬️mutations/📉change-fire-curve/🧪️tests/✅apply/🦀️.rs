//! 📉 `change-fire-curve` — switches the nominal fire curve from standard to hydrocarbon.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/✅apply — the committed vector.

/// 📉 The committed `change-fire-curve` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-curve",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_curve_hydrocarbon() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-curve` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_curve_hydrocarbon_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

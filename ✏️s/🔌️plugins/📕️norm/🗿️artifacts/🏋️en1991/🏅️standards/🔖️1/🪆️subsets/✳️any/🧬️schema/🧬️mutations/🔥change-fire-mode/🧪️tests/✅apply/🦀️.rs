//! 🔥 `change-fire-mode` — switches the fire design from none to a parametric fire.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply — the committed vector.

/// 🔥 The committed `change-fire-mode` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-mode",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_mode_parametric() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-mode` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_mode_parametric_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

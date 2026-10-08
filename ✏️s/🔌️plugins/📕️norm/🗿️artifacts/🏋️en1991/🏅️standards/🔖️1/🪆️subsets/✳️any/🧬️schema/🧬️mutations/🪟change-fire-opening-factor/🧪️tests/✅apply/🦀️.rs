//! 🪟 `change-fire-opening-factor` — raises the opening factor O from 0.04 m^½ to 0.06 m^½.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🪟change-fire-opening-factor/✅apply — the committed vector.

/// 🪟 The committed `change-fire-opening-factor` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-fire-opening-factor",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪟change-fire-opening-factor/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪟change-fire-opening-factor/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪟change-fire-opening-factor/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪟change-fire-opening-factor/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪟change-fire-opening-factor/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_fire_opening_factor_0_06() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-fire-opening-factor` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_fire_opening_factor_0_06_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

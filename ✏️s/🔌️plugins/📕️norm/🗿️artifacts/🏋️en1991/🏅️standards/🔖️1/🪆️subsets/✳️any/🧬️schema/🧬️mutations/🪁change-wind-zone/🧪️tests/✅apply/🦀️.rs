//! 🪁 `change-wind-zone` — moves the site from wind zone 2 to wind zone 3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply — the committed vector.

/// 🪁 The committed `change-wind-zone` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-wind-zone",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🪁change-wind-zone/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_wind_zone_zone_3() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-wind-zone` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_wind_zone_zone_3_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

//! ⛄ `change-snow-zone` — moves the site from snow zone 2 to snow zone 3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/✅apply — the committed vector.

/// ⛄ The committed `change-snow-zone` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-snow-zone",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🗺️change-snow-zone/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_snow_zone_zone_3() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-snow-zone` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_snow_zone_zone_3_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

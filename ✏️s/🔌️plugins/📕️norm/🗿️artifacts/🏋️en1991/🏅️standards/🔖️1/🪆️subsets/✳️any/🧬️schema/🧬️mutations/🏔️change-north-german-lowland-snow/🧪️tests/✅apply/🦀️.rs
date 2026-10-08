//! ⛄ `change-north-german-lowland-snow` — flags the site as exposed to exceptional snow in the North German lowlands.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏔️change-north-german-lowland-snow/✅apply — the committed vector.

/// ⛄ The committed `change-north-german-lowland-snow` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-north-german-lowland-snow",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔️change-north-german-lowland-snow/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔️change-north-german-lowland-snow/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔️change-north-german-lowland-snow/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔️change-north-german-lowland-snow/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔️change-north-german-lowland-snow/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_north_german_lowland_snow_on() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-north-german-lowland-snow` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_north_german_lowland_snow_on_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

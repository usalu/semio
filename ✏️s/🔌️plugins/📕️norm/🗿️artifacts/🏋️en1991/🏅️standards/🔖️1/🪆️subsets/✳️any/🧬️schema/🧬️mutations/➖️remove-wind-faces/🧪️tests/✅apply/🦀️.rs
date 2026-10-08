//! ➖ `remove-wind-faces` — removes the windward façade zone D.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply — the committed vector.

/// ➖ The committed `remove-wind-faces` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "remove-wind-faces",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn remove_wind_faces_windward() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `remove-wind-faces` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn remove_wind_faces_windward_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

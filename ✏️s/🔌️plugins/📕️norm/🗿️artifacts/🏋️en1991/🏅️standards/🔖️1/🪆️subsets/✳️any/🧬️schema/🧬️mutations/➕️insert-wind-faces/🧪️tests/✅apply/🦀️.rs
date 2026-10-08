//! ➕ `insert-wind-faces` — adds the leeward façade zone E behind the windward zone D.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply — the committed vector.

/// ➕ The committed `insert-wind-faces` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "insert-wind-faces",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn insert_wind_faces_leeward() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `insert-wind-faces` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn insert_wind_faces_leeward_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

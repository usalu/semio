//! 🗻 `change-altitude` — raises the site altitude above sea level from 150 m to 480 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply — the committed vector.

/// 🗻 The committed `change-altitude` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-altitude",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_altitude_480_m() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-altitude` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_altitude_480_m_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

//! 🚫 `change-crane-claimed` — withdraws the crane claim.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply — the committed vector.

/// 🚫 The committed `change-crane-claimed` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-crane-claimed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏗️change-crane-claimed/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_crane_claimed_withdrawn() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-crane-claimed` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_crane_claimed_withdrawn_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

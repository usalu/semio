//! 🧲 `change-assumed-crane-horizontal` — assumes a 7.5 kN horizontal crane load.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply — the committed vector.

/// 🧲 The committed `change-assumed-crane-horizontal` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-crane-horizontal",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-assumed-crane-horizontal/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_crane_horizontal_7_5_kn() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-crane-horizontal` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_crane_horizontal_7_5_kn_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

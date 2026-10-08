//! 🔆 `change-assumed-h-net` — raises the assumed net heat flux from 25 kW/m² to 35 kW/m².
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔆change-assumed-h-net/✅apply — the committed vector.

/// 🔆 The committed `change-assumed-h-net` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-assumed-h-net",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔆change-assumed-h-net/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔆change-assumed-h-net/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔆change-assumed-h-net/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔆change-assumed-h-net/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔆change-assumed-h-net/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_assumed_h_net_35_kw_m2() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-assumed-h-net` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_assumed_h_net_35_kw_m2_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

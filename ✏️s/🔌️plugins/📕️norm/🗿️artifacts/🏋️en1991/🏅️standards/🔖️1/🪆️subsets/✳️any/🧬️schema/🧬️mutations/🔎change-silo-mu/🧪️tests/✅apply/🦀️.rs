//! 🔎 `change-silo-mu` — raises the wall friction coefficient μ from 0.4 to 0.5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply — the committed vector.

/// 🔎 The committed `change-silo-mu` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-silo-mu",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔎change-silo-mu/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_silo_mu_0_5() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-silo-mu` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_silo_mu_0_5_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

//! 🚗 `change-accidental-assumed-force` — raises the assumed impact force of accidental case 0 from 50 kN to 150 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply — the committed vector.

/// 🚗 The committed `change-accidental-assumed-force` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-accidental-assumed-force",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_accidental_assumed_force_150_kn() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-accidental-assumed-force` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_accidental_assumed_force_150_kn_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

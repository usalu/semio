//! ⛄ `change-roof-assumed-sk` — raises the assumed roof snow load of roof 0 from 200 Pa to 900 Pa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/✅apply — the committed vector.

/// ⛄ The committed `change-roof-assumed-sk` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-roof-assumed-sk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_roof_assumed_sk_900_pa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-roof-assumed-sk` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_roof_assumed_sk_900_pa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

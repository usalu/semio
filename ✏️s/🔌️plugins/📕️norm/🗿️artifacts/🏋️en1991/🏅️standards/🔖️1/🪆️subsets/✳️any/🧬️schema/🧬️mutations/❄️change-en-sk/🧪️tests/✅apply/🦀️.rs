//! ⛄ `change-en-sk` — raises the EN characteristic ground snow load s_k from 850 Pa to 1250 Pa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply — the committed vector.

/// ⛄ The committed `change-en-sk` vector holds the specification-vector law.
const VECTOR: super::Vector = super::Vector {
        kind: "change-en-sk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/🎯️outcome/🔣️.json"),
};

#[test]
fn change_en_sk_1250_pa() {
    super::assert_vector(VECTOR);
}

/// ⚖️ The committed `change-en-sk` vector's inverse diffs sum to the negative of its forward diff.
#[semio_framework_async_macros::async_test]
async fn change_en_sk_1250_pa_inverse_diffs_sum_to_the_negative_diff() {
    let (mutation, before) = super::committed_op_and_before(&VECTOR);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}

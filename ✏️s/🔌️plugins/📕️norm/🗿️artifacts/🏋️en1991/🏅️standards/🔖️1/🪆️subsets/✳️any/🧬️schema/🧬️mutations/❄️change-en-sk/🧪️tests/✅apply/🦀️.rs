//! ⛄ `change-en-sk` — raises the EN characteristic ground snow load s_k from 850 Pa to 1250 Pa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply — the committed vector.

/// ⛄ The committed `change-en-sk` vector holds the specification-vector law.
#[test]
fn change_en_sk_1250_pa() {
    super::assert_vector(super::Vector {
        kind: "change-en-sk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-en-sk/✅apply/🎯️outcome/🔣️.json"),
    });
}

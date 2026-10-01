//! ⚓ `change-permanents` — raises the unfavourable permanent action G-sup from 80 kN to 95 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply — the committed vector.

/// ⚓ The committed `change-permanents` vector holds the specification-vector law.
#[test]
fn change_permanents_95_kn() {
    super::assert_vector(super::Vector {
        kind: "change-permanents",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⚓️change-permanents/✅apply/🎯️outcome/🔣️.json"),
    });
}

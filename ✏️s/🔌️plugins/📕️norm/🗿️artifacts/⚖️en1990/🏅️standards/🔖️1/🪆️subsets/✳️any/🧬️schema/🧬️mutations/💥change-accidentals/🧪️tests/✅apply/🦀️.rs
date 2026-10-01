//! 💥 `change-accidentals` — raises the design impact action A_d from 50 kN to 75 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/✅apply — the committed vector.

/// 💥 The committed `change-accidentals` vector holds the specification-vector law.
#[test]
fn change_accidentals_75_kn() {
    super::assert_vector(super::Vector {
        kind: "change-accidentals",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/✅apply/🎯️outcome/🔣️.json"),
    });
}

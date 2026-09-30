//! 💥 `change-accidentals` — raises the design impact action A_d from 50 kN to 75 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/💥75-kn — the committed vector.

/// 💥 The committed `change-accidentals` vector holds the specification-vector law.
#[test]
fn change_accidentals_75_kn() {
    super::assert_vector(super::Vector {
        kind: "change-accidentals",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/💥75-kn/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/💥75-kn/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/💥75-kn/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/💥75-kn/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💥change-accidentals/💥75-kn/🎯️outcome/🔣️.json"),
    });
}

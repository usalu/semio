//! 🚗 `change-accidental-assumed-force` — raises the assumed impact force of accidental case 0 from 50 kN to 150 kN.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply — the committed vector.

/// 🚗 The committed `change-accidental-assumed-force` vector holds the specification-vector law.
#[test]
fn change_accidental_assumed_force_150_kn() {
    super::assert_vector(super::Vector {
        kind: "change-accidental-assumed-force",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚗change-accidental-assumed-force/✅apply/🎯️outcome/🔣️.json"),
    });
}

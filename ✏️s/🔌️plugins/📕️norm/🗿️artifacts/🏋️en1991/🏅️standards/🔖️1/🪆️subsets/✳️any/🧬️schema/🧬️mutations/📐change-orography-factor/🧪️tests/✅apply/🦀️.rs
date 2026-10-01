//! 📐 `change-orography-factor` — raises the orography factor c_o from 1.0 to 1.15.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply — the committed vector.

/// 📐 The committed `change-orography-factor` vector holds the specification-vector law.
#[test]
fn change_orography_factor_1_15() {
    super::assert_vector(super::Vector {
        kind: "change-orography-factor",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-orography-factor/✅apply/🎯️outcome/🔣️.json"),
    });
}

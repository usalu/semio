//! 🏠 `change-width` — widens the building b from 15 m to 18.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply — the committed vector.

/// 🏠 The committed `change-width` vector holds the specification-vector law.
#[test]
fn change_width_18_5_m() {
    super::assert_vector(super::Vector {
        kind: "change-width",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/✅apply/🎯️outcome/🔣️.json"),
    });
}

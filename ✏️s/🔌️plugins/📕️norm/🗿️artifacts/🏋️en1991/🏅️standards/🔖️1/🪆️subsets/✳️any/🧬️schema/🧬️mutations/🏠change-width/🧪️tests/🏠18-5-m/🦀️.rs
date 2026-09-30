//! 🏠 `change-width` — widens the building b from 15 m to 18.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏠change-width/🏠18-5-m — the committed vector.

/// 🏠 The committed `change-width` vector holds the specification-vector law.
#[test]
fn change_width_18_5_m() {
    super::assert_vector(super::Vector {
        kind: "change-width",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/🏠18-5-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/🏠18-5-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/🏠18-5-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/🏠18-5-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏠change-width/🏠18-5-m/🎯️outcome/🔣️.json"),
    });
}

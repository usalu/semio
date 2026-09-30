//! 🧱 `change-height` — raises the building height h from 20 m to 24 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧱change-height/🧱24-m — the committed vector.

/// 🧱 The committed `change-height` vector holds the specification-vector law.
#[test]
fn change_height_24_m() {
    super::assert_vector(super::Vector {
        kind: "change-height",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/🧱24-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/🧱24-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/🧱24-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/🧱24-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/🧱24-m/🎯️outcome/🔣️.json"),
    });
}

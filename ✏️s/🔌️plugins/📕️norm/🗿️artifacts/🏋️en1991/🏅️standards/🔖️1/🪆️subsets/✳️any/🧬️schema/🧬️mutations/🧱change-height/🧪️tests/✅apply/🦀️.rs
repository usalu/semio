//! 🧱 `change-height` — raises the building height h from 20 m to 24 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply — the committed vector.

/// 🧱 The committed `change-height` vector holds the specification-vector law.
#[test]
fn change_height_24_m() {
    super::assert_vector(super::Vector {
        kind: "change-height",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧱change-height/✅apply/🎯️outcome/🔣️.json"),
    });
}

//! 📏 `change-silo-height` — raises the silo from 12 m to 18 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏷change-silo-height/📏18-m — the committed vector.

/// 📏 The committed `change-silo-height` vector holds the specification-vector law.
#[test]
fn change_silo_height_18_m() {
    super::assert_vector(super::Vector {
        kind: "change-silo-height",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷change-silo-height/📏18-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷change-silo-height/📏18-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷change-silo-height/📏18-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷change-silo-height/📏18-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷change-silo-height/📏18-m/🎯️outcome/🔣️.json"),
    });
}

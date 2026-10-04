//! 🗻 `change-altitude` — raises the site altitude above sea level from 150 m to 480 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply — the committed vector.

/// 🗻 The committed `change-altitude` vector holds the specification-vector law.
#[test]
fn change_altitude_480_m() {
    super::assert_vector(super::Vector {
        kind: "change-altitude",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/❄️change-altitude/✅apply/🎯️outcome/🔣️.json"),
    });
}

//! 🗻 `change-altitude-m` — raises the site altitude from 120 m to 950 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/🗻950-m — the committed vector.

/// 🗻 The committed `change-altitude-m` vector holds the specification-vector law.
#[test]
fn change_altitude_m_950_m() {
    super::assert_vector(super::Vector {
        kind: "change-altitude-m",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/🗻950-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/🗻950-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/🗻950-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/🗻950-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/🗻950-m/🎯️outcome/🔣️.json"),
    });
}

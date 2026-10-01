//! 🗻 `change-altitude-m` — raises the site altitude from 120 m to 950 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/✅apply — the committed vector.

/// 🗻 The committed `change-altitude-m` vector holds the specification-vector law.
#[test]
fn change_altitude_m_950_m() {
    super::assert_vector(super::Vector {
        kind: "change-altitude-m",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛰️change-altitude-m/✅apply/🎯️outcome/🔣️.json"),
    });
}

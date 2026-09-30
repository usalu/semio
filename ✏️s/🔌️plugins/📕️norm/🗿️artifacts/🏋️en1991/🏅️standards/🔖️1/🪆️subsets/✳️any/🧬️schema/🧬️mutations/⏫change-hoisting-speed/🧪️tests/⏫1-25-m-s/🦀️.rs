//! ⏫ `change-hoisting-speed` — speeds hoisting from 0.5 m/s to 1.25 m/s.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/⏫1-25-m-s — the committed vector.

/// ⏫ The committed `change-hoisting-speed` vector holds the specification-vector law.
#[test]
fn change_hoisting_speed_1_25_m_s() {
    super::assert_vector(super::Vector {
        kind: "change-hoisting-speed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/⏫1-25-m-s/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/⏫1-25-m-s/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/⏫1-25-m-s/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/⏫1-25-m-s/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/⏫1-25-m-s/🎯️outcome/🔣️.json"),
    });
}

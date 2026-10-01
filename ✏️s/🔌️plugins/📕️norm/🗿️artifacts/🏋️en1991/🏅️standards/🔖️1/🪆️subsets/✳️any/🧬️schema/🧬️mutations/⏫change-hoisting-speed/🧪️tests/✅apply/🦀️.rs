//! ⏫ `change-hoisting-speed` — speeds hoisting from 0.5 m/s to 1.25 m/s.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply — the committed vector.

/// ⏫ The committed `change-hoisting-speed` vector holds the specification-vector law.
#[test]
fn change_hoisting_speed_1_25_m_s() {
    super::assert_vector(super::Vector {
        kind: "change-hoisting-speed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏫change-hoisting-speed/✅apply/🎯️outcome/🔣️.json"),
    });
}

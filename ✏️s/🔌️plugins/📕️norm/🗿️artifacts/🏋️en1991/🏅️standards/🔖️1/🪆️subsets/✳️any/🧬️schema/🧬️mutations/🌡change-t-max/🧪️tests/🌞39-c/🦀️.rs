//! 🌞 `change-t-max` — raises the maximum shade air temperature T_max from 37 °C to 39 °C.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌡change-t-max/🌞39-c — the committed vector.

/// 🌞 The committed `change-t-max` vector holds the specification-vector law.
#[test]
fn change_t_max_39_c() {
    super::assert_vector(super::Vector {
        kind: "change-t-max",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-t-max/🌞39-c/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-t-max/🌞39-c/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-t-max/🌞39-c/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-t-max/🌞39-c/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌡change-t-max/🌞39-c/🎯️outcome/🔣️.json"),
    });
}

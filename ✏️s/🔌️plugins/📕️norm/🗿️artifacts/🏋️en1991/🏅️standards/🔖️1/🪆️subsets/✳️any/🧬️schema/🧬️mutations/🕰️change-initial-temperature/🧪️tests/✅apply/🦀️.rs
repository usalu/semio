//! ⏰ `change-initial-temperature` — raises the initial temperature T_0 from 10 °C to 15 °C.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply — the committed vector.

/// ⏰ The committed `change-initial-temperature` vector holds the specification-vector law.
#[test]
fn change_initial_temperature_15_c() {
    super::assert_vector(super::Vector {
        kind: "change-initial-temperature",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🕰️change-initial-temperature/✅apply/🎯️outcome/🔣️.json"),
    });
}

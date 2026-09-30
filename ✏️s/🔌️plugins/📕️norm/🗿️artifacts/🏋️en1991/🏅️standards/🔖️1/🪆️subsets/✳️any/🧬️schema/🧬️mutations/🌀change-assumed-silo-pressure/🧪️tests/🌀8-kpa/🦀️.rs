//! 🌀 `change-assumed-silo-pressure` — raises the assumed horizontal wall pressure from 5 kPa to 8 kPa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/🌀8-kpa — the committed vector.

/// 🌀 The committed `change-assumed-silo-pressure` vector holds the specification-vector law.
#[test]
fn change_assumed_silo_pressure_8_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-silo-pressure",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/🌀8-kpa/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/🌀8-kpa/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/🌀8-kpa/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/🌀8-kpa/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌀change-assumed-silo-pressure/🌀8-kpa/🎯️outcome/🔣️.json"),
    });
}

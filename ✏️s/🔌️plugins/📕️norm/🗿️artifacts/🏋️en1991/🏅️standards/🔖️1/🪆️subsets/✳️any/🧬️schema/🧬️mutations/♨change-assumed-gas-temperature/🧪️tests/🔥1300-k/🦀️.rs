//! 🔥 `change-assumed-gas-temperature` — raises the assumed gas temperature from 1200 K to 1300 K.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/♨change-assumed-gas-temperature/🔥1300-k — the committed vector.

/// 🔥 The committed `change-assumed-gas-temperature` vector holds the specification-vector law.
#[test]
fn change_assumed_gas_temperature_1300_k() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-gas-temperature",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨change-assumed-gas-temperature/🔥1300-k/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨change-assumed-gas-temperature/🔥1300-k/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨change-assumed-gas-temperature/🔥1300-k/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨change-assumed-gas-temperature/🔥1300-k/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/♨change-assumed-gas-temperature/🔥1300-k/🎯️outcome/🔣️.json"),
    });
}

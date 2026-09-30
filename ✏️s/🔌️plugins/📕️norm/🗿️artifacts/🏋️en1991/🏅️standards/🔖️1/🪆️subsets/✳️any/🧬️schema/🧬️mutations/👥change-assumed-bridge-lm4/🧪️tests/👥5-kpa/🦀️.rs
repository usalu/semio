//! 👥 `change-assumed-bridge-lm4` — assumes a 5 kPa crowd load LM4.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/👥5-kpa — the committed vector.

/// 👥 The committed `change-assumed-bridge-lm4` vector holds the specification-vector law.
#[test]
fn change_assumed_bridge_lm4_5_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-bridge-lm4",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/👥5-kpa/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/👥5-kpa/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/👥5-kpa/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/👥5-kpa/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/👥5-kpa/🎯️outcome/🔣️.json"),
    });
}

//! 👥 `change-assumed-bridge-lm4` — assumes a 5 kPa crowd load LM4.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/✅apply — the committed vector.

/// 👥 The committed `change-assumed-bridge-lm4` vector holds the specification-vector law.
#[test]
fn change_assumed_bridge_lm4_5_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-bridge-lm4",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/👥change-assumed-bridge-lm4/✅apply/🎯️outcome/🔣️.json"),
    });
}

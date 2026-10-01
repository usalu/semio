//! 🚛 `change-assumed-bridge-lm3` — assumes a 600 kN special vehicle LM3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply — the committed vector.

/// 🚛 The committed `change-assumed-bridge-lm3` vector holds the specification-vector law.
#[test]
fn change_assumed_bridge_lm3_600_kn() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-bridge-lm3",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm3/✅apply/🎯️outcome/🔣️.json"),
    });
}

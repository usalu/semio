//! 🚛 `change-assumed-bridge-lm2` — assumes a 400 kN single axle load LM2.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm2/🚛400-kn — the committed vector.

/// 🚛 The committed `change-assumed-bridge-lm2` vector holds the specification-vector law.
#[test]
fn change_assumed_bridge_lm2_400_kn() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-bridge-lm2",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm2/🚛400-kn/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm2/🚛400-kn/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm2/🚛400-kn/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm2/🚛400-kn/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚛change-assumed-bridge-lm2/🚛400-kn/🎯️outcome/🔣️.json"),
    });
}

//! 🚚 `change-assumed-bridge-tandem` — assumes a 600 kN tandem system TS.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌾change-assumed-bridge-tandem/🚚600-kn — the committed vector.

/// 🚚 The committed `change-assumed-bridge-tandem` vector holds the specification-vector law.
#[test]
fn change_assumed_bridge_tandem_600_kn() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-bridge-tandem",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-assumed-bridge-tandem/🚚600-kn/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-assumed-bridge-tandem/🚚600-kn/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-assumed-bridge-tandem/🚚600-kn/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-assumed-bridge-tandem/🚚600-kn/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-assumed-bridge-tandem/🚚600-kn/🎯️outcome/🔣️.json"),
    });
}

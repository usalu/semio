//! 🚶 `change-assumed-bridge-footway` — assumes a 5 kPa footway load.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/🚶5-kpa — the committed vector.

/// 🚶 The committed `change-assumed-bridge-footway` vector holds the specification-vector law.
#[test]
fn change_assumed_bridge_footway_5_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-bridge-footway",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/🚶5-kpa/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/🚶5-kpa/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/🚶5-kpa/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/🚶5-kpa/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🚶change-assumed-bridge-footway/🚶5-kpa/🎯️outcome/🔣️.json"),
    });
}

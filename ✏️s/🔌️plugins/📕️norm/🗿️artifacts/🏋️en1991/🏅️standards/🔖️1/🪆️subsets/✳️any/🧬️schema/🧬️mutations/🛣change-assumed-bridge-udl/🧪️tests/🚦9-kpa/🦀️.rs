//! 🚦 `change-assumed-bridge-udl` — assumes a 9 kPa uniformly distributed load UDL.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🛣change-assumed-bridge-udl/🚦9-kpa — the committed vector.

/// 🚦 The committed `change-assumed-bridge-udl` vector holds the specification-vector law.
#[test]
fn change_assumed_bridge_udl_9_kpa() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-bridge-udl",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣change-assumed-bridge-udl/🚦9-kpa/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣change-assumed-bridge-udl/🚦9-kpa/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣change-assumed-bridge-udl/🚦9-kpa/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣change-assumed-bridge-udl/🚦9-kpa/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛣change-assumed-bridge-udl/🚦9-kpa/🎯️outcome/🔣️.json"),
    });
}

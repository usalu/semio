//! 🚦 `change-bridge-lane` — widens the carriageway from 1 to 3 notional lanes.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/🚦3-lanes — the committed vector.

/// 🚦 The committed `change-bridge-lane` vector holds the specification-vector law.
#[test]
fn change_bridge_lane_3_lanes() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-lane",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/🚦3-lanes/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/🚦3-lanes/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/🚦3-lanes/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/🚦3-lanes/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/🚦3-lanes/🎯️outcome/🔣️.json"),
    });
}

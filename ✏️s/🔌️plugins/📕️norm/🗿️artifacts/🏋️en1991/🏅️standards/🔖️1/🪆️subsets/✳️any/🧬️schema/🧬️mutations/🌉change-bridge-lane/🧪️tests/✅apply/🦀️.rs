//! 🚦 `change-bridge-lane` — widens the carriageway from 1 to 3 notional lanes.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply — the committed vector.

/// 🚦 The committed `change-bridge-lane` vector holds the specification-vector law.
#[test]
fn change_bridge_lane_3_lanes() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-lane",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌉change-bridge-lane/✅apply/🎯️outcome/🔣️.json"),
    });
}

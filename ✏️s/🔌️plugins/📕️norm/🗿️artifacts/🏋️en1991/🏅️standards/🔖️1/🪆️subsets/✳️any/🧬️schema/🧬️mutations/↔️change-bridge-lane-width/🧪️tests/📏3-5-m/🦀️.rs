//! 📏 `change-bridge-lane-width` — widens the notional lane from 3 m to 3.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/📏3-5-m — the committed vector.

/// 📏 The committed `change-bridge-lane-width` vector holds the specification-vector law.
#[test]
fn change_bridge_lane_width_3_5_m() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-lane-width",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/📏3-5-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/📏3-5-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/📏3-5-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/📏3-5-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/📏3-5-m/🎯️outcome/🔣️.json"),
    });
}

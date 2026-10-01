//! 📏 `change-bridge-lane-width` — widens the notional lane from 3 m to 3.5 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply — the committed vector.

/// 📏 The committed `change-bridge-lane-width` vector holds the specification-vector law.
#[test]
fn change_bridge_lane_width_3_5_m() {
    super::assert_vector(super::Vector {
        kind: "change-bridge-lane-width",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/↔️change-bridge-lane-width/✅apply/🎯️outcome/🔣️.json"),
    });
}

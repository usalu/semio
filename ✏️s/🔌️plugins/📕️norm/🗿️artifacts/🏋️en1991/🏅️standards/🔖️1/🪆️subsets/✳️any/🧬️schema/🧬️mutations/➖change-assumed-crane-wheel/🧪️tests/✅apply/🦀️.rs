//! 🛞 `change-assumed-crane-wheel` — assumes a 75 kN crane wheel load.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply — the committed vector.

/// 🛞 The committed `change-assumed-crane-wheel` vector holds the specification-vector law.
#[test]
fn change_assumed_crane_wheel_75_kn() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-crane-wheel",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖change-assumed-crane-wheel/✅apply/🎯️outcome/🔣️.json"),
    });
}

//! 🏢 `change-depth` — deepens the building d from 25 m to 30 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💨change-depth/🏢30-m — the committed vector.

/// 🏢 The committed `change-depth` vector holds the specification-vector law.
#[test]
fn change_depth_30_m() {
    super::assert_vector(super::Vector {
        kind: "change-depth",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/🏢30-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/🏢30-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/🏢30-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/🏢30-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/🏢30-m/🎯️outcome/🔣️.json"),
    });
}

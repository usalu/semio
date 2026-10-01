//! 🏢 `change-depth` — deepens the building d from 25 m to 30 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply — the committed vector.

/// 🏢 The committed `change-depth` vector holds the specification-vector law.
#[test]
fn change_depth_30_m() {
    super::assert_vector(super::Vector {
        kind: "change-depth",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💨change-depth/✅apply/🎯️outcome/🔣️.json"),
    });
}

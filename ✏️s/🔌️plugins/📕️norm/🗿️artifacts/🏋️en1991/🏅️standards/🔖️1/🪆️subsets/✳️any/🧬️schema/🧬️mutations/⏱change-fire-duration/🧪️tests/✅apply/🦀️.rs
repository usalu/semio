//! ⌛ `change-fire-duration` — extends the fire duration from 60 min to 90 min.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⏱change-fire-duration/✅apply — the committed vector.

/// ⌛ The committed `change-fire-duration` vector holds the specification-vector law.
#[test]
fn change_fire_duration_90_min() {
    super::assert_vector(super::Vector {
        kind: "change-fire-duration",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱change-fire-duration/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱change-fire-duration/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱change-fire-duration/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱change-fire-duration/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱change-fire-duration/✅apply/🎯️outcome/🔣️.json"),
    });
}

//! 🌋 `change-seismics` — raises the seismic importance class of E-1 from II to III.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply — the committed vector.

/// 🌋 The committed `change-seismics` vector holds the specification-vector law.
#[test]
fn change_seismics_class_iii() {
    super::assert_vector(super::Vector {
        kind: "change-seismics",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/✅apply/🎯️outcome/🔣️.json"),
    });
}

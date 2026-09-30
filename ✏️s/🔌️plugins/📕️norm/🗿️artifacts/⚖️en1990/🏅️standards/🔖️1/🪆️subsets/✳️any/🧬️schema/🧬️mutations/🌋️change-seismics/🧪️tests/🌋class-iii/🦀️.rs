//! 🌋 `change-seismics` — raises the seismic importance class of E-1 from II to III.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/🌋class-iii — the committed vector.

/// 🌋 The committed `change-seismics` vector holds the specification-vector law.
#[test]
fn change_seismics_class_iii() {
    super::assert_vector(super::Vector {
        kind: "change-seismics",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/🌋class-iii/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/🌋class-iii/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/🌋class-iii/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/🌋class-iii/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋️change-seismics/🌋class-iii/🎯️outcome/🔣️.json"),
    });
}

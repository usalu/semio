//! 🔗 `change-effects` — halves the influence of the wind action on beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/🔗half-wind — the committed vector.

/// 🔗 The committed `change-effects` vector holds the specification-vector law.
#[test]
fn change_effects_half_wind() {
    super::assert_vector(super::Vector {
        kind: "change-effects",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/🔗half-wind/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/🔗half-wind/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/🔗half-wind/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/🔗half-wind/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔗change-effects/🔗half-wind/🎯️outcome/🔣️.json"),
    });
}

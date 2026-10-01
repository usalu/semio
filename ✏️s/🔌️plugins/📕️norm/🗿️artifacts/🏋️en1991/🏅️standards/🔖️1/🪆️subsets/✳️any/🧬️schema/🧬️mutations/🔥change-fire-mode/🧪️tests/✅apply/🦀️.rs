//! 🔥 `change-fire-mode` — switches the fire design from none to a parametric fire.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply — the committed vector.

/// 🔥 The committed `change-fire-mode` vector holds the specification-vector law.
#[test]
fn change_fire_mode_parametric() {
    super::assert_vector(super::Vector {
        kind: "change-fire-mode",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-fire-mode/✅apply/🎯️outcome/🔣️.json"),
    });
}

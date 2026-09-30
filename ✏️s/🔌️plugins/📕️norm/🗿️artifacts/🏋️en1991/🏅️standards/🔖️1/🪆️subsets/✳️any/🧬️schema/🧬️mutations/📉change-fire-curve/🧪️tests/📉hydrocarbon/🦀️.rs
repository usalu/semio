//! 📉 `change-fire-curve` — switches the nominal fire curve from standard to hydrocarbon.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/📉hydrocarbon — the committed vector.

/// 📉 The committed `change-fire-curve` vector holds the specification-vector law.
#[test]
fn change_fire_curve_hydrocarbon() {
    super::assert_vector(super::Vector {
        kind: "change-fire-curve",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/📉hydrocarbon/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/📉hydrocarbon/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/📉hydrocarbon/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/📉hydrocarbon/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📉change-fire-curve/📉hydrocarbon/🎯️outcome/🔣️.json"),
    });
}

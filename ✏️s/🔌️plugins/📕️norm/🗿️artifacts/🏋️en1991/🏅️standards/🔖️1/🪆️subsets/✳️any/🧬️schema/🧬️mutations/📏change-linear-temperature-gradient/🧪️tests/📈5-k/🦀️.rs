//! 📈 `change-linear-temperature-gradient` — applies a 5 K linear temperature difference ΔT_M.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/📈5-k — the committed vector.

/// 📈 The committed `change-linear-temperature-gradient` vector holds the specification-vector law.
#[test]
fn change_linear_temperature_gradient_5_k() {
    super::assert_vector(super::Vector {
        kind: "change-linear-temperature-gradient",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/📈5-k/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/📈5-k/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/📈5-k/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/📈5-k/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📏change-linear-temperature-gradient/📈5-k/🎯️outcome/🔣️.json"),
    });
}

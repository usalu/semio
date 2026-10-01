//! 🧊 `change-t-min` — lowers the minimum shade air temperature T_min from -24 °C to -28 °C.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply — the committed vector.

/// 🧊 The committed `change-t-min` vector holds the specification-vector law.
#[test]
fn change_t_min_minus_28_c() {
    super::assert_vector(super::Vector {
        kind: "change-t-min",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧊change-t-min/✅apply/🎯️outcome/🔣️.json"),
    });
}

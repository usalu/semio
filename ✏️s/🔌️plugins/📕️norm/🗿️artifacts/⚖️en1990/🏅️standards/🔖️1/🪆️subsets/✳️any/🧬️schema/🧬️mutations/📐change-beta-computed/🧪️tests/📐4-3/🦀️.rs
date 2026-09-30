//! 📐 `change-beta-computed` — raises the computed reliability index β from 3.8 to 4.3.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/📐4-3 — the committed vector.

/// 📐 The committed `change-beta-computed` vector holds the specification-vector law.
#[test]
fn change_beta_computed_4_3() {
    super::assert_vector(super::Vector {
        kind: "change-beta-computed",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/📐4-3/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/📐4-3/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/📐4-3/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/📐4-3/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📐change-beta-computed/📐4-3/🎯️outcome/🔣️.json"),
    });
}

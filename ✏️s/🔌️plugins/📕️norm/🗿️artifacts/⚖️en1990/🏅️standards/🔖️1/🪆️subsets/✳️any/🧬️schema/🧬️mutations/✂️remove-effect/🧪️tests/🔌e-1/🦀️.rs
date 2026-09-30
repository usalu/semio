//! 🔌 `remove-effect` — detaches the seismic action E-1 from beam B1.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/🔌e-1 — the committed vector.

/// 🔌 The committed `remove-effect` vector holds the specification-vector law.
#[test]
fn remove_effect_e_1() {
    super::assert_vector(super::Vector {
        kind: "remove-effect",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/🔌e-1/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/🔌e-1/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/🔌e-1/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/🔌e-1/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/✂️remove-effect/🔌e-1/🎯️outcome/🔣️.json"),
    });
}

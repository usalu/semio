//! ➖ `remove-permanent` — removes the favourable permanent action G-inf.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/➖g-inf — the committed vector.

/// ➖ The committed `remove-permanent` vector holds the specification-vector law.
#[test]
fn remove_permanent_g_inf() {
    super::assert_vector(super::Vector {
        kind: "remove-permanent",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/➖g-inf/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/➖g-inf/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/➖g-inf/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/➖g-inf/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/➖g-inf/🎯️outcome/🔣️.json"),
    });
}

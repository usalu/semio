//! ➖ `remove-permanent` — removes the favourable permanent action G-inf.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply — the committed vector.

/// ➖ The committed `remove-permanent` vector holds the specification-vector law.
#[test]
fn remove_permanent_g_inf() {
    super::assert_vector(super::Vector {
        kind: "remove-permanent",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖remove-permanent/✅apply/🎯️outcome/🔣️.json"),
    });
}

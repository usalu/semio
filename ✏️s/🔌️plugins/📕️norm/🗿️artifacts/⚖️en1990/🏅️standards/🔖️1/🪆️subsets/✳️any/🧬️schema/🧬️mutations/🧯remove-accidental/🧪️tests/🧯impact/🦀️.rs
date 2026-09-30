//! 🧯 `remove-accidental` — removes the impact action A-impact.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/🧯impact — the committed vector.

/// 🧯 The committed `remove-accidental` vector holds the specification-vector law.
#[test]
fn remove_accidental_impact() {
    super::assert_vector(super::Vector {
        kind: "remove-accidental",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/🧯impact/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/🧯impact/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/🧯impact/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/🧯impact/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/🧯impact/🎯️outcome/🔣️.json"),
    });
}

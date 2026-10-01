//! 🧯 `remove-accidental` — removes the impact action A-impact.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/✅apply — the committed vector.

/// 🧯 The committed `remove-accidental` vector holds the specification-vector law.
#[test]
fn remove_accidental_impact() {
    super::assert_vector(super::Vector {
        kind: "remove-accidental",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🧯remove-accidental/✅apply/🎯️outcome/🔣️.json"),
    });
}

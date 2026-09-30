//! 📎 `insert-effect` — adds a second office load path into beam B1 at half influence.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/📎office-half — the committed vector.

/// 📎 The committed `insert-effect` vector holds the specification-vector law.
#[test]
fn insert_effect_office_half() {
    super::assert_vector(super::Vector {
        kind: "insert-effect",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/📎office-half/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/📎office-half/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/📎office-half/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/📎office-half/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📎insert-effect/📎office-half/🎯️outcome/🔣️.json"),
    });
}

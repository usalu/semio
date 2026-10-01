//! ⛔ `insert-variable` — re-applying the canonical insert to its own after-snapshot repeats an id it already holds, a `mutation.duplicate-id`.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/⛔dupe — the committed vector.

/// ⛔ The committed `insert-variable` dupe vector holds the specification-vector law.
#[test]
fn insert_variable_dupe() {
    super::assert_vector(super::Vector {
        kind: "insert-variable",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/⛔dupe/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
        diff: None,
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📥insert-variable/⛔dupe/🎯️outcome/🔣️.json"),
    });
}

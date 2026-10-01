//! ⛔ `insert-permanent` — re-applying the canonical insert to its own after-snapshot repeats an id it already holds, a `mutation.duplicate-id`.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/⛔dupe — the committed vector.

/// ⛔ The committed `insert-permanent` dupe vector holds the specification-vector law.
#[test]
fn insert_permanent_dupe() {
    super::assert_vector(super::Vector {
        kind: "insert-permanent",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/⛔dupe/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
        diff: None,
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕insert-permanent/⛔dupe/🎯️outcome/🔣️.json"),
    });
}

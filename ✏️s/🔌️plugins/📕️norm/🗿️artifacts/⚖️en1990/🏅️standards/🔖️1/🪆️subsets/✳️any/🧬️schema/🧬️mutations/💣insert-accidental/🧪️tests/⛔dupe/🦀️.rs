//! ⛔ `insert-accidental` — re-applying the canonical insert to its own after-snapshot repeats an id it already holds, a `mutation.duplicate-id`.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/⛔dupe — the committed vector.

/// ⛔ The committed `insert-accidental` dupe vector holds the specification-vector law.
#[test]
fn insert_accidental_dupe() {
    super::assert_vector(super::Vector {
        kind: "insert-accidental",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/⛔dupe/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
        diff: None,
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/💣insert-accidental/⛔dupe/🎯️outcome/🔣️.json"),
    });
}

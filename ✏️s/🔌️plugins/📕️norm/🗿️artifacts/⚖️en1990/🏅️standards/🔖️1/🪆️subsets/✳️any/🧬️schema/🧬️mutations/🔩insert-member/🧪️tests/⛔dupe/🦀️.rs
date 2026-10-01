//! ⛔ `insert-member` — re-applying the canonical insert to its own after-snapshot repeats an id it already holds, a `mutation.duplicate-id`.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/⛔dupe — the committed vector.

/// ⛔ The committed `insert-member` dupe vector holds the specification-vector law.
#[test]
fn insert_member_dupe() {
    super::assert_vector(super::Vector {
        kind: "insert-member",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/⛔dupe/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
        diff: None,
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔩insert-member/⛔dupe/🎯️outcome/🔣️.json"),
    });
}

//! ⛔ `insert-seismic` — re-applying the canonical insert to its own after-snapshot repeats an id it already holds, a `mutation.duplicate-id`.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/⛔dupe — the committed vector.

/// ⛔ The committed `insert-seismic` dupe vector holds the specification-vector law.
#[test]
fn insert_seismic_dupe() {
    super::assert_vector(super::Vector {
        kind: "insert-seismic",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/⛔dupe/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/⛔dupe/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/⛔dupe/📸️snapshot/➡️after/🔣️.json"),
        diff: None,
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/⛔dupe/🎯️outcome/🔣️.json"),
    });
}

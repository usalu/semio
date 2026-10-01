//! 🌋 `insert-seismic` — adds a 45 kN seismic action E-2 of importance class IV.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply — the committed vector.

/// 🌋 The committed `insert-seismic` vector holds the specification-vector law.
#[test]
fn insert_seismic_class_iv() {
    super::assert_vector(super::Vector {
        kind: "insert-seismic",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/✅apply/🎯️outcome/🔣️.json"),
    });
}

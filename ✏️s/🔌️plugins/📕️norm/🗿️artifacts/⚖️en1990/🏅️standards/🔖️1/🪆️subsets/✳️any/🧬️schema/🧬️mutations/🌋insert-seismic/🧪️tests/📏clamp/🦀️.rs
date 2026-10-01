//! 📏 `insert-seismic` — the canonical insert asked for a position past the list's end lands last under a `mutation.clamped` warning.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/📏clamp — the committed vector.

/// 📏 The committed `insert-seismic` clamp vector holds the specification-vector law.
#[test]
fn insert_seismic_clamp() {
    super::assert_vector(super::Vector {
        kind: "insert-seismic",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/📏clamp/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/📏clamp/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/📏clamp/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/📏clamp/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌋insert-seismic/📏clamp/🎯️outcome/🔣️.json"),
    });
}

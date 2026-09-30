//! 🌾 `change-silo-bulk-density` — raises the bulk unit weight γ from 8 kN/m³ to 9 kN/m³.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/🌾9-kn-m3 — the committed vector.

/// 🌾 The committed `change-silo-bulk-density` vector holds the specification-vector law.
#[test]
fn change_silo_bulk_density_9_kn_m3() {
    super::assert_vector(super::Vector {
        kind: "change-silo-bulk-density",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/🌾9-kn-m3/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/🌾9-kn-m3/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/🌾9-kn-m3/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/🌾9-kn-m3/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/🌾9-kn-m3/🎯️outcome/🔣️.json"),
    });
}

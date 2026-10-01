//! 🌾 `change-silo-bulk-density` — raises the bulk unit weight γ from 8 kN/m³ to 9 kN/m³.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply — the committed vector.

/// 🌾 The committed `change-silo-bulk-density` vector holds the specification-vector law.
#[test]
fn change_silo_bulk_density_9_kn_m3() {
    super::assert_vector(super::Vector {
        kind: "change-silo-bulk-density",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌾change-silo-bulk-density/✅apply/🎯️outcome/🔣️.json"),
    });
}

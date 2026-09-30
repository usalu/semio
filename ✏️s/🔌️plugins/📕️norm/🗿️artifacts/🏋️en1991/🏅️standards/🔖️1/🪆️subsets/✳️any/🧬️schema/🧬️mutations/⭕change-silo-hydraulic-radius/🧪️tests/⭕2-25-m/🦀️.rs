//! ⭕ `change-silo-hydraulic-radius` — widens the hydraulic radius from 1.5 m to 2.25 m.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/⭕2-25-m — the committed vector.

/// ⭕ The committed `change-silo-hydraulic-radius` vector holds the specification-vector law.
#[test]
fn change_silo_hydraulic_radius_2_25_m() {
    super::assert_vector(super::Vector {
        kind: "change-silo-hydraulic-radius",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/⭕2-25-m/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/⭕2-25-m/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/⭕2-25-m/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/⭕2-25-m/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⭕change-silo-hydraulic-radius/⭕2-25-m/🎯️outcome/🔣️.json"),
    });
}

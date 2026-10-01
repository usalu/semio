//! ⛽ `change-fire-load-density-qf` — raises the characteristic fire load density q_f,k from 420 MJ/m² to 600 MJ/m².
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply — the committed vector.

/// ⛽ The committed `change-fire-load-density-qf` vector holds the specification-vector law.
#[test]
fn change_fire_load_density_qf_600_mj() {
    super::assert_vector(super::Vector {
        kind: "change-fire-load-density-qf",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/✅apply/🎯️outcome/🔣️.json"),
    });
}

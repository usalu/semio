//! ⛽ `change-fire-load-density-qf` — raises the characteristic fire load density q_f,k from 420 MJ/m² to 600 MJ/m².
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/⛽600-mj — the committed vector.

/// ⛽ The committed `change-fire-load-density-qf` vector holds the specification-vector law.
#[test]
fn change_fire_load_density_qf_600_mj() {
    super::assert_vector(super::Vector {
        kind: "change-fire-load-density-qf",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/⛽600-mj/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/⛽600-mj/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/⛽600-mj/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/⛽600-mj/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⛽change-fire-load-density-qf/⛽600-mj/🎯️outcome/🔣️.json"),
    });
}

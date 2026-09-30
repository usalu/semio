//! 🔋 `change-assumed-qf-d` — raises the assumed design fire load density q_f,d from 420 MJ/m² to 511 MJ/m².
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/🔋511-mj — the committed vector.

/// 🔋 The committed `change-assumed-qf-d` vector holds the specification-vector law.
#[test]
fn change_assumed_qf_d_511_mj() {
    super::assert_vector(super::Vector {
        kind: "change-assumed-qf-d",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/🔋511-mj/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/🔋511-mj/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/🔋511-mj/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/🔋511-mj/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔋change-assumed-qf-d/🔋511-mj/🎯️outcome/🔣️.json"),
    });
}

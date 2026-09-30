//! 💨 `change-en-vb` — raises the EN basic wind velocity v_b from 25 m/s to 27.5 m/s.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌬change-en-vb/💨27-5-m-s — the committed vector.

/// 💨 The committed `change-en-vb` vector holds the specification-vector law.
#[test]
fn change_en_vb_27_5_m_s() {
    super::assert_vector(super::Vector {
        kind: "change-en-vb",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬change-en-vb/💨27-5-m-s/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬change-en-vb/💨27-5-m-s/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬change-en-vb/💨27-5-m-s/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬change-en-vb/💨27-5-m-s/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌬change-en-vb/💨27-5-m-s/🎯️outcome/🔣️.json"),
    });
}

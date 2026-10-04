//! 🪟 `change-wind-face-assumed-wp` — raises the assumed wind pressure of face 0 from 100 Pa to 750 Pa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🛡️change-wind-face-assumed-wp/✅apply — the committed vector.

/// 🪟 The committed `change-wind-face-assumed-wp` vector holds the specification-vector law.
#[test]
fn change_wind_face_assumed_wp_750_pa() {
    super::assert_vector(super::Vector {
        kind: "change-wind-face-assumed-wp",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛡️change-wind-face-assumed-wp/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛡️change-wind-face-assumed-wp/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛡️change-wind-face-assumed-wp/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛡️change-wind-face-assumed-wp/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🛡️change-wind-face-assumed-wp/✅apply/🎯️outcome/🔣️.json"),
    });
}

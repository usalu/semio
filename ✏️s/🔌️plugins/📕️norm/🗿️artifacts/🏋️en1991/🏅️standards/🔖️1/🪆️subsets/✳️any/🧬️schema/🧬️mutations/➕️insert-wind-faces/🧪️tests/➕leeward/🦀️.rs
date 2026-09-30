//! ➕ `insert-wind-faces` — adds the leeward façade zone E behind the windward zone D.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/➕leeward — the committed vector.

/// ➕ The committed `insert-wind-faces` vector holds the specification-vector law.
#[test]
fn insert_wind_faces_leeward() {
    super::assert_vector(super::Vector {
        kind: "insert-wind-faces",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/➕leeward/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/➕leeward/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/➕leeward/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/➕leeward/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/➕leeward/🎯️outcome/🔣️.json"),
    });
}

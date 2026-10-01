//! ➕ `insert-wind-faces` — adds the leeward façade zone E behind the windward zone D.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply — the committed vector.

/// ➕ The committed `insert-wind-faces` vector holds the specification-vector law.
#[test]
fn insert_wind_faces_leeward() {
    super::assert_vector(super::Vector {
        kind: "insert-wind-faces",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-wind-faces/✅apply/🎯️outcome/🔣️.json"),
    });
}

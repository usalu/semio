//! ➖ `remove-wind-faces` — removes the windward façade zone D.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply — the committed vector.

/// ➖ The committed `remove-wind-faces` vector holds the specification-vector law.
#[test]
fn remove_wind_faces_windward() {
    super::assert_vector(super::Vector {
        kind: "remove-wind-faces",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/✅apply/🎯️outcome/🔣️.json"),
    });
}

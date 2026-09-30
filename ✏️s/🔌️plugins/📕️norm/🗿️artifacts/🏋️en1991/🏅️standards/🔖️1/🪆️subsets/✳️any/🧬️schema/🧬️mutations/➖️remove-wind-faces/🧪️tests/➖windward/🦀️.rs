//! ➖ `remove-wind-faces` — removes the windward façade zone D.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/➖windward — the committed vector.

/// ➖ The committed `remove-wind-faces` vector holds the specification-vector law.
#[test]
fn remove_wind_faces_windward() {
    super::assert_vector(super::Vector {
        kind: "remove-wind-faces",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/➖windward/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/➖windward/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/➖windward/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/➖windward/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-wind-faces/➖windward/🎯️outcome/🔣️.json"),
    });
}

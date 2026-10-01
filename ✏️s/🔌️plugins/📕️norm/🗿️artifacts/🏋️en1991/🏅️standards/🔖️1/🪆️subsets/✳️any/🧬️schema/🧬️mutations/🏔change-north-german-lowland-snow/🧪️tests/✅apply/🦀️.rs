//! ⛄ `change-north-german-lowland-snow` — flags the site as exposed to exceptional snow in the North German lowlands.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏔change-north-german-lowland-snow/✅apply — the committed vector.

/// ⛄ The committed `change-north-german-lowland-snow` vector holds the specification-vector law.
#[test]
fn change_north_german_lowland_snow_on() {
    super::assert_vector(super::Vector {
        kind: "change-north-german-lowland-snow",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-north-german-lowland-snow/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-north-german-lowland-snow/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-north-german-lowland-snow/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-north-german-lowland-snow/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-north-german-lowland-snow/✅apply/🎯️outcome/🔣️.json"),
    });
}

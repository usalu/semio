//! ⛄ `change-exceptional-snow-north-german-lowlands` — flags the site as exposed to exceptional snow in the North German lowlands.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏔change-exceptional-snow-north-german-lowlands/⛄on — the committed vector.

/// ⛄ The committed `change-exceptional-snow-north-german-lowlands` vector holds the specification-vector law.
#[test]
fn change_exceptional_snow_north_german_lowlands_on() {
    super::assert_vector(super::Vector {
        kind: "change-exceptional-snow-north-german-lowlands",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-exceptional-snow-north-german-lowlands/⛄on/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-exceptional-snow-north-german-lowlands/⛄on/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-exceptional-snow-north-german-lowlands/⛄on/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-exceptional-snow-north-german-lowlands/⛄on/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏔change-exceptional-snow-north-german-lowlands/⛄on/🎯️outcome/🔣️.json"),
    });
}

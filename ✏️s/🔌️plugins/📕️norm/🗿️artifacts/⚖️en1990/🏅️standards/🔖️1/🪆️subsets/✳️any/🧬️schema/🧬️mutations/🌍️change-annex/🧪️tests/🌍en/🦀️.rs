//! 🌍 `change-annex` — switches the national annex from the German NA to the recommended EN values.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍en — the committed vector.

/// 🌍 The committed `change-annex` vector holds the specification-vector law.
#[test]
fn change_annex_en() {
    super::assert_vector(super::Vector {
        kind: "change-annex",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍en/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍en/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍en/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍en/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍️change-annex/🌍en/🎯️outcome/🔣️.json"),
    });
}

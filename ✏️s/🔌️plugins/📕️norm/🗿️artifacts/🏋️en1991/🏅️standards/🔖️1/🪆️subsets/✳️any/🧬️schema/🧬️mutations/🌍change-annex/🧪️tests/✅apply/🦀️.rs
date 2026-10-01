//! 🌍 `change-annex` — switches the national annex from the German NA to the recommended EN values.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply — the committed vector.

/// 🌍 The committed `change-annex` vector holds the specification-vector law.
#[test]
fn change_annex_en() {
    super::assert_vector(super::Vector {
        kind: "change-annex",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌍change-annex/✅apply/🎯️outcome/🔣️.json"),
    });
}

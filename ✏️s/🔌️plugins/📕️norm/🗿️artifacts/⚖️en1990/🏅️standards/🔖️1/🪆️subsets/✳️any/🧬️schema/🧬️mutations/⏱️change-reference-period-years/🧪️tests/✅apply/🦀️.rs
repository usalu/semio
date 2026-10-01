//! ⌛ `change-reference-period-years` — shortens the reliability reference period from 50 years to 1 year.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/⏱️change-reference-period-years/✅apply — the committed vector.

/// ⌛ The committed `change-reference-period-years` vector holds the specification-vector law.
#[test]
fn change_reference_period_years_1_y() {
    super::assert_vector(super::Vector {
        kind: "change-reference-period-years",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-reference-period-years/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-reference-period-years/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-reference-period-years/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-reference-period-years/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/⏱️change-reference-period-years/✅apply/🎯️outcome/🔣️.json"),
    });
}

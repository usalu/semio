//! 📆 `change-design-working-life-years` — doubles the design working life from 50 to 100 years.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/📆100-y — the committed vector.

/// 📆 The committed `change-design-working-life-years` vector holds the specification-vector law.
#[test]
fn change_design_working_life_years_100_y() {
    super::assert_vector(super::Vector {
        kind: "change-design-working-life-years",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/📆100-y/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/📆100-y/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/📆100-y/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/📆100-y/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/📆100-y/🎯️outcome/🔣️.json"),
    });
}

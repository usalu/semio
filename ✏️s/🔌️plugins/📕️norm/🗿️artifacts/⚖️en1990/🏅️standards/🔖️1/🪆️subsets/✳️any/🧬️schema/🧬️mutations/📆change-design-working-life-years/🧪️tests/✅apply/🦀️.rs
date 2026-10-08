//! 📆 `change-design-working-life-years` — doubles the design working life from 50 to 100 years.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-design-working-life-years",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📆change-design-working-life-years/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 📆 The committed `change-design-working-life-years` vector holds the specification-vector law.
#[test]
fn change_design_working_life_years_100_y() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}


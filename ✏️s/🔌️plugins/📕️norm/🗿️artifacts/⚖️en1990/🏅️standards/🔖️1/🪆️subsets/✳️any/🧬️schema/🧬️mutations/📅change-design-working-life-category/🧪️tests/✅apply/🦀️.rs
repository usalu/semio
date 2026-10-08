//! ⏳ `change-design-working-life-category` — reclassifies the design working life from category 4 to category 5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-design-working-life-category",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// ⏳ The committed `change-design-working-life-category` vector holds the specification-vector law.
#[test]
fn change_design_working_life_category_cat_5() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}


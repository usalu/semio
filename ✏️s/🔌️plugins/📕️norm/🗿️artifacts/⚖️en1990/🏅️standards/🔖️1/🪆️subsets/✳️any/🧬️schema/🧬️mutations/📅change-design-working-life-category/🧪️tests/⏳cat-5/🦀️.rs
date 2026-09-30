//! ⏳ `change-design-working-life-category` — reclassifies the design working life from category 4 to category 5.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/⏳cat-5 — the committed vector.

/// ⏳ The committed `change-design-working-life-category` vector holds the specification-vector law.
#[test]
fn change_design_working_life_category_cat_5() {
    super::assert_vector(super::Vector {
        kind: "change-design-working-life-category",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/⏳cat-5/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/⏳cat-5/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/⏳cat-5/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/⏳cat-5/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/📅change-design-working-life-category/⏳cat-5/🎯️outcome/🔣️.json"),
    });
}

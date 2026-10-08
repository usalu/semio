//! ⛄ `change-variables` — replaces the variable actions with a set that adds a 15 kN snow action.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-variables",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏋️change-variables/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// ⛄ The committed `change-variables` vector holds the specification-vector law.
#[test]
fn change_variables_adds_snow() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}


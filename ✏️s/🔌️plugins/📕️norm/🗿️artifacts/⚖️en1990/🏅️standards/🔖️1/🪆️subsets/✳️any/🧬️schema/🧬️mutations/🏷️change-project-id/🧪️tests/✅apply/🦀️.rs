//! 📛 `change-project-id` — renames the project to office-tower-b.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/✅apply — the committed vector.

fn vector() -> super::Vector {
    super::Vector {
        kind: "change-project-id",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/✅apply/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/✅apply/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/✅apply/📸️snapshot/➡️after/🔣️.json"),
        diff: Some(include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/✅apply/🔺️diff/🔣️.json")),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/✅apply/🎯️outcome/🔣️.json"),
    }
}

/// 📛 The committed `change-project-id` vector holds the specification-vector law.
#[test]
fn change_project_id_office_tower_b() {
    super::assert_vector(vector());
}

#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    super::assert_inverse_sum_law(vector()).await;
}


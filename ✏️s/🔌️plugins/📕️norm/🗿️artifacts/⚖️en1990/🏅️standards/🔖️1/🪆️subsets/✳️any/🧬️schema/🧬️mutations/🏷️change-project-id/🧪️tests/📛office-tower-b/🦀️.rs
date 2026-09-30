//! 📛 `change-project-id` — renames the project to office-tower-b.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/📛office-tower-b — the committed vector.

/// 📛 The committed `change-project-id` vector holds the specification-vector law.
#[test]
fn change_project_id_office_tower_b() {
    super::assert_vector(super::Vector {
        kind: "change-project-id",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/📛office-tower-b/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/📛office-tower-b/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/📛office-tower-b/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/📛office-tower-b/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🏷️change-project-id/📛office-tower-b/🎯️outcome/🔣️.json"),
    });
}

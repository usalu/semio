//! 🧰 `change-construction-activity` — switches the execution-stage activity from scaffolding to formwork.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/🧰formwork — the committed vector.

/// 🧰 The committed `change-construction-activity` vector holds the specification-vector law.
#[test]
fn change_construction_activity_formwork() {
    super::assert_vector(super::Vector {
        kind: "change-construction-activity",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/🧰formwork/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/🧰formwork/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/🧰formwork/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/🧰formwork/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🔥change-construction-activity/🧰formwork/🎯️outcome/🔣️.json"),
    });
}

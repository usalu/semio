//! ➕ `insert-self-weight-elements` — adds a 50 mm cement screed layer behind the slab.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/➕screed — the committed vector.

/// ➕ The committed `insert-self-weight-elements` vector holds the specification-vector law.
#[test]
fn insert_self_weight_elements_screed() {
    super::assert_vector(super::Vector {
        kind: "insert-self-weight-elements",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/➕screed/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/➕screed/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/➕screed/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/➕screed/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➕️insert-self-weight-elements/➕screed/🎯️outcome/🔣️.json"),
    });
}

//! ➖ `remove-self-weight-elements` — removes the reinforced-concrete slab layer.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/➖slab — the committed vector.

/// ➖ The committed `remove-self-weight-elements` vector holds the specification-vector law.
#[test]
fn remove_self_weight_elements_slab() {
    super::assert_vector(super::Vector {
        kind: "remove-self-weight-elements",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/➖slab/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/➖slab/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/➖slab/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/➖slab/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️remove-self-weight-elements/➖slab/🎯️outcome/🔣️.json"),
    });
}

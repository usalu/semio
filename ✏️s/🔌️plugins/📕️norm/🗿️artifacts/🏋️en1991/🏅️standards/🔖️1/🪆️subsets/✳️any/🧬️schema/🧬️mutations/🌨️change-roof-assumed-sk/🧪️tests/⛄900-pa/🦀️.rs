//! ⛄ `change-roof-assumed-sk` — raises the assumed roof snow load of roof 0 from 200 Pa to 900 Pa.
//!
//! @see ../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/⛄900-pa — the committed vector.

/// ⛄ The committed `change-roof-assumed-sk` vector holds the specification-vector law.
#[test]
fn change_roof_assumed_sk_900_pa() {
    super::assert_vector(super::Vector {
        kind: "change-roof-assumed-sk",
        before: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/⛄900-pa/📸️snapshot/⬅️before/🔣️.json"),
        mutation: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/⛄900-pa/🦠️mutation/🔣️.json"),
        after: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/⛄900-pa/📸️snapshot/➡️after/🔣️.json"),
        diff: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/⛄900-pa/🔺️diff/🔣️.json"),
        outcome: include_str!("../../../../../🧫️fixtures/🧬️mutations/🌨️change-roof-assumed-sk/⛄900-pa/🎯️outcome/🔣️.json"),
    });
}

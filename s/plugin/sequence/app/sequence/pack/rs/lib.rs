//! 📦 Sequence app — binary document surface + laws (constitutional: pack).

use sequence::SequenceFixture;
use store::PackError;

/// 📦 Encodes a `SequenceFixture` to its binary pack form.
pub fn encode(fixture: &SequenceFixture) -> Vec<u8> {
    store::DocumentPack::encode_pack(fixture)
}

/// 📖 Decodes a `SequenceFixture` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<SequenceFixture, PackError> {
    <SequenceFixture as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Dictionary, Value};
    use sequence::{default_fixture, SequenceStep, SlotRef, StepParams};

    #[test]
    fn pack_round_trips_default_fixture() {
        let fixture = default_fixture();
        store::test_support::assert_dsl_pack_equivalence(&fixture);
        let bytes = encode(&fixture);
        assert_eq!(decode(&bytes).expect("decode"), fixture);
    }

    #[test]
    fn default_sequence_example_pack_round_trips() {
        let fixture = sequence_dsl::parse_dsl(sequence_dsl::SEQUENCE_EXAMPLE_TEXT).expect("default.sequence must parse");
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    #[test]
    fn pack_round_trips_fixture_with_slots_and_nested_params() {
        let mut fixture = default_fixture();
        fixture.steps.push(SequenceStep {
            id: "step-3".into(),
            kind: "control.if".into(),
            params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))),
            x: 560.0,
            y: 0.0,
            slot: None,
            collapsed: true,
        });
        fixture.steps.push(SequenceStep {
            id: "step-4".into(),
            kind: "log.print".into(),
            params: StepParams::new().insert("message", Value::Atom(Atom::String("nested \"quote\" and \\ backslash".into()))).insert(
                "meta",
                Value::Dictionary(Dictionary::new().insert("count", Value::Atom(Atom::Integer(-3))).insert("ratio", Value::Atom(Atom::Decimal(2.5)))),
            ),
            x: 560.0,
            y: 160.0,
            slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
            collapsed: false,
        });
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }
}
//#endregion 🧪Tests

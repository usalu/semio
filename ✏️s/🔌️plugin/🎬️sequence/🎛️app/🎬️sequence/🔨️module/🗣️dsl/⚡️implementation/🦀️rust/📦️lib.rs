//! 📜️ Sequence app — textual document grammar surface + laws (constitutional: dsl).

use sequence::SequenceFixture;

/// 📄️ The handcrafted `.sequence` DSL-text fixture (regenerated from `default_fixture()`'s canonical
/// print form) — the permanent proof that the checked-in fixture still parses and round trips, not a
/// one-time migration script.
pub const SEQUENCE_EXAMPLE_TEXT: &str = include_str!("../../../../../../../../../✏️s/🔌️plugin/🎬️sequence/📚️example/🎬️default.sequence");

/// 📖️ Parses `.sequence` DSL text into a `SequenceFixture`.
pub fn parse_dsl(text: &str) -> Result<SequenceFixture, store::TextError> {
    <SequenceFixture as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `SequenceFixture` back to `.sequence` DSL text.
pub fn print_dsl(fixture: &SequenceFixture) -> String {
    store::DocumentDsl::print_dsl(fixture)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use neural_engine::{Atom, Dictionary, Value};
    use sequence::{default_fixture, SequenceStep, SlotRef, StepParams};

    #[test]
    fn dsl_round_trips_default_fixture() {
        store::test_support::assert_dsl_round_trip(&default_fixture());
    }

    #[test]
    fn default_sequence_example_dsl_round_trips() {
        let fixture = parse_dsl(SEQUENCE_EXAMPLE_TEXT).expect("🎬️default.sequence must parse");
        store::test_support::assert_dsl_round_trip(&fixture);
    }

    #[test]
    fn dsl_round_trips_fixture_with_slots_and_nested_params() {
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
        store::test_support::assert_dsl_round_trip(&fixture);
    }
}
//#endregion 🧪️Tests

//! 📜️ Sequence artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::ArtifactDsl for SequenceSnapshot` is implemented on the snapshot facet (see
//! `📸️snapshot/🧬️schema`, `🔖️HandcraftedArtifactCodecs`). This component only carries the grammar
//! doc-string, example text, and round-trip laws. Ticket
//! `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`) dropped the old
//! `SequenceEdgeDsl` unified-`dsl::Wire` mirror here — the snapshot no longer embeds `edges`
//! structurally in its own text grammar at all (only the opaque composed `content` handle), so a
//! per-edge DSL mirror has nothing left to mirror.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::sequence::SequenceSnapshot;

//#region 🔖️Example
/// 📄️ The handcrafted `.sequence` DSL-text fixture (regenerated from `default_snapshot()`'s canonical
/// print form) — the permanent proof that the checked-in fixture still parses and round trips, not a
/// one-time migration script.
pub const SEQUENCE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.sequence` DSL text into a `SequenceSnapshot`.
pub fn parse_dsl(text: &str) -> Result<SequenceSnapshot, store::TextError> {
    <SequenceSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `SequenceSnapshot` back to `.sequence` DSL text.
pub fn print_dsl(snapshot: &SequenceSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_snapshot, SequenceStep, SlotRef, StepParams};

    #[test]
    fn dsl_round_trips_default_snapshot() {
        store::os_store::test_support::assert_dsl_round_trip(&default_snapshot());
    }

    #[test]
    fn default_sequence_example_dsl_round_trips() {
        let snapshot = parse_dsl(SEQUENCE_EXAMPLE_TEXT).expect("🎬️default.sequence must parse");
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }

    #[test]
    fn dsl_round_trips_snapshot_with_slots_and_nested_params() {
        use neural_engine::{Atom, Dictionary, Value};
        let mut fixture = default_snapshot().to_fixture();
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
            params: StepParams::new()
                .insert("message", Value::Atom(Atom::String("nested \"quote\" and \\ backslash".into())))
                .insert("meta", Value::Dictionary(Dictionary::new().insert("count", Value::Atom(Atom::Integer(-3))).insert("ratio", Value::Atom(Atom::Decimal(2.5))))),
            x: 560.0,
            y: 160.0,
            slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
            collapsed: false,
        });
        let snapshot = SequenceSnapshot::from_fixture(fixture);
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }
}
//#endregion 🧪️Tests

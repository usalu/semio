//! 📜️ Sequence artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::DocumentDsl for SequenceSnapshot` is implemented on the snapshot facet (see
//! `📸️snapshot/🧬️schema`). This component only adds edge wire mirrors, example text, and laws.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::sequence::{SequenceEdge, SequenceSnapshot};

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `SequenceEdge` — models the `from`/`to` step-id pair as a single unified
/// `dsl::Wire` literal (`from->to`) instead of two separate string fields, per the unified syntax
/// law for graph edges/connections. Converts at the `store::DocumentDsl`/`protocol::OpText` boundary
/// only; `SequenceEdge` itself (and every consumer matching on its `from`/`to` fields directly)
/// is completely untouched.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct SequenceEdgeDsl {
    pub id: String,
    pub link: dsl::Wire,
}

pub fn sequence_edge_to_dsl(edge: &SequenceEdge) -> SequenceEdgeDsl {
    let from = dsl::WireNode { id: edge.from.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.to.clone(), kind: None, port: None };
    SequenceEdgeDsl {
        id: edge.id.clone(),
        link: dsl::Wire(dsl::WireValue {
            from,
            edge: Some((true, to)),
            edge_label: dsl::WireEdgeLabel::default(),
            properties: dsl::DslValue::Object(Vec::new()),
        }),
    }
}

pub fn sequence_edge_from_dsl(edge: SequenceEdgeDsl) -> Result<SequenceEdge, String> {
    let dsl::WireValue { from, edge: link, .. } = edge.link.0;
    let (directed, to) = link.ok_or_else(|| "sequence edge wire literal must have a target".to_string())?;
    if !directed {
        return Err("sequence edge wire literal must be directed".into());
    }
    Ok(SequenceEdge { id: edge.id, from: from.id, to: to.id })
}
//#endregion 🔖️Dsl

//#region 🔖️Example
/// 📄️ The handcrafted `.sequence` DSL-text fixture (regenerated from `default_snapshot()`'s canonical
/// print form) — the permanent proof that the checked-in fixture still parses and round trips, not a
/// one-time migration script.
pub const SEQUENCE_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.sequence` DSL text into a `SequenceSnapshot`.
pub fn parse_dsl(text: &str) -> Result<SequenceSnapshot, store::TextError> {
    <SequenceSnapshot as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `SequenceSnapshot` back to `.sequence` DSL text.
pub fn print_dsl(snapshot: &SequenceSnapshot) -> String {
    store::DocumentDsl::print_dsl(snapshot)
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_snapshot, SlotRef, StepParams};
    use neural_engine::{Atom, Dictionary, Value};

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
        let mut snapshot = default_snapshot();
        snapshot.steps.push(crate::artifacts::sequence::SequenceStep {
            id: "step-3".into(),
            kind: "control.if".into(),
            params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))),
            x: 560.0,
            y: 0.0,
            slot: None,
            collapsed: true,
        });
        snapshot.steps.push(crate::artifacts::sequence::SequenceStep {
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
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }
}
//#endregion 🧪️Tests

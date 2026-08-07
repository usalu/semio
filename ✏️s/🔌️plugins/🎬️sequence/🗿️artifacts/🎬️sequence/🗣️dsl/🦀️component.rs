//! 📜️ Sequence artifact — textual document grammar surface + laws (constitutional: dsl).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::sequence::{SequenceEdge, SequenceFixture};

//#region 🔖️Dsl
/// 🔌️ DSL-only mirror of `SequenceEdge` — models the `from`/`to` step-id pair as a single unified
/// `dsl::Wire` literal (`from->to`) instead of two separate string fields, per the unified syntax
/// law for graph edges/connections. Converts at the `store::DocumentDsl`/`protocol::OpText` boundary
/// only (`sequence_fixture_to_dsl` here and `🔧️op`'s `sequence_operation_to_dsl`, and their
/// inverses); `SequenceEdge` itself (and every consumer matching on its `from`/`to` fields directly)
/// is completely untouched. `SequenceEdgePatch` stays a plain sparse two-`Option<String>` patch
/// rather than a `Wire` — a `Wire`'s two endpoints are not independently optional, but `EdgesPatch`
/// legitimately needs to rewire only `from` OR only `to`. `pub` (unlike the document-only
/// `SequenceFixtureDsl` below) because `🔧️op`'s `SequenceOperationDsl::EdgesAdd` embeds it too.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
pub struct SequenceEdgeDsl {
    pub id: String,
    pub link: dsl::Wire,
}

pub fn sequence_edge_to_dsl(edge: &SequenceEdge) -> SequenceEdgeDsl {
    let from = dsl::WireNode { id: edge.from.clone(), kind: None, port: None };
    let to = dsl::WireNode { id: edge.to.clone(), kind: None, port: None };
    SequenceEdgeDsl { id: edge.id.clone(), link: dsl::Wire(dsl::WireValue { from, edge: Some((true, to)), edge_label: dsl::WireEdgeLabel::default(), properties: dsl::DslValue::Object(Vec::new()) }) }
}

pub fn sequence_edge_from_dsl(edge: SequenceEdgeDsl) -> Result<SequenceEdge, String> {
    let dsl::WireValue { from, edge: link, .. } = edge.link.0;
    let (directed, to) = link.ok_or_else(|| "sequence edge wire literal must have a target".to_string())?;
    if !directed {
        return Err("sequence edge wire literal must be directed".into());
    }
    Ok(SequenceEdge { id: edge.id, from: from.id, to: to.id })
}

/// 📄️ DSL-only mirror of `SequenceFixture` — `steps`/`edges` print as SoA `#[dsl(table)]` columns
/// instead of the old array-of-structures form, and `edges` goes through `SequenceEdgeDsl` for the
/// unified wire syntax. See this region's opening doc comment on `SequenceEdgeDsl`.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "sequence")]
#[dsl(layout = "lines")]
pub(crate) struct SequenceFixtureDsl {
    schema: String,
    #[dsl(table)]
    steps: Vec<crate::artifacts::sequence::SequenceStep>,
    #[dsl(table)]
    edges: Vec<SequenceEdgeDsl>,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for SequenceFixtureDsl {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for SequenceFixtureDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec


/// 📤️ `pub(crate)` — `🎒️pack`'s `DocumentPack` impl converts through this same mirror (see its doc
/// comment for why the two impls can't be written directly against `SequenceFixture`).
pub(crate) fn sequence_fixture_to_dsl(fixture: &SequenceFixture) -> SequenceFixtureDsl {
    SequenceFixtureDsl { schema: fixture.schema.clone(), steps: fixture.steps.clone(), edges: fixture.edges.iter().map(sequence_edge_to_dsl).collect() }
}

pub(crate) fn sequence_fixture_dsl_to_fixture(fixture: SequenceFixtureDsl) -> Result<SequenceFixture, String> {
    Ok(SequenceFixture { schema: fixture.schema, steps: fixture.steps, edges: fixture.edges.into_iter().map(sequence_edge_from_dsl).collect::<Result<Vec<_>, _>>()? })
}

impl store::DocumentDsl for SequenceFixture {
    const EXTENSION: &'static str = "sequence";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let dsl_fixture = <SequenceFixtureDsl as store::DocumentDsl>::parse_dsl(text)?;
        sequence_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        <SequenceFixtureDsl as store::DocumentDsl>::print_dsl(&sequence_fixture_to_dsl(self))
    }
}
//#endregion 🔖️Dsl

//#region 🔖️Example
/// 📄️ The handcrafted `.sequence` DSL-text fixture (regenerated from `default_fixture()`'s canonical
/// print form) — the permanent proof that the checked-in fixture still parses and round trips, not a
/// one-time migration script.
pub const SEQUENCE_EXAMPLE_TEXT: &str = include_str!("../📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.sequence.sequence.dsl.semio");

/// 📖️ Parses `.sequence` DSL text into a `SequenceFixture`.
pub fn parse_dsl(text: &str) -> Result<SequenceFixture, store::TextError> {
    <SequenceFixture as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `SequenceFixture` back to `.sequence` DSL text.
pub fn print_dsl(fixture: &SequenceFixture) -> String {
    store::DocumentDsl::print_dsl(fixture)
}
//#endregion 🔖️Example

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_fixture, SlotRef, StepParams};
    use neural_engine::{Atom, Dictionary, Value};

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
        fixture.steps.push(crate::artifacts::sequence::SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))), x: 560.0, y: 0.0, slot: None, collapsed: true });
        fixture.steps.push(crate::artifacts::sequence::SequenceStep {
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
        store::test_support::assert_dsl_round_trip(&fixture);
    }
}
//#endregion 🧪️Tests

//! 📦️ Sequence artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::sequence::dsl::{sequence_fixture_dsl_to_fixture, sequence_fixture_to_dsl, SequenceFixtureDsl};
use crate::artifacts::sequence::SequenceFixture;
use store::PackError;

//#region 🔖️Pack
/// 📦️ Hand-written `store::DocumentPack` mirror of the `DocumentDsl` impl in `🗣️dsl` —
/// `SequenceFixture` itself doesn't derive `dsl::DslDocument` (see `SequenceFixtureDsl`'s doc
/// comment there), so it doesn't pick up the blanket derive-emitted `DocumentPack` impl either;
/// this converts through the same `SequenceFixtureDsl` mirror, which does derive it.
impl store::DocumentPack for SequenceFixture {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        <SequenceFixtureDsl as store::DocumentPack>::encode_pack_with(&sequence_fixture_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let dsl_fixture = <SequenceFixtureDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        sequence_fixture_dsl_to_fixture(dsl_fixture).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}

/// 📦️ Encodes a `SequenceFixture` to its binary pack form.
pub fn encode(fixture: &SequenceFixture) -> Vec<u8> {
    store::DocumentPack::encode_pack(fixture)
}

/// 📖️ Decodes a `SequenceFixture` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<SequenceFixture, PackError> {
    <SequenceFixture as store::DocumentPack>::decode_pack(bytes)
}
//#endregion 🔖️Pack

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::dsl;
    use crate::artifacts::sequence::{default_fixture, SequenceStep, SlotRef, StepParams};
    use neural_engine::{Atom, Dictionary, Value};

    #[test]
    fn pack_round_trips_default_fixture() {
        let fixture = default_fixture();
        store::test_support::assert_dsl_pack_equivalence(&fixture);
        let bytes = encode(&fixture);
        assert_eq!(decode(&bytes).expect("decode"), fixture);
    }

    #[test]
    fn default_sequence_example_pack_round_trips() {
        let fixture = dsl::parse_dsl(dsl::SEQUENCE_EXAMPLE_TEXT).expect("🎬️default.sequence must parse");
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    #[test]
    fn pack_round_trips_fixture_with_slots_and_nested_params() {
        let mut fixture = default_fixture();
        fixture.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))), x: 560.0, y: 0.0, slot: None, collapsed: true });
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
        store::test_support::assert_dsl_pack_equivalence(&fixture);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `SequenceOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::sequence::op::SequenceOperation;
        use protocol::{DocumentId, Edit, SchemaId};

        let envelope = store::create_document_envelope::<SequenceFixture, SequenceOperation>(crate::artifacts::sequence::SEQUENCE_FIXTURE_SCHEMA, "sequence-envelope-test", default_fixture(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![SequenceOperation::StepsAdd { index: 2, item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        let edit: &Edit<SequenceOperation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<SequenceFixture, SequenceOperation>(edit, &DocumentId(doc_store.envelope().id.clone()), &SchemaId(doc_store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

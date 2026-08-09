//! 📦️ Sequence artifact — binary document surface + laws (constitutional: pack).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::sequence::SequenceSnapshot;
use store::PackError;

//#region 🔖️Pack
/// 📦️ Encodes a `SequenceSnapshot` to its binary pack form.
pub fn encode(snapshot: &SequenceSnapshot) -> Vec<u8> {
    store::DocumentPack::encode_pack(snapshot)
}

/// 📖️ Decodes a `SequenceSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<SequenceSnapshot, PackError> {
    <SequenceSnapshot as store::DocumentPack>::decode_pack(bytes)
}
//#endregion 🔖️Pack

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::dsl;
    use crate::artifacts::sequence::mutations::SequenceMutation;
    use crate::artifacts::sequence::{default_snapshot, SequenceStep, SlotRef, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
    use neural_engine::{Atom, Dictionary, Value};
    use protocol::{DocumentId, Edit, SchemaId};
    use store::{create_document_envelope, DocumentCommand, DocumentStore};

    #[test]
    fn pack_round_trips_default_snapshot() {
        let snapshot = default_snapshot();
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
        let bytes = encode(&snapshot);
        assert_eq!(decode(&bytes).expect("decode"), snapshot);
    }

    #[test]
    fn default_sequence_example_pack_round_trips() {
        let snapshot = dsl::parse_dsl(dsl::SEQUENCE_EXAMPLE_TEXT).expect("🎬️default.sequence must parse");
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
    }

    #[test]
    fn pack_round_trips_snapshot_with_slots_and_nested_params() {
        let mut snapshot = default_snapshot();
        snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("flag", Value::Atom(Atom::Boolean(true))), x: 560.0, y: 0.0, slot: None, collapsed: true });
        snapshot.steps.push(SequenceStep {
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
        store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
    }

    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        let envelope = create_document_envelope::<SequenceSnapshot, SequenceMutation>(SEQUENCE_DOCUMENT_SCHEMA, "sequence-envelope-test", default_snapshot(), None);
        let mut doc_store = DocumentStore::new(envelope);
        doc_store
            .dispatch(DocumentCommand::Apply {
                mutations: vec![SequenceMutation::StepsAdd {
                    index: 2,
                    item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false },
                }],
                description: None,
            })
            .expect("apply");
        let edit: &Edit<SequenceMutation> = doc_store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<SequenceSnapshot, SequenceMutation>(
            edit,
            &DocumentId(doc_store.envelope().id.clone()),
            &SchemaId(doc_store.envelope().schema.clone()),
        );
    }

    #[test]
    fn pack_protocol_declares_snapshot_segment() {
        assert!(COMPONENT_PROTOCOL_SEMIO.contains("segment Snapshot"));
    }
}
//#endregion 🧪️Tests

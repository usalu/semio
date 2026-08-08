//! ⚖️ Sequence artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! The app's typed `SequenceCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it now lives in `🎛️apps/🎬️sequence/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::sequence::dsl::{sequence_edge_from_dsl, sequence_edge_to_dsl, SequenceEdgeDsl};
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::{SequenceEdgePatch, SequenceStepPatch};
use protocol::OpBinary;

//#region 🔖️OpText
/// ✂️ DSL-only mirror of `SequenceMutation` — identical shape except `EdgesAdd.item` goes through
/// `SequenceEdgeDsl` for the unified wire syntax (see `🗣️dsl`'s doc comment on `SequenceEdgeDsl` for
/// why `EdgesPatch.patch` stays a plain `SequenceEdgePatch`, not a wire).
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum SequenceMutationDsl {
    StepsAdd {
        index: usize,
        #[dsl(block)]
        item: crate::artifacts::sequence::SequenceStep,
    },
    StepsRemove {
        id: String,
    },
    StepsMove {
        id: String,
        to_index: usize,
    },
    StepsPatch {
        id: String,
        #[dsl(block)]
        patch: SequenceStepPatch,
    },
    EdgesAdd {
        index: usize,
        #[dsl(block)]
        item: SequenceEdgeDsl,
    },
    EdgesRemove {
        id: String,
    },
    EdgesMove {
        id: String,
        to_index: usize,
    },
    EdgesPatch {
        id: String,
        #[dsl(block)]
        patch: SequenceEdgePatch,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for SequenceMutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for SequenceMutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn sequence_operation_to_dsl(operation: &SequenceMutation) -> SequenceMutationDsl {
    match operation {
        SequenceMutation::StepsAdd { index, item } => SequenceMutationDsl::StepsAdd { index: *index, item: item.clone() },
        SequenceMutation::StepsRemove { id } => SequenceMutationDsl::StepsRemove { id: id.clone() },
        SequenceMutation::StepsMove { id, to_index } => SequenceMutationDsl::StepsMove { id: id.clone(), to_index: *to_index },
        SequenceMutation::StepsPatch { id, patch } => SequenceMutationDsl::StepsPatch { id: id.clone(), patch: patch.clone() },
        SequenceMutation::EdgesAdd { index, item } => SequenceMutationDsl::EdgesAdd { index: *index, item: sequence_edge_to_dsl(item) },
        SequenceMutation::EdgesRemove { id } => SequenceMutationDsl::EdgesRemove { id: id.clone() },
        SequenceMutation::EdgesMove { id, to_index } => SequenceMutationDsl::EdgesMove { id: id.clone(), to_index: *to_index },
        SequenceMutation::EdgesPatch { id, patch } => SequenceMutationDsl::EdgesPatch { id: id.clone(), patch: patch.clone() },
    }
}

fn sequence_operation_from_dsl(operation: SequenceMutationDsl) -> Result<SequenceMutation, String> {
    Ok(match operation {
        SequenceMutationDsl::StepsAdd { index, item } => SequenceMutation::StepsAdd { index, item },
        SequenceMutationDsl::StepsRemove { id } => SequenceMutation::StepsRemove { id },
        SequenceMutationDsl::StepsMove { id, to_index } => SequenceMutation::StepsMove { id, to_index },
        SequenceMutationDsl::StepsPatch { id, patch } => SequenceMutation::StepsPatch { id, patch },
        SequenceMutationDsl::EdgesAdd { index, item } => SequenceMutation::EdgesAdd { index, item: sequence_edge_from_dsl(item)? },
        SequenceMutationDsl::EdgesRemove { id } => SequenceMutation::EdgesRemove { id },
        SequenceMutationDsl::EdgesMove { id, to_index } => SequenceMutation::EdgesMove { id, to_index },
        SequenceMutationDsl::EdgesPatch { id, patch } => SequenceMutation::EdgesPatch { id, patch },
    })
}

impl protocol::OpText for SequenceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let dsl_operation = <SequenceMutationDsl as protocol::OpText>::parse_op(line)?;
        sequence_operation_from_dsl(dsl_operation).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }

    fn print_op(&self) -> String {
        <SequenceMutationDsl as protocol::OpText>::print_op(&sequence_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` impl above — `SequenceMutationDsl` already derives `OpBinary`
/// via `#[derive(dsl::DslEnum)]`, so this is a pure to/from-dsl forward.
impl OpBinary for SequenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        sequence_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let dsl_operation = SequenceMutationDsl::decode_op(bytes)?;
        sequence_operation_from_dsl(dsl_operation).map_err(|message| protocol::ProtocolError::Malformed { what: "sequence operation", offset: 0, detail: message })
    }
}

/// 📦️ Encodes a `SequenceMutation` to its binary state-patch form.
pub fn encode_op(operation: &SequenceMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `SequenceMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<SequenceMutation, protocol::ProtocolError> {
    SequenceMutation::decode_op(bytes)
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::{default_fixture, SequenceEdge, SequenceFixture, SequenceStep, SlotRef, StepParams};
    use neural_engine::{Atom, Dictionary, Value};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = SequenceMutation::StepsPatch { id: "step-1".into(), patch: SequenceStepPatch { x: Some(42.0), ..Default::default() } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// 🧪️ Whole-store round trip: applies an operation through a real `SequenceStore`, then proves
    /// the resulting envelope survives both the text and binary document-level protocols.
    #[test]
    fn sequence_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<SequenceFixture, SequenceMutation>(crate::artifacts::sequence::SEQUENCE_FIXTURE_SCHEMA, "sequence-text-test", default_fixture(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                mutations: vec![SequenceMutation::StepsAdd { index: 2, item: SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false } }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trips_steps_add() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::StepsAdd {
            index: 2,
            item: SequenceStep { id: "step-99".into(), kind: "log.print".into(), params: StepParams::new().insert("message", Value::Atom(Atom::String("hi there".into()))), x: 5.0, y: -6.5, slot: None, collapsed: false },
        });
    }

    #[test]
    fn op_text_round_trips_steps_add_with_slot() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::StepsAdd {
            index: 0,
            item: SequenceStep { id: "step-98".into(), kind: "control.while".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "body".into() }), collapsed: true },
        });
    }

    #[test]
    fn op_text_round_trips_steps_remove() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::StepsRemove { id: "step-99".into() });
    }

    #[test]
    fn op_text_round_trips_steps_move() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::StepsMove { id: "step-99".into(), to_index: 3 });
    }

    #[test]
    fn op_text_round_trips_steps_patch() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::StepsPatch {
            id: "step-99".into(),
            patch: SequenceStepPatch {
                params: Some(StepParams::new().insert("value", Value::Atom(Atom::Decimal(120.0))).insert("meta", Value::Dictionary(Dictionary::new().insert("k", Value::Atom(Atom::Null))))),
                x: Some(120.0),
                y: None,
                collapsed: Some(true),
            },
        });
    }

    #[test]
    fn op_text_round_trips_steps_patch_with_no_fields() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::StepsPatch { id: "step-99".into(), patch: SequenceStepPatch::default() });
    }

    #[test]
    fn op_text_round_trips_edges_add() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::EdgesAdd { index: 1, item: SequenceEdge { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() } });
    }

    #[test]
    fn op_text_round_trips_edges_remove() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::EdgesRemove { id: "edge-1".into() });
    }

    #[test]
    fn op_text_round_trips_edges_move() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::EdgesMove { id: "edge-1".into(), to_index: 0 });
    }

    #[test]
    fn op_text_round_trips_edges_patch() {
        store::test_support::assert_op_line_round_trip(&SequenceMutation::EdgesPatch { id: "edge-1".into(), patch: SequenceEdgePatch { from: Some("step-3".into()), to: None } });
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests

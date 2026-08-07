//! ⚖️ Procedural2d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::procedural2d::dsl::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,
};
use crate::artifacts::procedural2d::op::Procedural2dOperation;
use playbook::GenerationOperation;
use protocol::OpBinary;

//#region 🔖️OpTextMirror
/// ⚡️ Local twin of `Procedural2dOperation` — flattens the `Generation(GenerationOperation)` newtype
/// variant into its own four top-level keyword variants since a `#[derive(dsl::DslOps)]` enum's
/// variants are each their own tagged record, not a nested enum-in-enum.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Procedural2dOperationDsl {
    SetWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    RemoveWidget {
        id: String,
    },
    SetSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    RemoveSynapse {
        id: String,
    },
    SetLayout {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl,
    },
    RemoveLayout {
        id: String,
    },
    SetCamera {
        #[dsl(block)]
        camera: CameraJsonDsl,
    },
    SetSchema {
        schema: String,
    },
    GenerationAdd {
        #[dsl(block)]
        generation: FormGenerationDsl,
    },
    GenerationRemove {
        id: String,
    },
    GenerationRename {
        id: String,
        name: String,
    },
    GenerationUpdateValues {
        id: String,
        question_id: String,
        value: dsl::DslValue,
    },
}

fn procedural2d_operation_to_dsl(operation: &Procedural2dOperation) -> Procedural2dOperationDsl {
    match operation {
        Procedural2dOperation::SetWidget { index, widget } => Procedural2dOperationDsl::SetWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural2dOperation::RemoveWidget { id } => Procedural2dOperationDsl::RemoveWidget { id: id.clone() },
        Procedural2dOperation::SetSynapse { index, synapse } => Procedural2dOperationDsl::SetSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural2dOperation::RemoveSynapse { id } => Procedural2dOperationDsl::RemoveSynapse { id: id.clone() },
        Procedural2dOperation::SetLayout { id, layout } => Procedural2dOperationDsl::SetLayout { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural2dOperation::RemoveLayout { id } => Procedural2dOperationDsl::RemoveLayout { id: id.clone() },
        Procedural2dOperation::SetCamera { camera } => Procedural2dOperationDsl::SetCamera { camera: camera_to_dsl(camera) },
        Procedural2dOperation::SetSchema { schema } => Procedural2dOperationDsl::SetSchema { schema: schema.clone() },
        Procedural2dOperation::Generation(GenerationOperation::Add { generation }) => Procedural2dOperationDsl::GenerationAdd { generation: form_generation_to_dsl(generation) },
        Procedural2dOperation::Generation(GenerationOperation::Remove { id }) => Procedural2dOperationDsl::GenerationRemove { id: id.clone() },
        Procedural2dOperation::Generation(GenerationOperation::Rename { id, name }) => Procedural2dOperationDsl::GenerationRename { id: id.clone(), name: name.clone() },
        Procedural2dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }) => {
            Procedural2dOperationDsl::GenerationUpdateValues { id: id.clone(), question_id: question_id.clone(), value: dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null) }
        }
    }
}

fn procedural2d_operation_from_dsl(operation: Procedural2dOperationDsl) -> Result<Procedural2dOperation, store::TextError> {
    Ok(match operation {
        Procedural2dOperationDsl::SetWidget { index, widget } => Procedural2dOperation::SetWidget { index, widget: widget_from_dsl(*widget)? },
        Procedural2dOperationDsl::RemoveWidget { id } => Procedural2dOperation::RemoveWidget { id },
        Procedural2dOperationDsl::SetSynapse { index, synapse } => Procedural2dOperation::SetSynapse { index, synapse: synapse_from_dsl(synapse) },
        Procedural2dOperationDsl::RemoveSynapse { id } => Procedural2dOperation::RemoveSynapse { id },
        Procedural2dOperationDsl::SetLayout { id, layout } => Procedural2dOperation::SetLayout { id, layout: layout_from_dsl(&layout) },
        Procedural2dOperationDsl::RemoveLayout { id } => Procedural2dOperation::RemoveLayout { id },
        Procedural2dOperationDsl::SetCamera { camera } => Procedural2dOperation::SetCamera { camera: camera_from_dsl(&camera) },
        Procedural2dOperationDsl::SetSchema { schema } => Procedural2dOperation::SetSchema { schema },
        Procedural2dOperationDsl::GenerationAdd { generation } => Procedural2dOperation::Generation(GenerationOperation::Add { generation: form_generation_from_dsl(generation) }),
        Procedural2dOperationDsl::GenerationRemove { id } => Procedural2dOperation::Generation(GenerationOperation::Remove { id }),
        Procedural2dOperationDsl::GenerationRename { id, name } => Procedural2dOperation::Generation(GenerationOperation::Rename { id, name }),
        Procedural2dOperationDsl::GenerationUpdateValues { id, question_id, value } => Procedural2dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value: dsl::from_dsl_value(value).unwrap_or(serde_json::Value::Null) }),
    })
}

/// ⚡️ `Procedural2dOperation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural2dOperationDsl` (see above); `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Procedural2dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural2dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural2d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural2dOperationDsl as protocol::OpText>::print_op(&procedural2d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Procedural2dOperationDsl` already derives
/// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
impl OpBinary for Procedural2dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural2d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural2dOperationDsl::decode_op(bytes)?;
        procedural2d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "procedural2d operation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Procedural2dOperation` to its binary state-patch form.
pub fn encode_op(operation: &Procedural2dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural2dOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural2dOperation, protocol::ProtocolError> {
    Procedural2dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural2d::{Procedural2dDocument, PROCEDURAL_2D_SCHEMA};
    use flow_core::{CameraJson, SynapseSpec, Widget, WidgetLayout};
    use protocol::OpText;
    use store::{create_document_envelope, test_support, DocumentCommand};

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSynapse { index: 1, synapse: SynapseSpec { id: "s1".into(), from: "rect".into(), to: "fill".into(), from_port: "draw.drawing".into(), to_port: String::new() } });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveSynapse { id: "s1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetLayout { id: "rect".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::RemoveLayout { id: "rect".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural2dOperation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural2dOperation::Generation(GenerationOperation::Add { generation }));
    }
    //#endregion 🔖️OpTextTests

    //#region 🔖️OpTextErrorTests
    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = Procedural2dOperation::parse_op("bogus-op id=\"x\"").unwrap_err();
        assert!(error.message.contains("unknown operation"), "unexpected error: {}", error.message);
    }

    #[test]
    fn op_text_parse_rejects_non_integer_index() {
        let error = Procedural2dOperation::parse_op("set-widget index=abc note text=\"\" id=\"x\"").unwrap_err();
        assert!(error.message.contains("expected Int"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖️OpTextErrorTests

    #[test]
    fn op_binary_round_trips_via_wrapper_fns() {
        let operation = Procedural2dOperation::SetSchema { schema: "flow.fixture".into() };
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = store::DocumentStore::<Procedural2dDocument, Procedural2dOperation>::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", Procedural2dDocument::default(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural2dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests

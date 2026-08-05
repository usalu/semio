//! ⚖️ Procedural3d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

use crate::artifacts::procedural3d::dsl::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,
};
use crate::artifacts::procedural3d::op::Procedural3dOperation;
use playbook::GenerationOperation;
use protocol::OpBinary;

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum Procedural3dOperationDsl {
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

fn procedural3d_operation_to_dsl(operation: &Procedural3dOperation) -> Procedural3dOperationDsl {
    match operation {
        Procedural3dOperation::SetWidget { index, widget } => Procedural3dOperationDsl::SetWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural3dOperation::RemoveWidget { id } => Procedural3dOperationDsl::RemoveWidget { id: id.clone() },
        Procedural3dOperation::SetSynapse { index, synapse } => Procedural3dOperationDsl::SetSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural3dOperation::RemoveSynapse { id } => Procedural3dOperationDsl::RemoveSynapse { id: id.clone() },
        Procedural3dOperation::SetLayout { id, layout } => Procedural3dOperationDsl::SetLayout { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural3dOperation::RemoveLayout { id } => Procedural3dOperationDsl::RemoveLayout { id: id.clone() },
        Procedural3dOperation::SetCamera { camera } => Procedural3dOperationDsl::SetCamera { camera: camera_to_dsl(camera) },
        Procedural3dOperation::SetSchema { schema } => Procedural3dOperationDsl::SetSchema { schema: schema.clone() },
        Procedural3dOperation::Generation(GenerationOperation::Add { generation }) => Procedural3dOperationDsl::GenerationAdd { generation: form_generation_to_dsl(generation) },
        Procedural3dOperation::Generation(GenerationOperation::Remove { id }) => Procedural3dOperationDsl::GenerationRemove { id: id.clone() },
        Procedural3dOperation::Generation(GenerationOperation::Rename { id, name }) => Procedural3dOperationDsl::GenerationRename { id: id.clone(), name: name.clone() },
        Procedural3dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value }) => {
            Procedural3dOperationDsl::GenerationUpdateValues { id: id.clone(), question_id: question_id.clone(), value: dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null) }
        }
    }
}

fn procedural3d_operation_from_dsl(operation: Procedural3dOperationDsl) -> Result<Procedural3dOperation, store::TextError> {
    Ok(match operation {
        Procedural3dOperationDsl::SetWidget { index, widget } => Procedural3dOperation::SetWidget { index, widget: widget_from_dsl(*widget)? },
        Procedural3dOperationDsl::RemoveWidget { id } => Procedural3dOperation::RemoveWidget { id },
        Procedural3dOperationDsl::SetSynapse { index, synapse } => Procedural3dOperation::SetSynapse { index, synapse: synapse_from_dsl(synapse) },
        Procedural3dOperationDsl::RemoveSynapse { id } => Procedural3dOperation::RemoveSynapse { id },
        Procedural3dOperationDsl::SetLayout { id, layout } => Procedural3dOperation::SetLayout { id, layout: layout_from_dsl(layout) },
        Procedural3dOperationDsl::RemoveLayout { id } => Procedural3dOperation::RemoveLayout { id },
        Procedural3dOperationDsl::SetCamera { camera } => Procedural3dOperation::SetCamera { camera: camera_from_dsl(camera) },
        Procedural3dOperationDsl::SetSchema { schema } => Procedural3dOperation::SetSchema { schema },
        Procedural3dOperationDsl::GenerationAdd { generation } => Procedural3dOperation::Generation(GenerationOperation::Add { generation: form_generation_from_dsl(generation) }),
        Procedural3dOperationDsl::GenerationRemove { id } => Procedural3dOperation::Generation(GenerationOperation::Remove { id }),
        Procedural3dOperationDsl::GenerationRename { id, name } => Procedural3dOperation::Generation(GenerationOperation::Rename { id, name }),
        Procedural3dOperationDsl::GenerationUpdateValues { id, question_id, value } => Procedural3dOperation::Generation(GenerationOperation::UpdateValues { id, question_id, value: dsl::from_dsl_value(value).unwrap_or(serde_json::Value::Null) }),
    })
}

impl protocol::OpText for Procedural3dOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural3dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural3d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural3dOperationDsl as protocol::OpText>::print_op(&procedural3d_operation_to_dsl(self))
    }
}

impl protocol::OpBinary for Procedural3dOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural3d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural3dOperationDsl::decode_op(bytes)?;
        procedural3d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "procedural3d operation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Procedural3dOperation` to its binary state-patch form.
pub fn encode_op(operation: &Procedural3dOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural3dOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural3dOperation, protocol::ProtocolError> {
    Procedural3dOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::{Procedural3dDocument, PROCEDURAL_3D_SCHEMA};
    use flow_core::{CameraJson, SynapseSpec, Widget, WidgetLayout};
    use playbook::GenerationOperation;
    use store::{create_document_envelope, test_support, DocumentCommand};

    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetSynapse { index: 1, synapse: SynapseSpec { id: "e1".into(), from: "height".into(), to: "extrude".into(), from_port: "number".into(), to_port: String::new() } });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveSynapse { id: "e1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::RemoveLayout { id: "extrude".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural3dOperation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural3dOperation::Generation(GenerationOperation::Add { generation }));
    }

    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = <Procedural3dOperation as protocol::OpText>::parse_op("bogus-op id=\"w-1\"").expect_err("unknown operation must fail to parse");
        assert!(error.to_string().contains("unknown operation"), "unexpected error: {error}");
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = store::DocumentStore::<Procedural3dDocument, Procedural3dOperation>::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", Procedural3dDocument::default(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![Procedural3dOperation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests

//! ⚖️ Procedural3d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::procedural3d::dsl::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl};
use crate::artifacts::procedural3d::schema::mutations::text::Procedural3dMutation;
use flow::playbook::GenerationMutation;
use protocol::OpBinary;

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Procedural3dOperationDsl {
    SetWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>},
    RemoveWidget {
        id: String},
    SetSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl},
    RemoveSynapse {
        id: String},
    SetLayout {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl},
    RemoveLayout {
        id: String},
    SetCamera {
        #[dsl(block)]
        camera: CameraJsonDsl},
    SetSchema {
        schema: String},
    GenerationAdd {
        #[dsl(block)]
        generation: FormGenerationDsl},
    GenerationRemove {
        id: String},
    GenerationRename {
        id: String,
        name: String},
    GenerationUpdateValues {
        id: String,
        question_id: String,
        value: dsl::DslValue}}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Procedural3dOperationDsl {
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
        Err(dsl::__rt::field_error(format!("unknown operation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Procedural3dOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn procedural3d_operation_to_dsl(operation: &Procedural3dMutation) -> Procedural3dOperationDsl {
    match operation {
        Procedural3dMutation::SetWidget { index, widget } => Procedural3dOperationDsl::SetWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural3dMutation::RemoveWidget { id } => Procedural3dOperationDsl::RemoveWidget { id: id.clone() },
        Procedural3dMutation::SetSynapse { index, synapse } => Procedural3dOperationDsl::SetSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural3dMutation::RemoveSynapse { id } => Procedural3dOperationDsl::RemoveSynapse { id: id.clone() },
        Procedural3dMutation::SetLayout { id, layout } => Procedural3dOperationDsl::SetLayout { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural3dMutation::RemoveLayout { id } => Procedural3dOperationDsl::RemoveLayout { id: id.clone() },
        Procedural3dMutation::SetCamera { camera } => Procedural3dOperationDsl::SetCamera { camera: camera_to_dsl(camera) },
        Procedural3dMutation::SetSchema { schema } => Procedural3dOperationDsl::SetSchema { schema: schema.clone() },
        Procedural3dMutation::Generation(GenerationMutation::Add { generation }) => Procedural3dOperationDsl::GenerationAdd { generation: form_generation_to_dsl(generation) },
        Procedural3dMutation::Generation(GenerationMutation::Remove { id }) => Procedural3dOperationDsl::GenerationRemove { id: id.clone() },
        Procedural3dMutation::Generation(GenerationMutation::Rename { id, name }) => Procedural3dOperationDsl::GenerationRename { id: id.clone(), name: name.clone() },
        Procedural3dMutation::Generation(GenerationMutation::UpdateValues { id, question_id, value }) => {
            Procedural3dOperationDsl::GenerationUpdateValues { id: id.clone(), question_id: question_id.clone(), value: dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null) }
        }
    }
}

fn procedural3d_operation_from_dsl(operation: Procedural3dOperationDsl) -> Result<Procedural3dMutation, store::TextError> {
    Ok(match operation {
        Procedural3dOperationDsl::SetWidget { index, widget } => Procedural3dMutation::SetWidget { index, widget: widget_from_dsl(*widget)? },
        Procedural3dOperationDsl::RemoveWidget { id } => Procedural3dMutation::RemoveWidget { id },
        Procedural3dOperationDsl::SetSynapse { index, synapse } => Procedural3dMutation::SetSynapse { index, synapse: synapse_from_dsl(synapse) },
        Procedural3dOperationDsl::RemoveSynapse { id } => Procedural3dMutation::RemoveSynapse { id },
        Procedural3dOperationDsl::SetLayout { id, layout } => Procedural3dMutation::SetLayout { id, layout: layout_from_dsl(&layout) },
        Procedural3dOperationDsl::RemoveLayout { id } => Procedural3dMutation::RemoveLayout { id },
        Procedural3dOperationDsl::SetCamera { camera } => Procedural3dMutation::SetCamera { camera: camera_from_dsl(&camera) },
        Procedural3dOperationDsl::SetSchema { schema } => Procedural3dMutation::SetSchema { schema },
        Procedural3dOperationDsl::GenerationAdd { generation } => Procedural3dMutation::Generation(GenerationMutation::Add { generation: form_generation_from_dsl(generation) }),
        Procedural3dOperationDsl::GenerationRemove { id } => Procedural3dMutation::Generation(GenerationMutation::Remove { id }),
        Procedural3dOperationDsl::GenerationRename { id, name } => Procedural3dMutation::Generation(GenerationMutation::Rename { id, name }),
        Procedural3dOperationDsl::GenerationUpdateValues { id, question_id, value } => Procedural3dMutation::Generation(GenerationMutation::UpdateValues { id, question_id, value: dsl::from_dsl_value(value).unwrap_or(serde_json::Value::Null) })})
}

/// ⚡️ `Procedural3dMutation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural3dOperationDsl`; `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Procedural3dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural3dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural3d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural3dOperationDsl as protocol::OpText>::print_op(&procedural3d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above.
impl OpBinary for Procedural3dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural3d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural3dOperationDsl::decode_op(bytes)?;
        procedural3d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed {
            what: "procedural3d mutation",
            offset: 0,
            detail: error.to_string()})
    }
}
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Procedural3dMutation` to its binary state-patch form.
pub fn encode_op(operation: &Procedural3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural3dMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural3dMutation, protocol::ProtocolError> {
    Procedural3dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::{Procedural3dSnapshot, PROCEDURAL_3D_SCHEMA};
    use flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
    use flow::playbook::GenerationMutation;
    use semio_framework_os_kernel::os_store::test_support;
    use store::{create_document_envelope, ArtifactCommand};

    #[test]
    fn op_text_round_trip_set_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::SetWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } });
    }

    #[test]
    fn op_text_round_trip_remove_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::RemoveWidget { id: "note-9".into() });
    }

    #[test]
    fn op_text_round_trip_set_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::SetSynapse { index: 1, synapse: SynapseSpec { id: "e1".into(), from: "height".into(), to: "extrude".into(), from_port: "number".into(), to_port: String::new() } });
    }

    #[test]
    fn op_text_round_trip_remove_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::RemoveSynapse { id: "e1".into() });
    }

    #[test]
    fn op_text_round_trip_set_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::SetLayout { id: "extrude".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } });
    }

    #[test]
    fn op_text_round_trip_remove_layout() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::RemoveLayout { id: "extrude".into() });
    }

    #[test]
    fn op_text_round_trip_set_camera() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::SetCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } });
    }

    #[test]
    fn op_text_round_trip_set_schema() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::SetSchema { schema: "flow.fixture".into() });
    }

    #[test]
    fn op_text_round_trip_generation() {
        let generation = flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural3dMutation::Generation(GenerationMutation::Add { generation }));
    }

    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = <Procedural3dMutation as protocol::OpText>::parse_op("bogus-op id=\"w-1\"").expect_err("unknown operation must fail to parse");
        assert!(error.to_string().contains("unknown operation"), "unexpected error: {error}");
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = store::ArtifactStore::<Procedural3dSnapshot, Procedural3dMutation>::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", Procedural3dSnapshot::default(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Procedural3dMutation::SetWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } }], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests

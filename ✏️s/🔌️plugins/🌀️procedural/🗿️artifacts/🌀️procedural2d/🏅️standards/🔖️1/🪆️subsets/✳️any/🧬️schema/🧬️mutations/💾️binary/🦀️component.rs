//! ⚖️ Procedural2d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::procedural2d::dsl::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl};
use crate::artifacts::procedural2d::schema::mutations::text::Procedural2dMutation;
use protocol::OpBinary;

//#region 🔖️OpTextMirror
/// ⚡️ Local twin of `Procedural2dMutation` — one flattened, `#[derive(dsl::DslEnum)]`-friendly
/// keyword variant per semantic mutation (each payload struct embeds a foreign `flow` type —
/// `Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson`/`FormGeneration` — that can't itself derive
/// `dsl::DslRecord`, so this mirror + the existing `*_to_dsl`/`*_from_dsl` bridge functions do the
/// wire conversion instead of deriving the codec straight off the payload structs).
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Procedural2dOperationDsl {
    CreateWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>},
    ReplaceWidget {
        #[dsl(statements)]
        widget: Box<WidgetDsl>},
    DeleteWidget {
        id: String},
    ConnectSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl},
    ReplaceSynapse {
        #[dsl(block)]
        synapse: SynapseSpecDsl},
    DisconnectSynapse {
        id: String},
    MoveWidget {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl},
    ClearWidgetLayout {
        id: String},
    UpdateCamera {
        #[dsl(block)]
        camera: CameraJsonDsl},
    ChangeSchema {
        schema: String},
    CreateGeneration {
        #[dsl(block)]
        generation: FormGenerationDsl},
    DeleteGeneration {
        id: String},
    RenameGeneration {
        id: String,
        name: String},
    ChangeGenerationValue {
        id: String,
        question_id: String,
        value: dsl::DslValue}}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Procedural2dOperationDsl {
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

impl protocol::OpBinary for Procedural2dOperationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




fn procedural2d_operation_to_dsl(operation: &Procedural2dMutation) -> Procedural2dOperationDsl {
    match operation {
        Procedural2dMutation::CreateWidget(payload) => Procedural2dOperationDsl::CreateWidget { index: payload.index, widget: Box::new(widget_to_dsl(&payload.widget)) },
        Procedural2dMutation::ReplaceWidget(payload) => Procedural2dOperationDsl::ReplaceWidget { widget: Box::new(widget_to_dsl(&payload.widget)) },
        Procedural2dMutation::DeleteWidget(payload) => Procedural2dOperationDsl::DeleteWidget { id: payload.id.clone() },
        Procedural2dMutation::ConnectSynapse(payload) => Procedural2dOperationDsl::ConnectSynapse { index: payload.index, synapse: synapse_to_dsl(&payload.synapse) },
        Procedural2dMutation::ReplaceSynapse(payload) => Procedural2dOperationDsl::ReplaceSynapse { synapse: synapse_to_dsl(&payload.synapse) },
        Procedural2dMutation::DisconnectSynapse(payload) => Procedural2dOperationDsl::DisconnectSynapse { id: payload.id.clone() },
        Procedural2dMutation::MoveWidget(payload) => Procedural2dOperationDsl::MoveWidget { id: payload.id.clone(), layout: layout_to_dsl(&payload.layout) },
        Procedural2dMutation::ClearWidgetLayout(payload) => Procedural2dOperationDsl::ClearWidgetLayout { id: payload.id.clone() },
        Procedural2dMutation::UpdateCamera(payload) => Procedural2dOperationDsl::UpdateCamera { camera: camera_to_dsl(&payload.camera) },
        Procedural2dMutation::ChangeSchema(payload) => Procedural2dOperationDsl::ChangeSchema { schema: payload.schema.clone() },
        Procedural2dMutation::CreateGeneration(payload) => Procedural2dOperationDsl::CreateGeneration { generation: form_generation_to_dsl(&payload.generation) },
        Procedural2dMutation::DeleteGeneration(payload) => Procedural2dOperationDsl::DeleteGeneration { id: payload.id.clone() },
        Procedural2dMutation::RenameGeneration(payload) => Procedural2dOperationDsl::RenameGeneration { id: payload.id.clone(), name: payload.name.clone() },
        Procedural2dMutation::ChangeGenerationValue(payload) => {
            Procedural2dOperationDsl::ChangeGenerationValue { id: payload.id.clone(), question_id: payload.question_id.clone(), value: dsl::to_dsl_value(&payload.value).unwrap_or(dsl::DslValue::Null) }
        }
    }
}

fn procedural2d_operation_from_dsl(operation: Procedural2dOperationDsl) -> Result<Procedural2dMutation, store::TextError> {
    use crate::artifacts::procedural2d::mutations::{
        change_generation_value, change_schema, clear_widget_layout, connect_synapse, create_generation, create_widget, delete_generation, delete_widget, disconnect_synapse, move_widget,
        rename_generation, replace_synapse, replace_widget, update_camera};
    Ok(match operation {
        Procedural2dOperationDsl::CreateWidget { index, widget } => create_widget(index, widget_from_dsl(*widget)?),
        Procedural2dOperationDsl::ReplaceWidget { widget } => replace_widget(widget_from_dsl(*widget)?),
        Procedural2dOperationDsl::DeleteWidget { id } => delete_widget(id),
        Procedural2dOperationDsl::ConnectSynapse { index, synapse } => connect_synapse(index, synapse_from_dsl(synapse)),
        Procedural2dOperationDsl::ReplaceSynapse { synapse } => replace_synapse(synapse_from_dsl(synapse)),
        Procedural2dOperationDsl::DisconnectSynapse { id } => disconnect_synapse(id),
        Procedural2dOperationDsl::MoveWidget { id, layout } => move_widget(id, layout_from_dsl(&layout)),
        Procedural2dOperationDsl::ClearWidgetLayout { id } => clear_widget_layout(id),
        Procedural2dOperationDsl::UpdateCamera { camera } => update_camera(camera_from_dsl(&camera)),
        Procedural2dOperationDsl::ChangeSchema { schema } => change_schema(schema),
        Procedural2dOperationDsl::CreateGeneration { generation } => create_generation(form_generation_from_dsl(generation)),
        Procedural2dOperationDsl::DeleteGeneration { id } => delete_generation(id),
        Procedural2dOperationDsl::RenameGeneration { id, name } => rename_generation(id, name),
        Procedural2dOperationDsl::ChangeGenerationValue { id, question_id, value } => change_generation_value(id, question_id, dsl::from_dsl_value(value).unwrap_or(serde_json::Value::Null))})
}

/// ⚡️ `Procedural2dMutation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural2dOperationDsl` (see above); `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Procedural2dMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural2dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural2d_operation_from_dsl(parsed)
    }

    fn print_op(&self) -> String {
        <Procedural2dOperationDsl as protocol::OpText>::print_op(&procedural2d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above — `Procedural2dOperationDsl` already implements
/// `OpBinary`, so this is a pure to/from-dsl forward.
impl OpBinary for Procedural2dMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural2d_operation_to_dsl(self).encode_op()
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural2dOperationDsl::decode_op(bytes)?;
        procedural2d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed {
            what: "procedural2d mutation",
            offset: 0,
            detail: error.to_string()})
    }
}
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Procedural2dMutation` to its binary state-patch form.
pub fn encode_op(operation: &Procedural2dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural2dMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<Procedural2dMutation, protocol::ProtocolError> {
    Procedural2dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural2d::mutations::{change_schema, connect_synapse, create_generation, create_widget, delete_widget};
    use crate::artifacts::procedural2d::{Procedural2dSnapshot, PROCEDURAL_2D_SCHEMA};
    use flow::{SynapseSpec, Widget};
    use protocol::OpText;
    use semio_framework_os_kernel::os_store::test_support;
    use store::{create_document_envelope, ArtifactCommand};

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_create_widget() {
        test_support::assert_op_line_round_trip(&create_widget(2, Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() }));
    }

    #[test]
    fn op_text_round_trip_delete_widget() {
        test_support::assert_op_line_round_trip(&delete_widget("note-9".into()));
    }

    #[test]
    fn op_text_round_trip_connect_synapse() {
        test_support::assert_op_line_round_trip(&connect_synapse(1, SynapseSpec { id: "s1".into(), from: "rect".into(), to: "fill".into(), from_port: "draw.drawing".into(), to_port: String::new() }));
    }

    #[test]
    fn op_text_round_trip_change_schema() {
        test_support::assert_op_line_round_trip(&change_schema("flow.fixture".into()));
    }

    #[test]
    fn op_text_round_trip_create_generation() {
        let generation = flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&create_generation(generation));
    }
    //#endregion 🔖️OpTextTests

    //#region 🔖️OpTextErrorTests
    #[test]
    fn op_text_parse_rejects_unknown_operation() {
        let error = Procedural2dMutation::parse_op("bogus-op id=\"x\"").unwrap_err();
        assert!(error.message.contains("unknown operation"), "unexpected error: {}", error.message);
    }

    #[test]
    fn op_text_parse_rejects_non_integer_index() {
        let error = Procedural2dMutation::parse_op("create-widget index=abc note text=\"\" id=\"x\"").unwrap_err();
        assert!(error.message.contains("expected Int"), "unexpected error: {}", error.message);
    }
    //#endregion 🔖️OpTextErrorTests

    #[test]
    fn op_binary_round_trips_via_wrapper_fns() {
        let operation = change_schema("flow.fixture".into());
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = store::ArtifactStore::<Procedural2dSnapshot, Procedural2dMutation>::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", Procedural2dSnapshot::default(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![create_widget(3, Widget::InputNote { id: "note-9".into(), text: String::new() })], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests

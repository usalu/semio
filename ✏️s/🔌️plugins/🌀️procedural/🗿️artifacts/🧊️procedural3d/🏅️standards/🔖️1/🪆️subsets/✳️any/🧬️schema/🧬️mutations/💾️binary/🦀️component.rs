//! ⚖️ Procedural3d artifact — state-patch-representation wire codec + laws (was: constitutional
//! `protocol`; no `📡️protocol` path segment may survive under plugins).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::procedural3d::dsl::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl,
    WidgetLayoutDsl,
};
use crate::artifacts::procedural3d::mutations::change_generation_value::mutation::ChangeGenerationValue;
use crate::artifacts::procedural3d::mutations::change_schema::mutation::ChangeSchema;
use crate::artifacts::procedural3d::mutations::connect_synapse::mutation::ConnectSynapse;
use crate::artifacts::procedural3d::mutations::create_generation::mutation::CreateGeneration;
use crate::artifacts::procedural3d::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::procedural3d::mutations::delete_generation::mutation::DeleteGeneration;
use crate::artifacts::procedural3d::mutations::delete_widget::mutation::DeleteWidget;
use crate::artifacts::procedural3d::mutations::delete_widget_position::mutation::DeleteWidgetPosition;
use crate::artifacts::procedural3d::mutations::disconnect_synapse::mutation::DisconnectSynapse;
use crate::artifacts::procedural3d::mutations::move_widget::mutation::MoveWidget;
use crate::artifacts::procedural3d::mutations::rename_generation::mutation::RenameGeneration;
use crate::artifacts::procedural3d::mutations::update_camera::mutation::UpdateCamera;
use crate::artifacts::procedural3d::mutations::update_synapse::mutation::UpdateSynapse;
use crate::artifacts::procedural3d::mutations::update_widget::mutation::UpdateWidget;
use crate::artifacts::procedural3d::schema::mutations::text::Procedural3dMutation;
use protocol::OpBinary;

//#region 🔖️OpTextMirror
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum Procedural3dOperationDsl {
    CreateWidget {
        index: usize,
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    UpdateWidget {
        #[dsl(statements)]
        widget: Box<WidgetDsl>,
    },
    DeleteWidget {
        id: String,
    },
    ConnectSynapse {
        index: usize,
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    UpdateSynapse {
        #[dsl(block)]
        synapse: SynapseSpecDsl,
    },
    DisconnectSynapse {
        id: String,
    },
    MoveWidget {
        id: String,
        #[dsl(block)]
        layout: WidgetLayoutDsl,
    },
    DeleteWidgetPosition {
        id: String,
    },
    UpdateCamera {
        #[dsl(block)]
        camera: CameraJsonDsl,
    },
    ChangeSchema {
        new_schema: String,
    },
    CreateGeneration {
        #[dsl(block)]
        generation: FormGenerationDsl,
    },
    DeleteGeneration {
        id: String,
    },
    RenameGeneration {
        id: String,
        new_name: String,
    },
    ChangeGenerationValue {
        id: String,
        question_id: String,
        new_value: dsl::DslValue,
    },
}
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Procedural3dOperationDsl {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl OpBinary for Procedural3dOperationDsl {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs

async fn procedural3d_operation_to_dsl(operation: &Procedural3dMutation) -> Procedural3dOperationDsl {
    match operation {
        Procedural3dMutation::CreateWidget(CreateWidget { index, widget }) => Procedural3dOperationDsl::CreateWidget { index: *index, widget: Box::new(widget_to_dsl(widget)) },
        Procedural3dMutation::UpdateWidget(UpdateWidget { widget }) => Procedural3dOperationDsl::UpdateWidget { widget: Box::new(widget_to_dsl(widget)) },
        Procedural3dMutation::DeleteWidget(DeleteWidget { id }) => Procedural3dOperationDsl::DeleteWidget { id: id.clone() },
        Procedural3dMutation::ConnectSynapse(ConnectSynapse { index, synapse }) => Procedural3dOperationDsl::ConnectSynapse { index: *index, synapse: synapse_to_dsl(synapse) },
        Procedural3dMutation::UpdateSynapse(UpdateSynapse { synapse }) => Procedural3dOperationDsl::UpdateSynapse { synapse: synapse_to_dsl(synapse) },
        Procedural3dMutation::DisconnectSynapse(DisconnectSynapse { id }) => Procedural3dOperationDsl::DisconnectSynapse { id: id.clone() },
        Procedural3dMutation::MoveWidget(MoveWidget { id, layout }) => Procedural3dOperationDsl::MoveWidget { id: id.clone(), layout: layout_to_dsl(layout) },
        Procedural3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id }) => Procedural3dOperationDsl::DeleteWidgetPosition { id: id.clone() },
        Procedural3dMutation::UpdateCamera(UpdateCamera { camera }) => Procedural3dOperationDsl::UpdateCamera { camera: camera_to_dsl(camera) },
        Procedural3dMutation::ChangeSchema(ChangeSchema { new_schema }) => Procedural3dOperationDsl::ChangeSchema { new_schema: new_schema.clone() },
        Procedural3dMutation::CreateGeneration(CreateGeneration { generation }) => Procedural3dOperationDsl::CreateGeneration { generation: form_generation_to_dsl(generation) },
        Procedural3dMutation::DeleteGeneration(DeleteGeneration { id }) => Procedural3dOperationDsl::DeleteGeneration { id: id.clone() },
        Procedural3dMutation::RenameGeneration(RenameGeneration { id, new_name }) => Procedural3dOperationDsl::RenameGeneration { id: id.clone(), new_name: new_name.clone() },
        Procedural3dMutation::ChangeGenerationValue(ChangeGenerationValue { id, question_id, new_value }) => {
            Procedural3dOperationDsl::ChangeGenerationValue { id: id.clone(), question_id: question_id.clone(), new_value: dsl::to_dsl_value(new_value).unwrap_or(dsl::DslValue::Null) }
        }
    }
}

async fn procedural3d_operation_from_dsl(operation: Procedural3dOperationDsl) -> Result<Procedural3dMutation, store::TextError> {
    Ok(match operation {
        Procedural3dOperationDsl::CreateWidget { index, widget } => Procedural3dMutation::CreateWidget(CreateWidget { index, widget: widget_from_dsl(*widget)? }),
        Procedural3dOperationDsl::UpdateWidget { widget } => Procedural3dMutation::UpdateWidget(UpdateWidget { widget: widget_from_dsl(*widget)? }),
        Procedural3dOperationDsl::DeleteWidget { id } => Procedural3dMutation::DeleteWidget(DeleteWidget { id }),
        Procedural3dOperationDsl::ConnectSynapse { index, synapse } => Procedural3dMutation::ConnectSynapse(ConnectSynapse { index, synapse: synapse_from_dsl(synapse) }),
        Procedural3dOperationDsl::UpdateSynapse { synapse } => Procedural3dMutation::UpdateSynapse(UpdateSynapse { synapse: synapse_from_dsl(synapse) }),
        Procedural3dOperationDsl::DisconnectSynapse { id } => Procedural3dMutation::DisconnectSynapse(DisconnectSynapse { id }),
        Procedural3dOperationDsl::MoveWidget { id, layout } => Procedural3dMutation::MoveWidget(MoveWidget { id, layout: layout_from_dsl(&layout) }),
        Procedural3dOperationDsl::DeleteWidgetPosition { id } => Procedural3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id }),
        Procedural3dOperationDsl::UpdateCamera { camera } => Procedural3dMutation::UpdateCamera(UpdateCamera { camera: camera_from_dsl(&camera) }),
        Procedural3dOperationDsl::ChangeSchema { new_schema } => Procedural3dMutation::ChangeSchema(ChangeSchema { new_schema }),
        Procedural3dOperationDsl::CreateGeneration { generation } => Procedural3dMutation::CreateGeneration(CreateGeneration { generation: form_generation_from_dsl(generation) }),
        Procedural3dOperationDsl::DeleteGeneration { id } => Procedural3dMutation::DeleteGeneration(DeleteGeneration { id }),
        Procedural3dOperationDsl::RenameGeneration { id, new_name } => Procedural3dMutation::RenameGeneration(RenameGeneration { id, new_name }),
        Procedural3dOperationDsl::ChangeGenerationValue { id, question_id, new_value } => {
            Procedural3dMutation::ChangeGenerationValue(ChangeGenerationValue { id, question_id, new_value: dsl::from_dsl_value(new_value).unwrap_or(serde_json::Value::Null) })
        }
    })
}

/// ⚡️ `Procedural3dMutation`'s compact single-line op encoding — derive-engine grammar via
/// `Procedural3dOperationDsl`; `parse_op`/`print_op` convert at the boundary.
impl protocol::OpText for Procedural3dMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let parsed = <Procedural3dOperationDsl as protocol::OpText>::parse_op(line)?;
        procedural3d_operation_from_dsl(parsed)
    }

    async fn print_op(&self) -> String {
        <Procedural3dOperationDsl as protocol::OpText>::print_op(&procedural3d_operation_to_dsl(self))
    }
}

/// ⚡️ Binary mirror of the `OpText` bridge above.
impl OpBinary for Procedural3dMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        procedural3d_operation_to_dsl(self).encode_op()
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let parsed = Procedural3dOperationDsl::decode_op(bytes)?;
        procedural3d_operation_from_dsl(parsed).map_err(|error| protocol::ProtocolError::Malformed { what: "procedural3d mutation", offset: 0, detail: error.to_string() })
    }
}
//#endregion 🔖️OpTextMirror

/// 📦️ Encodes a `Procedural3dMutation` to its binary state-patch form.
pub async fn encode_op(operation: &Procedural3dMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `Procedural3dMutation` from its binary state-patch form.
pub async fn decode_op(bytes: &[u8]) -> Result<Procedural3dMutation, protocol::ProtocolError> {
    Procedural3dMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::procedural3d::{Procedural3dSnapshot, PROCEDURAL_3D_SCHEMA};
    use flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
    use semio_framework_os_kernel::os_store::test_support;
    use store::{create_document_envelope, ArtifactCommand};

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_create_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::CreateWidget(CreateWidget { index: 2, widget: Widget::InputNote { id: "note-9".into(), text: "hello \"world\"".into() } }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_delete_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::DeleteWidget(DeleteWidget { id: "note-9".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_connect_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::ConnectSynapse(ConnectSynapse {
            index: 1,
            synapse: SynapseSpec { id: "e1".into(), from: "height".into(), to: "extrude".into(), from_port: "number".into(), to_port: String::new() },
        }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_disconnect_synapse() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::DisconnectSynapse(DisconnectSynapse { id: "e1".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_move_widget() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::MoveWidget(MoveWidget { id: "extrude".into(), layout: WidgetLayout { x: 12.5, y: -8.25 } }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_delete_widget_position() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::DeleteWidgetPosition(DeleteWidgetPosition { id: "extrude".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_update_camera() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::UpdateCamera(UpdateCamera { camera: CameraJson { x: 1.5, y: -2.5, zoom: 1.2 } }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_change_schema() {
        test_support::assert_op_line_round_trip(&Procedural3dMutation::ChangeSchema(ChangeSchema { new_schema: "flow.fixture".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_round_trip_create_generation() {
        let generation = flow::playbook::FormGeneration { id: "generation-1".into(), name: "Generation 1".into(), values: serde_json::Map::new() };
        test_support::assert_op_line_round_trip(&Procedural3dMutation::CreateGeneration(CreateGeneration { generation }));
    }

    #[semio_framework_async_macros::async_test]
    async fn op_text_parse_rejects_unknown_operation() {
        let error = <Procedural3dMutation as protocol::OpText>::parse_op("bogus-op id=\"w-1\"").expect_err("unknown operation must fail to parse");
        assert!(error.to_string().contains("unknown operation"), "unexpected error: {error}");
    }

    #[semio_framework_async_macros::async_test]
    async fn document_text_round_trip_with_operation_applied() {
        let mut store = store::ArtifactStore::<Procedural3dSnapshot, Procedural3dMutation>::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", Procedural3dSnapshot::default(), None)).expect("valid artifact store fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Procedural3dMutation::CreateWidget(CreateWidget { index: 3, widget: Widget::InputNote { id: "note-9".into(), text: String::new() } })], description: None }).expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
